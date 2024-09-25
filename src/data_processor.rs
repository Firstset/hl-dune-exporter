use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, DateTime, Utc, NaiveDateTime};
use std::fs::File;
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

/// Parses a single trade from raw JSON data.
///
/// # Arguments
///
/// * `raw_trade` - A JSON Value containing the raw trade data.
/// * `side_info` - A slice of JSON Values containing side information.
/// * `time` - The parsed DateTime<Utc> for the trade.
///
/// # Returns
///
/// A Result containing the parsed Trade struct if successful, or an error if parsing fails.
fn format_single_trade(raw_trade: &serde_json::Value, side_info: &[serde_json::Value], time: DateTime<Utc>) -> Result<Trade> {
    Ok(Trade {
        coin: raw_trade["coin"].as_str().context("Missing 'coin' field")?.to_string(),
        side: raw_trade["side"].as_str().context("Missing 'side' field")?.to_string(),
        time,
        px: raw_trade["px"].as_str().context("Missing 'px' field")?.parse().context("Failed to parse 'px'")?,
        sz: raw_trade["sz"].as_str().context("Missing 'sz' field")?.parse().context("Failed to parse 'sz'")?,
        hash: raw_trade["hash"].as_str().context("Missing 'hash' field")?.to_string(),
        trade_dir_override: raw_trade["trade_dir_override"].as_str().context("Missing 'trade_dir_override' field")?.to_string(),
        user_a: side_info[0]["user"].as_str().context("Missing 'user' in side_info[0]")?.to_string(),
        start_pos_a: side_info[0]["start_pos"].as_str().context("Missing 'start_pos' in side_info[0]")?.parse().context("Failed to parse 'start_pos_a'")?,
        oid_a: side_info[0]["oid"].as_u64().context("Missing or invalid 'oid' in side_info[0]")?,
        twap_id_a: side_info[0]["twap_id"].as_str().map(String::from),
        cloid_a: side_info[0]["cloid"].as_str().map(String::from),
        user_b: side_info[1]["user"].as_str().context("Missing 'user' in side_info[1]")?.to_string(),
        start_pos_b: side_info[1]["start_pos"].as_str().context("Missing 'start_pos' in side_info[1]")?.parse().context("Failed to parse 'start_pos_b'")?,
        oid_b: side_info[1]["oid"].as_u64().context("Missing or invalid 'oid' in side_info[1]")?,
        twap_id_b: side_info[1]["twap_id"].as_str().map(String::from),
        cloid_b: side_info[1]["cloid"].as_str().map(String::from),
    })
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
pub fn process_data(data_dir: &str, batch_day: NaiveDate) -> Result<Vec<Trade>> {
    let mut trades = Vec::new();

    info!("Processing data for {} on dir {}", batch_day, data_dir);

    let date_folder = batch_day.format("%Y%m%d").to_string();
    let date_path = Path::new(data_dir).join(&date_folder);

    if !date_path.is_dir() {
        info!("No data found for {}", batch_day);
        return Ok(trades);
    }

    // Iterate through the hour files
    for hour in 0..24 {
        let hour_file = date_path.join(hour.to_string());
        if !hour_file.is_file() {
            continue;
        }

        info!("Reading hour file: {}", hour_file.display());

        // Process the trade file
        process_trade_file(&hour_file, &mut trades, batch_day)?;
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
/// * `batch_day` - The date to filter trades by.
///
/// # Returns
///
/// A Result indicating success or an error if the file couldn't be read or parsed.
fn process_trade_file(file_path: &Path, trades: &mut Vec<Trade>, batch_day: NaiveDate) -> Result<()> {
    let file = File::open(file_path).context("Failed to open trade file")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let raw_trade: serde_json::Value = serde_json::from_str(&line).context("Failed to parse trade JSON")?;
        
        if let Some(side_info) = raw_trade["side_info"].as_array() {
            if side_info.len() == 2 {
                let time_str = raw_trade["time"].as_str()
                    .context(format!("Missing 'time' field on line {} in file {:?}", line, file_path))?;
                
                let time = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S%.3f")
                    .context(format!("Failed to parse time '{}' on line {} in file {:?}", time_str, line, file_path))?;
                let time = DateTime::<Utc>::from_naive_utc_and_offset(time, Utc);
                
                if time.date_naive() == batch_day {
                    let trade = format_single_trade(&raw_trade, side_info, time)
                        .context(format!("Failed to parse trade on line {} in file {:?}", line, file_path))?;
                    trades.push(trade);
                }
            }
        }
    }

    Ok(())
}