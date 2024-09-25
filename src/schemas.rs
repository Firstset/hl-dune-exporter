use serde_json::json;

/// Returns the JSON schema for the trade data table in Dune.
///
/// This function defines the structure of the trade data that will be
/// stored in the Dune database. It includes fields for coin, side, time,
/// price, size, hash, trade direction override, and side info.
///
/// # Returns
///
/// A `serde_json::Value` representing the schema as a JSON array.
pub fn get_trade_schema() -> serde_json::Value {
    json!([
        {"name": "coin", "type": "string"},
        {"name": "side", "type": "string"},
        {"name": "time", "type": "timestamp"},
        {"name": "px", "type": "double"},
        {"name": "sz", "type": "double"},
        {"name": "hash", "type": "string"},
        {"name": "trade_dir_override", "type": "string"},
        {"name": "side_info", "type": "json"}
    ])
}