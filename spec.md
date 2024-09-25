# Project Spec
## Overview

This project is a simple script to periodically export Hyperliquid Node data to Dune. It can be run as a cron job on the same server where the Hyperliquid Node is running.

## Requirements

- Build in Rust
- Run as a script (manually or as a cron job)
- Read trade data accessible through ~/hl/data/node_trades/hourly/{date}/{hour}
- Insert data in batch to Dune API
- The configuration parameters listed in the Parameters section below are supported 
- Configuration parameters are loaded from a config.toml file
- The script is assumed to be run daily every day at the same time, so it should load the last 24h of data but avoid data from the current hour as it is not complete and we want to remain stateless

## Parameters

- Dune API key
- Hyperliquid Node data directory
- Dune user namespace
- Dune table name

## Hyperliquid data format

Sample file:

```
head -n 10 hl/data/node_trades/hourly/20240919/10
{"coin":"ZK","side":"B","time":"2024-09-19T10:00:02.015","px":"0.12398","sz":"158.0","hash":"0x47c078ffd46e96d69c5d040fe44603012500707c9f0e8e9c3f432f78494532bc","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"1680951.0","oid":14849962333,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"-2074344.0","oid":14849962111,"twap_id":null,"cloid":null}]}
{"coin":"ZK","side":"A","time":"2024-09-19T10:00:02.015","px":"0.12398","sz":"158.0","hash":"0x47c078ffd46e96d69c5d040fe44603012500707c9f0e8e9c3f432f78494532bc","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"1680951.0","oid":14849962333,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"-2074344.0","oid":14849962111,"twap_id":null,"cloid":null}]}
{"coin":"AXL","side":"B","time":"2024-09-19T10:00:02.015","px":"0.55271","sz":"28.0","hash":"0x2225285dad5694dfb8a8040fe4460301250102e9620e26ae9213655171a128a1","trade_dir_override":"Na","side_info":[{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"125220.0","oid":14849960103,"twap_id":null,"cloid":null},{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-125153.0","oid":14849962335,"twap_id":null,"cloid":null}]}
{"coin":"AXL","side":"A","time":"2024-09-19T10:00:02.015","px":"0.55271","sz":"28.0","hash":"0x2225285dad5694dfb8a8040fe4460301250102e9620e26ae9213655171a128a1","trade_dir_override":"Na","side_info":[{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"125220.0","oid":14849960103,"twap_id":null,"cloid":null},{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-125153.0","oid":14849962335,"twap_id":null,"cloid":null}]}
{"coin":"SUI","side":"B","time":"2024-09-19T10:00:04.351","px":"1.3319","sz":"12.3","hash":"0x3e3f86ab80a01a760e8f040fe4460d010d00582c74bd1be365e928fae9b707e7","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-81500.9","oid":14849964528,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"-159923.9","oid":14849964001,"twap_id":null,"cloid":null}]}
{"coin":"SUI","side":"A","time":"2024-09-19T10:00:04.351","px":"1.3319","sz":"12.3","hash":"0x3e3f86ab80a01a760e8f040fe4460d010d00582c74bd1be365e928fae9b707e7","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-81500.9","oid":14849964528,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"-159923.9","oid":14849964001,"twap_id":null,"cloid":null}]}
{"coin":"JOE","side":"B","time":"2024-09-19T10:00:04.497","px":"0.36109","sz":"50.0","hash":"0x6668edf3f8bc544a014b040fe4460e01180004671742606d8c37386842ba0032","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-133057.0","oid":14849962551,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"40212.0","oid":14849964638,"twap_id":null,"cloid":null}]}
{"coin":"JOE","side":"A","time":"2024-09-19T10:00:04.497","px":"0.36109","sz":"50.0","hash":"0x6668edf3f8bc544a014b040fe4460e01180004671742606d8c37386842ba0032","trade_dir_override":"Na","side_info":[{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-133057.0","oid":14849962551,"twap_id":null,"cloid":null},{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"40212.0","oid":14849964638,"twap_id":null,"cloid":null}]}
{"coin":"ETC","side":"B","time":"2024-09-19T10:00:04.692","px":"18.561","sz":"0.97","hash":"0x7b35799cb75fb8ac486f040fe4460f011c00e79771cf695f5c1c9c1ff30baea6","trade_dir_override":"Na","side_info":[{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"6805.87","oid":14849964201,"twap_id":null,"cloid":null},{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-6829.12","oid":14849964961,"twap_id":null,"cloid":null}]}
{"coin":"ETC","side":"A","time":"2024-09-19T10:00:04.692","px":"18.561","sz":"0.97","hash":"0x7b35799cb75fb8ac486f040fe4460f011c00e79771cf695f5c1c9c1ff30baea6","trade_dir_override":"Na","side_info":[{"user":"0x768484f7e2ebb675c57838366c02ae99ba2a9b08","start_pos":"6805.87","oid":14849964201,"twap_id":null,"cloid":null},{"user":"0xc64cc00b46101bd40aa1c3121195e85c0b0918d8","start_pos":"-6829.12","oid":14849964961,"twap_id":null,"cloid":null}]}
```

## Dune API

### Authentication

Sample request:

```
curl -X POST -H "x-dune-api-key:{{api_key}}" "https://api.dune.com/api/v1/query/{{query_id}}/execute"
```


### Create table

Sample request:

```
curl --request POST \
  --url https://api.dune.com/api/v1/table/create \
  --header 'X-DUNE-API-KEY: <x-dune-api-key>' \
  --header 'Content-Type: application/json' \
  --data '{
  "namespace":"my_user",
  "table_name":"interest_rates",
  "description": "10 year daily interest rates, sourced from https://fred.stlouisfed.org/series/DGS10",
  "is_private": false,
  "schema": [{"name": "date", "type": "timestamp"}, {"name": "dgs10", "type": "double",  "nullable": true}]
}'
```


### Insert data

Sample request:

```
curl --request POST \
  --url https://api.dune.com/api/v1/table/my_user/interest_rates/insert \
  --header 'X-DUNE-API-KEY: <x-dune-api-key>' \
  --header 'Content-Type: text/csv' \
  --upload-file interest_rates.csv
```

The `application/x-ndjson` content type is also accepted instead of `text/csv`, and in that case the data file should be a JSON file.