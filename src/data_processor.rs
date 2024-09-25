use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime, Timelike};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Represents a single trade from the Hyperliquid Node data.
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub coin: String,
    pub side: String,
    pub time: DateTime<Utc>,
    pub px: f64,
    pub sz: f64,
    pub hash: String,
    pub trade_dir_override: String,
    pub side_info: Vec<SideInfo>,
}

/// Represents additional information for each side of a trade.
#[derive(Debug, Deserialize, Serialize)]
pub struct SideInfo {
    pub user: String,
    pub start_pos: String,
    pub oid: u64,
    pub twap_id: Option<String>,
    pub cloid: Option<String>,
}

/// Processes trade data from the Hyperliquid Node data directory.
///
/// This function reads trade data files within the specified time range,
/// parses the JSON data into Trade structs, and returns a vector of trades.
///
/// # Arguments
///
/// * `data_dir` - The path to the Hyperliquid Node data directory.
/// * `start_time` - The start of the time range to process.
/// * `end_time` - The end of the time range to process.
///
/// # Returns
///
/// A Result containing a vector of Trade structs if successful, or an error if not.
pub fn process_data(data_dir: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<Vec<Trade>> {
    let mut trades = Vec::new();

    // Iterate through the date folders
    for date_entry in fs::read_dir(data_dir)? {
        let date_path = date_entry?.path();
        if !date_path.is_dir() {
            continue;
        }

        let date_str = date_path.file_name().unwrap().to_str().unwrap();
        let date = NaiveDateTime::parse_from_str(&format!("{}T00:00:00", date_str), "%Y%m%dT%H:%M:%S")?;

        // Iterate through the hour folders
        for hour_entry in fs::read_dir(date_path)? {
            let hour_path = hour_entry?.path();
            if !hour_path.is_dir() {
                continue;
            }

            let hour_str = hour_path.file_name().unwrap().to_str().unwrap();
            let hour: u32 = hour_str.parse()?;
            let folder_datetime = date.with_hour(hour).unwrap();
            let folder_datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(folder_datetime, Utc);

            // Skip if the folder is outside the time range
            if folder_datetime_utc < start_time || folder_datetime_utc > end_time {
                continue;
            }

            // Process the trade file in this hour folder
            let file_path = hour_path.join("trades");
            if file_path.exists() {
                process_trade_file(&file_path, &mut trades, start_time, end_time)?;
            }
        }
    }

    Ok(trades)
}

/// Processes a single trade file and adds the trades to the trades vector.
///
/// This function reads a trade file line by line, parses each line into a Trade struct,
/// and adds it to the trades vector if it falls within the specified time range.
///
/// # Arguments
///
/// * `file_path` - The path to the trade file.
/// * `trades` - A mutable reference to the vector of trades to add to.
/// * `start_time` - The start of the time range to process.
/// * `end_time` - The end of the time range to process.
///
/// # Returns
///
/// A Result indicating success or an error if the file couldn't be read or parsed.
fn process_trade_file(file_path: &Path, trades: &mut Vec<Trade>, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<()> {
    let file = File::open(file_path).context("Failed to open trade file")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trade: Trade = serde_json::from_str(&line).context("Failed to parse trade JSON")?;

        if trade.time >= start_time && trade.time <= end_time {
            trades.push(trade);
        }
    }

    Ok(())
}