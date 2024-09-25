# Hyperliquid Node Dune Exporter

This is a simple script to export Hyperliquid Node data to a Dune table.

See [spec.md](spec.md) for more details.

## Dependencies

HL Node requires Ubuntu so need to:

```
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

```
cargo run --release
```