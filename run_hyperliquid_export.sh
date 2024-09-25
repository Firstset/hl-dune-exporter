#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Set the path to your Rust binary
RUST_BIN="$SCRIPT_DIR/target/release/hl_dune_exporter"

# Set the log file path
LOG_DIR="$SCRIPT_DIR/logs"
LOG_FILE="$LOG_DIR/hyperliquid_export_$(date +%Y-%m-%d).log"

# Create log directory if it doesn't exist
mkdir -p "$LOG_DIR"

# Run the Rust program and append output to the log file
RUST_LOG=info $RUST_BIN --config "$SCRIPT_DIR/config.toml" >> "$LOG_FILE" 2>&1

# Optionally, you can add a timestamp to each run
echo "Job completed at $(date)" >> "$LOG_FILE"