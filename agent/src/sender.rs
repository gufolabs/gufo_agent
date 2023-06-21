// --------------------------------------------------------------------
// Gufo Agent: Openmetrics sender implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{MetricsData, MetricsDb, SenderConfig};
use common::{AgentError, Labels};
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddrV4;
use tokio::sync::mpsc;
use warp::{Filter, Reply};

const CONTENT_TYPE: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";

pub(crate) enum SenderCommand {
    Data(MetricsData),
    SetAgentLabels(Labels),
    Dump,
    Shutdown,
}

pub(crate) struct SenderHttp {
    listen: SocketAddrV4,
    tls_redirect: bool,
}

pub(crate) struct SenderHttps {
    listen: SocketAddrV4,
    cert_path: String,
    key_path: String,
    client_auth_required_path: Option<String>,
}
pub(crate) struct Sender {
    rx: mpsc::Receiver<SenderCommand>,
    tx: mpsc::Sender<SenderCommand>,
    db: MetricsDb,
    dump_metrics: bool,
    http: Option<SenderHttp>,
    https: Option<SenderHttps>,
    path: String,
}

const SENDER_CHANNEL_BUFFER: usize = 10_000;

impl TryFrom<&SenderConfig> for Sender {
    type Error = AgentError;

    fn try_from(value: &SenderConfig) -> Result<Self, Self::Error> {
        if value.r#type != *"openmetrics" {
            return Err(AgentError::ConfigurationError(
                "`sender.type` must be `openmetrics`".into(),
            ));
        }
        if value.mode != *"pull" {
            return Err(AgentError::ConfigurationError(
                "`sender.mode` must be `pull`".into(),
            ));
        }
        // Check HTTP settings
        let http = match &value.listen {
            Some(addr) => Some(SenderHttp {
                listen: addr.parse().map_err(|_| {
                    AgentError::ConfigurationError("Invalid `sender.listen`".to_string())
                })?,
                tls_redirect: value.tls_redirect,
            }),
            None => {
                if value.tls_redirect {
                    return Err(AgentError::ConfigurationError(
                        "`sender.tls_redirect` must not be set when HTTP endpoint is disabled"
                            .into(),
                    ));
                }
                None
            }
        };
        // Check HTTPS settings
        let https = match &value.listen_tls {
            Some(addr) => {
                let key_path = match &value.key_path {
                    Some(path) => path.clone(),
                    None => {
                        return Err(AgentError::ConfigurationError(
                            "`sender.key_path` must be set for HTTPS endpoint".to_string(),
                        ))
                    }
                };
                // Check key_path is readable
                fs::read_to_string(&key_path).map_err(|e| {
                    AgentError::ConfigurationError(format!(
                        "{} file is not readable: {}",
                        key_path, e
                    ))
                })?;
                let cert_path = match &value.cert_path {
                    Some(path) => path.clone(),
                    None => {
                        return Err(AgentError::ConfigurationError(
                            "`sender.cert_path` must be set for HTTPS endpoint".to_string(),
                        ))
                    }
                };
                // Check cert_path is readable
                fs::read_to_string(&cert_path).map_err(|e| {
                    AgentError::ConfigurationError(format!(
                        "{} file is not readable: {}",
                        cert_path, e
                    ))
                })?;
                let client_auth_required_path = match &value.client_auth_requred_path {
                    Some(path) => {
                        // Check path
                        fs::read_to_string(&path).map_err(|e| {
                            AgentError::ConfigurationError(format!(
                                "{} file is not readable: {}",
                                path, e
                            ))
                        })?;
                        Some(path.clone())
                    }
                    None => None,
                };
                Some(SenderHttps {
                    listen: addr.parse().map_err(|_| {
                        AgentError::ConfigurationError("Invalid `sender.listen_tls`".to_string())
                    })?,
                    key_path,
                    cert_path,
                    client_auth_required_path,
                })
            }
            None => {
                if value.tls_redirect {
                    return Err(AgentError::ConfigurationError(
                        "`sender.tls_redirect` must not be set when HTTPS endpoint is disabled"
                            .into(),
                    ));
                }
                None
            }
        };
        //
        let (tx, rx) = mpsc::channel::<SenderCommand>(SENDER_CHANNEL_BUFFER);
        Ok(Self {
            rx,
            tx,
            db: MetricsDb::default(),
            path: value.path.to_owned(),
            dump_metrics: false,
            http,
            https,
        })
    }
}

