// --------------------------------------------------------------------
// Gufo Agent: Openmetrics sender implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{MetricsData, MetricsDb, SenderConfig};
use common::{AgentError, Labels};
use std::convert::Infallible;
use std::net::SocketAddrV4;
use tokio::sync::mpsc;
use warp::{Filter, Reply};

const CONTENT_TYPE: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";

pub(crate) enum SenderCommand {
    Data(MetricsData),
    SetAgentLabels(Labels),
}

pub(crate) struct Sender {
    rx: mpsc::Receiver<SenderCommand>,
    tx: mpsc::Sender<SenderCommand>,
    db: MetricsDb,
    dump_metrics: bool,
    listen: SocketAddrV4,
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
        let sock_addr: SocketAddrV4 = value
            .listen
            .parse()
            .map_err(|_| AgentError::ConfigurationError("Invalid `listen`".to_string()))?;
        let (tx, rx) = mpsc::channel::<SenderCommand>(SENDER_CHANNEL_BUFFER);
        Ok(Self {
            rx,
            tx,
            db: MetricsDb::default(),
            dump_metrics: false,
            listen: sock_addr,
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
        self.run_endpoint();
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
                } //SenderCommand::Shutdown => break,
            }
        }
        log::info!("Shutting down");
    }
    //
    fn run_endpoint(&self) {
        log::info!("Starting openmetrics endpoint");
        let endpoint = warp::path("metrics")
            .and(warp::get())
            .and(Self::with_db(self.db.clone()))
            .and_then(Self::metrics_endpoint);
        let listen = self.listen;
        tokio::spawn(async move {
            warp::serve(endpoint).run(listen).await;
        });
    }

    fn with_db(db: MetricsDb) -> impl Filter<Extract = (MetricsDb,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
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
