use anyhow::Result;
use serde::Deserialize;
use std::fs;

/// Represents the configuration for the application.
///
/// This struct holds all the necessary configuration parameters
/// loaded from the config file.
#[derive(Deserialize, Clone)]
pub struct Config {
    /// The API key for authenticating with the Dune API.
    pub dune_api_key: String,
    /// The directory path where Hyperliquid Node data is stored.
    pub hyperliquid_data_dir: String,
    /// The user namespace in Dune where the table will be created.
    pub dune_user_namespace: String,
    /// The name of the table to be created in Dune.
    pub dune_table_name: String,
    /// The look back period in days.
    pub look_back_period: i64,
}

/// Loads the configuration from a TOML file.
///
/// This function reads the specified TOML file, parses its contents,
/// and returns a Config struct.
///
/// # Arguments
///
/// * `path` - The path to the TOML configuration file.
///
/// # Returns
///
/// A Result containing the Config struct if successful, or an error if not.
pub fn load_config(path: &str) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}