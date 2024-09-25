use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

use crate::config::Config;
use crate::data_processor::Trade;
use crate::schemas;

pub struct DuneApi {
    client: Client,
    config: Config,
}

const DUNE_API_URL: &str = "https://api.dune.com/api/v1";

impl DuneApi {
    /// Creates a new instance of the DuneApi.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration containing Dune API settings.
    ///
    /// # Returns
    ///
    /// A new `DuneApi` instance.
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Creates a new table in the Dune database.
    ///
    /// This function sends a POST request to the Dune API to create a new table
    /// with the schema defined in the `schemas` module.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the operation failed.
    pub async fn create_table(&self) -> Result<()> {
        // Prepare the request payload
        let payload = json!({
            "namespace": self.config.dune_user_namespace,
            "table_name": self.config.dune_table_name,
            "description": "Hyperliquid Testnet trade data",
            "is_private": false,
            "schema": schemas::get_trade_schema()
        });

        // Send the request to create the table
        let response = self.client
            .post(format!("{}/table/create", DUNE_API_URL))
            .header("X-DUNE-API-KEY", &self.config.dune_api_key)
            .json(&payload)
            .send()
            .await
            .context("Failed to send create table request")?;

        // Check if the request was successful
        if response.status().is_success() {
            println!("Table created successfully");
            Ok(())
        } else {
            let error_message = response.text().await.context("Failed to read error response")?;
            anyhow::bail!("Failed to create table: {}", error_message)
        }
    }

    /// Inserts trade data into the Dune database.
    ///
    /// This function converts the trade data to NDJSON format and sends a POST request
    /// to the Dune API to insert the data into the previously created table.
    ///
    /// # Arguments
    ///
    /// * `trades` - A vector of `Trade` structs containing the data to be inserted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the operation failed.
    pub async fn insert_data(&self, trades: Vec<Trade>) -> Result<()> {
        // Convert trades to NDJSON format
        let ndjson = trades.iter()
            .map(|trade| serde_json::to_string(trade))
            .collect::<Result<Vec<String>, _>>()?
            .join("\n");

        // Send the request to insert data
        let response = self.client
            .post(format!("{}/table/{}/{}/insert", DUNE_API_URL, self.config.dune_user_namespace, self.config.dune_table_name))
            .header("X-DUNE-API-KEY", &self.config.dune_api_key)
            .header("Content-Type", "application/x-ndjson")
            .body(ndjson)
            .send()
            .await
            .context("Failed to send insert data request")?;

        // Check if the request was successful
        if response.status().is_success() {
            println!("Data inserted successfully");
            Ok(())
        } else {
            let error_message = response.text().await.context("Failed to read error response")?;
            anyhow::bail!("Failed to insert data: {}", error_message)
        }
    }
}