impl Sender {
    // Set dump_metrics status
    pub fn set_dump_metrics(&mut self, status: bool) -> &mut Self {
        self.dump_metrics = status;
        self
    }
    // Get cloned tx channel
    pub fn get_tx(&self) -> mpsc::Sender<SenderCommand> {
        self.tx.clone()
    }
    // Run sender message processing
    pub async fn run(&mut self) {
        log::info!("Running sender");
        self.run_endpoints();
        while let Some(msg) = self.rx.recv().await {
            match msg {
                SenderCommand::Data(data) => {
                    self.db.apply_data(&data).await;
                    if self.dump_metrics {
                        if let Ok(data) = self.db.to_openmetrics_string().await {
                            println!("{}", data)
                        }
                    }
                }
                SenderCommand::SetAgentLabels(labels) => {
                    log::debug!("Set labels to: {:?}", labels);
                    self.db.set_labels(labels).await;
                }
                SenderCommand::Dump => {
                    if let Ok(data) = self.db.to_openmetrics_string().await {
                        println!("{}", data)
                    }
                }
                SenderCommand::Shutdown => break,
            }
        }
        log::info!("Shutting down");
    }
    //
    fn run_endpoints(&self) {
        self.run_http_endpoint();
        self.run_https_endpoint();
    }
    //
    fn run_http_endpoint(&self) {
        if let Some(http) = &self.http {
            let listen = http.listen;
            let path = String::from(&self.path[1..]);
            if http.tls_redirect {
                log::info!(
                    "Starting TLS redirect HTTP endpoint at http://{}/{}",
                    listen,
                    path
                );
                let tls_port = self.https.as_ref().map(|x| x.listen.port()).unwrap_or(443);
                let endpoint = warp::path(path)
                    .and(warp::get())
                    .map(move || tls_port)
                    .and(warp::header::<String>("host"))
                    .and(warp::path::full())
                    .and_then(Self::tls_redirect_endpoint);
                tokio::spawn(async move {
                    warp::serve(endpoint).run(listen).await;
                });
            } else {
                log::info!("Starting HTTP endpoint at http://{}/{}", listen, path);
                let endpoint = warp::path(path)
                    .and(warp::get())
                    .and(Self::with_db(self.db.clone()))
                    .and_then(Self::metrics_endpoint);
                tokio::spawn(async move {
                    warp::serve(endpoint).run(listen).await;
                });
            };
        }
    }

    fn run_https_endpoint(&self) {
        if let Some(https) = &self.https {
            log::info!("Starting HTTPS endpoint at {}", https.listen);
            let listen = https.listen;
            let path = String::from(&self.path[1..]);
            let endpoint = warp::path(path)
                .and(warp::get())
                .and(Self::with_db(self.db.clone()))
                .and_then(Self::metrics_endpoint);
            let cert_path = https.cert_path.clone();
            let key_path = https.key_path.clone();
            let client_auth_requred_path = https.client_auth_required_path.clone();
            tokio::spawn(async move {
                match client_auth_requred_path {
                    Some(client_auth) => {
                        warp::serve(endpoint)
                            .tls()
                            .cert_path(cert_path)
                            .key_path(key_path)
                            .client_auth_required_path(client_auth)
                            .run(listen)
                            .await
                    }
                    None => {
                        warp::serve(endpoint)
                            .tls()
                            .cert_path(cert_path)
                            .key_path(key_path)
                            .run(listen)
                            .await
                    }
                }
            });
        }
    }

    fn with_db(db: MetricsDb) -> impl Filter<Extract = (MetricsDb,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
    async fn tls_redirect_endpoint(
        tls_port: u16,
        host: String,
        path: warp::path::FullPath,
    ) -> Result<impl warp::Reply, Infallible> {
        // @todo: Get TLS port
        let tls_host = match host.find(':') {
            Some(idx) => &host[..idx],
            None => &host,
        };
        let redirect_url = format!("https://{}:{}{}", tls_host, tls_port, path.as_str());
        Ok(warp::redirect::found(
            redirect_url.parse::<warp::http::Uri>().unwrap(),
        ))
    }

    async fn metrics_endpoint(db: MetricsDb) -> Result<impl warp::Reply, Infallible> {
        match db.to_openmetrics_string().await {
            Ok(data) => {
                Ok(warp::reply::with_header(data, "Content-Type", CONTENT_TYPE).into_response())
            }
            Err(e) => {
                log::error!("Error formatting data: {}", e);
                Ok(
                    warp::reply::with_status("", warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .into_response(),
                )
            }
        }
    }
}
