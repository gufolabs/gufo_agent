// --------------------------------------------------------------------
// Gufo Agent: block_io collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, gauge, AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
counter!(read_ios, "Number of read I/Os processed", dev);
counter!(
    read_merges,
    "Number of read I/Os merged with in-queue I/O",
    dev
);
counter!(read_sectors, "Number of sectors read", dev);
counter!(read_ticks, "Total wait time for read requests, ms", dev);
counter!(write_ios, "Number of write I/Os processed", dev);
counter!(
    write_merges,
    "Number of write I/Os merged with in-queue I/O",
    dev
);
counter!(write_sectors, "Number of sectors written", dev);
counter!(write_ticks, "Total wait time for write requests, ms", dev);
gauge!(
    in_flight,
    "Number of I/Os currently in flight, requests",
    dev
);
gauge!(
    io_ticks,
    "Total time this block device has been active, ms",
    dev
);
gauge!(time_in_queue, "Total wait time for all requests, ms", dev);

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
            r.push(in_flight(s.in_flight as u64, &s.name));
            r.push(io_ticks(s.io_ticks as u64, &s.name));
            r.push(time_in_queue(s.time_in_queue as u64, &s.name));
        }
        // Push result
        Ok(r)
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}
