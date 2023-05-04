// --------------------------------------------------------------------
// Gufo Agent: Agent CLI
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use agent::{config_from_discovery, Agent, Collectors};
use clap::Parser;
use common::ConfigDiscoveryOpts;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    pub quiet: bool,
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[arg(short = 'k', long, env = "GA_INSECURE")]
    pub insecure: bool,
    #[arg(short, long, env = "GA_CONFIG")]
    pub config: Option<String>,
    #[arg(long, env = "GA_HOSTNAME")]
    pub hostname: Option<String>,
    #[arg(long)]
    pub list_collectors: bool,
    #[arg(long, env = "GA_DUMP_METRICS")]
    pub dump_metrics: bool,
    #[arg(long)]
    pub config_discovery: bool,
    #[arg(long, env = "GA_CONFIG_DISCOVERY_OPTS")]
    pub config_discovery_opts: Option<String>,
}

const ERR_EX_OTHER: i32 = 1;

/// Agent entrypoint
fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();
    // --list-collectors
    if cli.list_collectors {
        for name in Collectors::to_vec().iter() {
            println!("{}", name);
        }
        return;
    }
    // --config-discovery
    if cli.config_discovery {
        let opts = match cli.config_discovery_opts {
            Some(x) => match ConfigDiscoveryOpts::try_from(x) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {:?}", e);
                    process::exit(ERR_EX_OTHER);
                }
            },
            None => ConfigDiscoveryOpts::default(),
        };
        match config_from_discovery(&opts) {
            Ok(r) => {
                println!("{}", r);
                return;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                process::exit(ERR_EX_OTHER);
            }
        }
    }
    // Setup logging
    env_logger::builder()
        .format_timestamp_millis()
        .filter_level(match cli.quiet {
            true => log::LevelFilter::Off,
            false => match cli.verbose {
                0 => log::LevelFilter::Error,
                1 => log::LevelFilter::Info,
                _ => log::LevelFilter::Debug,
            },
        })
        .init();
    // Setup agent
    let mut agent = Agent::builder()
        .set_cert_validation(!cli.insecure)
        .set_dump_metrics(cli.dump_metrics)
        .set_config(cli.config)
        .set_hostname(cli.hostname)
        .build();
    // Run agent and wait for complete
    if let Err(e) = agent.run() {
        println!("Error: {:?}", e);
        process::exit(ERR_EX_OTHER);
    }
}
