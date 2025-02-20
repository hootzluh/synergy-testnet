#!/bin/bash

# Initialize the Synergy Testnet
echo "Initializing Synergy Testnet..."
mkdir -p data

# Start the blockchain node
./src/node/synergyd --config config/network-config.toml --genesis config/genesis.json &
NODE_PID=$!

echo "Synergy Testnet started with PID $NODE_PID"
echo $NODE_PID > data/testnet.pid
