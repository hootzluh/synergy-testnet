#!/bin/bash

# Initialize the Synergy Testnet
echo "Initializing Synergy Testnet..."
mkdir -p data

# Kill any existing synergy-testnet processes
pkill -f synergy-testnet || true

# Give the system a moment to free resources
sleep 1

# Use full path to cargo with the correct binary name
$HOME/.cargo/bin/cargo run --release --bin synergy-testnet -- --genesis config/genesis.json &
NODE_PID=$!

echo "Synergy Testnet started with PID $NODE_PID"
echo $NODE_PID > data/testnet.pid
