// --------------------------------------------------------------------
// Gufo Agent: twamp_reflector collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use common::{counter, AgentError, Collectable, Measure};
use connection::Connection;
use rand::Rng;
use serde::Deserialize;
use std::convert::TryFrom;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, RwLock},
    task::JoinHandle,
    time::timeout,
};
use twamp::{
    AcceptSession, RequestTwSession, ServerGreeting, ServerStart, SetupResponse, StartAck,
    StartSessions, StopSessions, TestRequest, TestResponse, DEFAULT_COUNT, MODE_UNAUTHENTICATED,
};
use udp::UdpConnection;

// Collector config
#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_all")]
    pub listen: String,
    #[serde(default = "default_862")]
    pub port: u16,
}

// Collector structure
pub struct Collector {
    listen: String,
    port: u16,
    listen_handle: Option<JoinHandle<()>>,
    stats: Arc<RwLock<CollectorStats>>,
}

#[derive(Default)]
struct CollectorStats {
    session_attempts: u64,
    session_started: u64,
    reflected_pkt: u64,
    reflected_octets: u64,
}

// Generated metrics
counter!(
    twamp_session_attempts,
    "Total amount of the attempted sessions"
);
counter!(
    twamp_session_started,
    "Total amount of the started sessions"
);
counter!(twamp_reflected_pkt, "Total amount of the reflected packets");
counter!(
    twamp_reflected_octets,
    "Total amount of the reflected octets"
);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            listen: value.listen,
            port: value.port,
            listen_handle: None,
            stats: Arc::new(RwLock::new(CollectorStats::default())),
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "twamp_reflector";
    const RANDOM_OFFSET: bool = false; // Listen immediately
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        Ok(match self.listen_handle {
            Some(_) => {
                // Collect metrics
                let stats = self.stats.read().await;
                stats.get_stats()
            }
            None => {
                // Start metrics reader
                // Start listener
                self.listen_handle = Some(self.listen().await?);
                vec![] // No metrics yet
            }
        })
    }
}

