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
    pub user_a: String,
    pub start_pos_a: f64,
    pub oid_a: u64,
    pub twap_id_a: Option<String>,
    pub cloid_a: Option<String>,
    pub user_b: String,
    pub start_pos_b: f64,
    pub oid_b: u64,
    pub twap_id_b: Option<String>,
    pub cloid_b: Option<String>,
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
        let raw_trade: serde_json::Value = serde_json::from_str(&line).context("Failed to parse trade JSON")?;
        
        if let Some(side_info) = raw_trade["side_info"].as_array() {
            if side_info.len() == 2 {
                let time = DateTime::parse_from_rfc3339(raw_trade["time"].as_str().unwrap())?.with_timezone(&Utc);
                
                if time >= start_time && time <= end_time {
                    let trade = Trade {
                        coin: raw_trade["coin"].as_str().unwrap().to_string(),
                        side: raw_trade["side"].as_str().unwrap().to_string(),
                        time,
                        px: raw_trade["px"].as_str().unwrap().parse()?,
                        sz: raw_trade["sz"].as_str().unwrap().parse()?,
                        hash: raw_trade["hash"].as_str().unwrap().to_string(),
                        trade_dir_override: raw_trade["trade_dir_override"].as_str().unwrap().to_string(),
                        user_a: side_info[0]["user"].as_str().unwrap().to_string(),
                        start_pos_a: side_info[0]["start_pos"].as_str().unwrap().parse()?,
                        oid_a: side_info[0]["oid"].as_u64().unwrap(),
                        twap_id_a: side_info[0]["twap_id"].as_str().map(String::from),
                        cloid_a: side_info[0]["cloid"].as_str().map(String::from),
                        user_b: side_info[1]["user"].as_str().unwrap().to_string(),
                        start_pos_b: side_info[1]["start_pos"].as_str().unwrap().parse()?,
                        oid_b: side_info[1]["oid"].as_u64().unwrap(),
                        twap_id_b: side_info[1]["twap_id"].as_str().map(String::from),
                        cloid_b: side_info[1]["cloid"].as_str().map(String::from),
                    };
                    trades.push(trade);
                }
            }
        }
    }

    Ok(())
}