#!/bin/bash

# Start Synergy Testnet Node

echo "🔧 Initializing Synergy Testnet..."

# Set script path to root of the project
cd "$(dirname "$0")/.."

# Prepare data directory
mkdir -p data/logs

# Kill any existing node processes
pkill -f synergy-testnet || true
sleep 1

# Build and launch testnet binary
echo "🚀 Launching node..."
cargo build --release --bin synergy-testnet

# Run the node in background with genesis and config loaded
./target/release/synergy-testnet \
  --genesis config/genesis.json \
  --config config/network-config.toml \
  > data/logs/testnet.out 2>&1 &

NODE_PID=$!
echo "✅ Synergy Testnet started with PID $NODE_PID"
echo $NODE_PID > data/testnet.pid
