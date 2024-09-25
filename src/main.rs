use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use clap::Parser;
use log::info;

mod config;
mod data_processor;
mod dune_api;
mod schemas;
mod batching;

use dune_api::DuneApi;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    info!("Starting Hyperliquid to Dune export");

    // Load configuration
    let config = config::load_config(&cli.config)
        .context("Failed to load configuration")?;

    // Initialize Dune API client
    let dune_api = DuneApi::new(config.clone());

    // Clear the table or create if it doesn't exist
    info!("Clearing or creating the table...");
    dune_api.clear_or_create_table().await
        .context("Failed to clear or create the table")?;

    // Calculate date range
    let end_time = Utc::now();
    let start_time = end_time - Duration::days(config.look_back_period_days);

    info!("Processing data from {} to {}", start_time, end_time);

    // Process and insert data in daily batches
    batching::process_and_insert_data(&config, &dune_api, start_time, end_time).await
        .context("Failed to process and insert data")?;

    info!("Data export completed successfully");
    Ok(())
}