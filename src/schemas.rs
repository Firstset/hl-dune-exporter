use serde_json::json;

/// Returns the JSON schema for the trade data table in Dune.
///
/// This function defines the flattened structure of the trade data that will be
/// stored in the Dune database. It includes fields for coin, side, time,
/// price, size, hash, trade direction override, and flattened side info for both sides.
///
/// # Returns
///
/// A `serde_json::Value` representing the schema as a JSON array.
pub fn get_trade_schema() -> serde_json::Value {
    json!([
        {"name": "coin", "type": "varchar"},
        {"name": "side", "type": "varchar"},
        {"name": "time", "type": "timestamp"},
        {"name": "px", "type": "double"},
        {"name": "sz", "type": "double"},
        {"name": "hash", "type": "varchar"},
        {"name": "trade_dir_override", "type": "varchar"},
        {"name": "user_a", "type": "varchar"},
        {"name": "start_pos_a", "type": "double"},
        {"name": "oid_a", "type": "uint256"},
        {"name": "twap_id_a", "type": "varchar", "nullable": true},
        {"name": "cloid_a", "type": "varchar", "nullable": true},
        {"name": "user_b", "type": "varchar"},
        {"name": "start_pos_b", "type": "double"},
        {"name": "oid_b", "type": "uint256"},
        {"name": "twap_id_b", "type": "varchar", "nullable": true},
        {"name": "cloid_b", "type": "varchar", "nullable": true},
    ])
}