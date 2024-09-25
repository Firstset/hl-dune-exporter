use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use log::info;

use crate::config::Config;
use crate::data_processor::process_data;
use crate::dune_api::DuneApi;

pub async fn process_and_insert_data(
    config: &Config,
    dune_api: &DuneApi,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<()> {
    let mut batch_day = start_time.date_naive();
    let end_date = end_time.date_naive();

    while batch_day <= end_date { 
        info!("Processing data for {}", batch_day);

        // Process data for the day
        let trades = process_data(&config.hyperliquid_data_dir, batch_day)
            .context(format!("Failed to process trade data for {}", batch_day))?;
        // If no trades were processed for the day, skip to the next day
        if trades.is_empty() {
            info!("No trades found for {}. Skipping to next day.", batch_day);
            batch_day += Duration::days(1);
            continue;
        }

        // Insert data into Dune
        info!("Inserting {} trades for {} into Dune...", trades.len(), batch_day);
        dune_api.insert_data(trades).await
            .context(format!("Failed to insert data for {} into Dune", batch_day))?;

        batch_day += Duration::days(1);
    }

    Ok(())
}