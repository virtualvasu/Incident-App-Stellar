# Incident-App-Stellar

deployed incident contract:CALH5YR6MDFTC5Z5NILDXKNXPEF3FX2Y33ULI4ELPMMHO2SSJMYYUKWE

alias: incident-report

developer:

cd stellar-contracts

stellar contract build

cargo test


stellar contract upload \
  --network testnet \
  --source vasu \
  --wasm target/wasm32v1-none/release/incident_report.wasm

  stellar contract deploy \
  --wasm-hash 9082a394bcab327186f1269e426d95385a32d60ebc47ff4f1ec3b7fbab897fd7 \
  --source vasu \
  --network testnet \
  --alias incident-report
