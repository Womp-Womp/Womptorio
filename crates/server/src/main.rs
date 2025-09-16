use anyhow::Result;
use clap::Parser;
use std::fs;
use womporio_core::{SimConfig, SimWorld};

#[derive(Parser, Debug)]
#[command(
    name = "womporio-server",
    about = "Headless deterministic simulation driver"
)]
struct Args {
    /// Number of ticks to advance before emitting a snapshot.
    #[arg(long, default_value_t = 256)]
    ticks: u64,

    /// Seed for the deterministic RNG.
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Optional path to persist a JSON snapshot.
    #[arg(long)]
    snapshot: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = SimConfig {
        seed: args.seed,
        fixed_time_step: 1,
    };

    let mut sim = SimWorld::new(config);
    sim.run_for_ticks(args.ticks);

    let snapshot = sim.snapshot();
    let json = serde_json::to_string_pretty(&snapshot)?;
    if let Some(path) = args.snapshot {
        fs::write(path, json)?;
    } else {
        println!("{}", json);
    }

    Ok(())
}