impl Collector {
    async fn listen(&self) -> Result<JoinHandle<()>, AgentError> {
        // Metrics channel
        let (tx, mut rx) = mpsc::channel::<SessionStats>(100);
        // Start metrics feeder
        let db = Arc::clone(&self.stats);
        tokio::spawn(async move {
            while let Some(s) = rx.recv().await {
                let mut stats = db.write().await;
                stats.session_attempts += 1;
                if s.reflected_pkt > 0 {
                    stats.session_started += 1;
                    stats.reflected_pkt += s.reflected_pkt;
                    stats.reflected_octets += s.reflected_octets;
                }
            }
        });
        // Start listener
        log::info!("Listening {}:{}", self.listen, self.port);
        let listener = match TcpListener::bind(format!("{}:{}", self.listen, self.port)).await {
            Ok(x) => x,
            Err(e) => return Err(AgentError::InternalError(e.to_string())),
        };
        // Serve connections
        let h = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        let session_tx = tx.clone();
                        // Span client session
                        tokio::spawn(async move {
                            let mut session = ClientSession::new_from(stream, addr);
                            let stats = session.run().await;
                            if let Err(e) = session_tx.send(stats).await {
                                log::error!("Cannot send stats: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
        Ok(h)
    }
}

#[derive(Debug)]
struct ClientSession {
    connection: Connection,
    addr: SocketAddr,
    dscp: u8,
    reflector_port: Option<u16>,
    reflector_handle: Option<JoinHandle<SessionStats>>,
}

#[derive(Clone, Copy, Debug, Default)]
struct SessionStats {
    reflected_pkt: u64,
    reflected_octets: u64,
}

impl ClientSession {
    fn new_from(stream: TcpStream, addr: SocketAddr) -> ClientSession {
        ClientSession {
            connection: Connection::new(stream),
            addr,
            dscp: 0,
            reflector_port: None,
            reflector_handle: None,
        }
    }

    async fn run(&mut self) -> SessionStats {
        match self.process().await {
            Ok(stats) => stats,
            Err(e) => {
                log::error!("[{}] Failed to process session: {}", self.addr, e);
                SessionStats::default()
            }
        }
    }

    async fn process(&mut self) -> Result<SessionStats, AgentError> {
        log::info!("[{}] Connected", self.addr);
        // Control messages timeout, 3 seconds by default
        let ctl_timeout = Duration::from_nanos(3_000_000_000);
        self.send_server_greeting().await?;
        self.recv_setup_response(ctl_timeout).await?;
        self.send_server_start().await?;
        self.recv_request_tw_session(ctl_timeout).await?;
        self.start_reflector().await?;
        self.send_accept_session().await?;
        self.recv_start_sessions(ctl_timeout).await?;
        self.send_start_ack().await?;
        self.recv_stop_sessions().await?; // No direct timeout
        let stats = if let Some(h) = self.reflector_handle.take() {
            h.await
                .map_err(|e| AgentError::InternalError(e.to_string()))?
        } else {
            SessionStats::default()
        };
        log::info!("[{}] Session complete", self.addr);
        Ok(stats)
    }
    async fn send_server_greeting(&mut self) -> Result<(), AgentError> {
        log::debug!("[{}] Sending Server-Greeting", self.addr);
        // Send Server-Greeting
        let challenge = rand::thread_rng().gen::<[u8; 16]>();
        let salt = rand::thread_rng().gen::<[u8; 16]>();
        let sg = ServerGreeting {
            modes: MODE_UNAUTHENTICATED,
            challenge: Bytes::copy_from_slice(&challenge),
            salt: Bytes::copy_from_slice(&salt),
            count: DEFAULT_COUNT,
        };
        self.connection.write_frame(&sg).await?;
        Ok(())
    }
    async fn recv_setup_response(&mut self, t: Duration) -> Result<(), AgentError> {
        log::debug!("Waiting for Setup-Response");
        let sr: SetupResponse = timeout(t, self.connection.read_frame()).await??;
        log::debug!("Received Setup-Response");
        match sr.mode {
            MODE_UNAUTHENTICATED => self.auth_unathenticated().await,
            _ => {
                log::error!("Unsupported mode: {}", sr.mode);
                Err(AgentError::FrameError("unsupported mode".into()))
            }
        }
    }
    async fn auth_unathenticated(&mut self) -> Result<(), AgentError> {
        log::debug!("Starting unauthenticated session");
        Ok(())
    }

    async fn send_server_start(&mut self) -> Result<(), AgentError> {
        log::debug!("Sending Server-Start");
        let server_iv = rand::thread_rng().gen::<[u8; 16]>();
        let ss = ServerStart {
            accept: 0,
            server_iv: Bytes::copy_from_slice(&server_iv),
            start_time: Utc::now(),
        };
        self.connection.write_frame(&ss).await?;
        Ok(())
    }
    async fn recv_request_tw_session(&mut self, t: Duration) -> Result<(), AgentError> {
        log::debug!("Waiting for Request-TW-Session");
        let req: RequestTwSession = timeout(t, self.connection.read_frame()).await??;
        log::debug!(
            "Received Request-TW-Session. Client timestamp={:?}, Type-P={}",
            req.start_time,
            req.type_p
        );
        self.dscp = (req.type_p & 0xff) as u8;
        Ok(())
    }
    async fn send_accept_session(&mut self) -> Result<(), AgentError> {
        log::debug!("Sending Accept-Session");
        let msg = AcceptSession {
            accept: 0,
            port: self.get_reflector_port()?,
        };
        self.connection.write_frame(&msg).await?;
        Ok(())
    }
    async fn recv_start_sessions(&mut self, t: Duration) -> Result<(), AgentError> {
        log::debug!("Waiting for Start-Sessions");
        let _: StartSessions = timeout(t, self.connection.read_frame()).await??;
        log::debug!("Start-Sessions received");
        Ok(())
    }
    async fn send_start_ack(&mut self) -> Result<(), AgentError> {
        log::debug!("Sending Start-Ack");
        let msg = StartAck { accept: 0 };
        self.connection.write_frame(&msg).await?;
        Ok(())
    }
    async fn recv_stop_sessions(&mut self) -> Result<(), AgentError> {
        log::debug!("Waiting for Stop-Sessions");
        let _: StopSessions = self.connection.read_frame().await?;
        log::debug!("Received Stop-Sessions");
        Ok(())
    }
    fn get_reflector_port(&self) -> Result<u16, AgentError> {
        match self.reflector_port {
            Some(port) => Ok(port),
            None => Err(AgentError::NetworkError("socket not created".into())),
        }
    }
    async fn start_reflector(&mut self) -> Result<(), AgentError> {
        let (tx, rx) = oneshot::channel();
        self.reflector_handle = Some(tokio::spawn(async move {
            let mut stats = SessionStats::default();
            if let Err(e) = ClientSession::reflect(tx, &mut stats).await {
                log::error!(" Reflector error: {:?}", e);
            }
            stats
        }));
        match rx.await {
            Ok(x) => self.reflector_port = Some(x),
            Err(e) => {
                log::error!("Reflector error: {}", e);
                return Err(AgentError::NetworkError(e.to_string()));
            }
        }
        Ok(())
    }
    async fn reflect(
        port_channel: oneshot::Sender<u16>,
        stats: &mut SessionStats,
    ) -> Result<(), AgentError> {
        log::debug!("Creating reflector");
        // Timeout
        let recv_timeout = Duration::from_nanos(3_000_000_000);
        // Reflector socket
        let mut socket = UdpConnection::bind("0.0.0.0:0").await?;
        // Reflector TTL must be set to 255
        socket.set_ttl(255)?;
        // Send back allocated port to session
        if let Err(e) = port_channel.send(socket.local_port()?) {
            log::error!("Cannot send reflector port: {}", e);
            return Err(AgentError::NetworkError(
                "cannot send reflector port".into(),
            ));
        }
        //
        let mut seq = 0u32;
        loop {
            let (req, addr) = match timeout(recv_timeout, socket.recv_from::<TestRequest>()).await {
                Ok(Ok(r)) => r,
                // recv_from returns an error
                // We'd expected truncated frames on high load, so count it as a loss
                Ok(Err(_)) => continue,
                // Timed out, break the loop
                Err(_) => break,
            };
            // Build response
            let ts = Utc::now();
            let resp = TestResponse {
                seq,
                timestamp: ts,
                err_estimate: 0,
                recv_timestamp: ts,
                sender_seq: req.seq,
                sender_timestamp: req.timestamp,
                sender_err_estimate: req.err_estimate,
                sender_ttl: 255, // @todo: Get real TTL
                pad_to: req.pad_to,
            };
            //
            stats.reflected_octets +=
                socket.send_to(&resp, addr).await? as u64 + IP_OVERHEAD + UDP_OVERHEAD;
            stats.reflected_pkt += 1;
            seq += 1;
        }
        log::debug!("Stopping reflector");
        Ok(())
    }
}

impl CollectorStats {
    pub fn get_stats(&self) -> Vec<Measure> {
        vec![
            twamp_session_attempts(self.session_attempts),
            twamp_session_started(self.session_started),
            twamp_reflected_pkt(self.reflected_pkt),
            twamp_reflected_octets(self.reflected_octets),
        ]
    }
}

fn default_862() -> u16 {
    862
}

fn default_all() -> String {
    "0.0.0.0".into()
}

const IP_OVERHEAD: u64 = 20;
const UDP_OVERHEAD: u64 = 8;
