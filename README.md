# Hyperliquid Node Dune Exporter

This is a simple script to export [Hyperliquid Node](https://github.com/hyperliquid-dex/node/) data to a Dune table.

See [spec.md](spec.md) for more details.

## Configuration

Copy `config.toml.example` to `config.toml` and set the correct values.

## Usage

The simplest way to run it is:

```
cargo run --release
```

## Dependencies

Besides the Hyperliquid Node requirement, Rust and a few libraries are necessary to build the binary:

```
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Deployment

Recommended to run as a cron job with the `run_hyperliquid_export.sh` convenience script, which generates logs in the `logs` subdirectory.

You can try it out like this:

```
# Build the binary
cargo build --release

# Run the bash script
./run_hyperliquid_export.sh &

# Monitor the logs
tail -f logs/*
```

Then set it up as a cron job:

```
crontab -e
```

And add a line like:

```
0 0 * * * /path/to/hl-dune-exporter/run_hyperliquid_export.sh
```
