use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use chrono::{Utc, Duration};

mod config;
mod data_processor;
mod dune_api;
mod schemas;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let config = config::load_config(&cli.config)?;

    info!("Starting Hyperliquid to Dune export");

    // Create DuneApi instance
    let dune_api = dune_api::DuneApi::new(config.clone());

    // Create the table in Dune (if it doesn't exist)
    dune_api.create_table().await.context("Failed to create table in Dune")?;

    // Calculate time range for the last 24 hours
    let end_time = Utc::now();
    let start_time = end_time - Duration::hours(24);

    // Process data for the last 24 hours
    let trades = data_processor::process_data(&config.hyperliquid_data_dir, start_time, end_time)
        .context("Failed to process trade data")?;

    info!("Processed {} trades", trades.len());

    // Send data to Dune API
    dune_api.insert_data(trades).await.context("Failed to insert data into Dune")?;

    info!("Export completed successfully");
    Ok(())
}