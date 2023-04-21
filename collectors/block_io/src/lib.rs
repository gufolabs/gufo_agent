// --------------------------------------------------------------------
// Gufo Agent: block_io collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, gauge, AgentError, Collectable, Measure};
use serde::Deserialize;
use systemstat::{Platform, System};

// Collector config
#[derive(Deserialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
counter!(read_ios, "Read operations", dev);
counter!(read_merges, "Read merges", dev);
counter!(read_sectors, "Total sectors read", dev);
counter!(read_ticks, "???", dev);
counter!(write_ios, "Write operatios", dev);
counter!(write_merges, "Write merges", dev);
counter!(write_sectors, "Total sectors written", dev);
counter!(write_ticks, "???", dev);
gauge!(in_flight, "???", dev);
gauge!(io_ticks, "???", dev);
gauge!(time_in_queue, "???", dev);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(_: Config) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    // !!! Set proper name
    const NAME: &'static str = "block_io";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let stats = System::new().block_device_statistics()?;
        if stats.is_empty() {
            return Ok(Vec::new());
        }
        let mut r = Vec::with_capacity(stats.len() * 11);
        for s in stats.values() {
            r.push(read_ios(s.read_ios as u64, &s.name));
            r.push(read_merges(s.read_merges as u64, &s.name));
            r.push(read_sectors(s.read_sectors as u64, &s.name));
            r.push(read_ticks(s.read_ticks as u64, &s.name));
            r.push(write_ios(s.write_ios as u64, &s.name));
            r.push(write_merges(s.write_merges as u64, &s.name));
            r.push(write_sectors(s.write_sectors as u64, &s.name));
            r.push(write_ticks(s.write_ticks as u64, &s.name));
            r.push(in_flight(s.in_flight as i64, &s.name));
            r.push(io_ticks(s.io_ticks as i64, &s.name));
            r.push(time_in_queue(s.time_in_queue as i64, &s.name));
        }
        // Push result
        Ok(r)
    }
}
