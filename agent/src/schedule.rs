// --------------------------------------------------------------------
// Gufo Agent: Schedule
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{CollectorConfig, Collectors, MetricsData, SenderCommand};
use common::{AgentError, Labels, Measure};
use rand::Rng;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::time::Duration;

pub(crate) struct Schedule {
    id: String,
    interval: u64,
    labels: Labels,
    collector: Collectors,
    sender_tx: Option<mpsc::Sender<SenderCommand>>,
}

impl TryFrom<CollectorConfig> for Schedule {
    type Error = AgentError;
    fn try_from(value: CollectorConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.clone(),
            interval: value
                .interval
                .ok_or(AgentError::ParseError("invalid interval".to_string()))?,
            labels: value.labels.clone().into(),
            collector: Collectors::try_from(value)?,
            sender_tx: None,
        })
    }
}

impl Schedule {
    pub fn set_sender(&mut self, tx: Option<mpsc::Sender<SenderCommand>>) {
        self.sender_tx = tx;
    }
    pub async fn run(&mut self) {
        log::info!("[{}] Starting collector", self.id);
        // Initial time offset
        let delay: u64 = if self.collector.is_random_offset() {
            // Sleep random time to prevent spikes of load
            let max_delay = self.interval * 1_000_000_000;
            rand::thread_rng().gen_range(0..max_delay)
        } else {
            0
        };
        log::debug!(
            "[{}] Starting delay {:?} of {:?}",
            self.id,
            Duration::from_nanos(delay),
            Duration::from_secs(self.interval)
        );
        tokio::time::sleep(Duration::from_nanos(delay)).await;
        //
        let collector_name = self.collector.get_name();
        loop {
            log::info!("[{}] Collecting", self.id);
            // Get Unix timestamp
            let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(x) => x.as_secs(),
                Err(e) => {
                    log::error!("Failed to get timestamp: {}", e);
                    0
                }
            };
            // Run collector
            let t0 = Instant::now();
            match self.collector.collect().await {
                Ok(measures) => {
                    if let Err(e) = self.send(collector_name, measures, ts).await {
                        log::error!("[{}] Failed to send: {}", self.id, e);
                    } else {
                        log::info!("[{}] Done", self.id);
                    }
                }
                Err(e) => log::error!("[{}] Crashed with: {}", self.id, e),
            };
            // Time elapsed
            let dt = t0.elapsed();
            // Adjust sleep time
            let to_sleep = if dt.as_secs() >= self.interval {
                // Elapsed time is greater, than interval
                log::info!("[{}] Overrun configured interval", self.id);
                Duration::from_secs(self.interval)
            } else {
                // Deduct elapsed time
                Duration::from_nanos(self.interval * 1_000_000_000 - dt.as_nanos() as u64)
            };
            tokio::time::sleep(to_sleep).await;
        }
    }
    async fn send(
        &self,
        collector_name: &'static str,
        measures: Vec<Measure>,
        ts: u64,
    ) -> Result<(), AgentError> {
        if let Some(tx) = &self.sender_tx {
            tx.send(SenderCommand::Data(MetricsData {
                collector: collector_name,
                labels: self.labels.clone(),
                measures,
                ts,
            }))
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        }
        Ok(())
    }
}
