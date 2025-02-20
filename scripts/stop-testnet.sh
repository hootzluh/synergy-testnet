#!/bin/bash

# Stop the Synergy Testnet
echo "Stopping Synergy Testnet..."
if [ -f data/testnet.pid ]; then
    NODE_PID=$(cat data/testnet.pid)
    kill $NODE_PID
    rm data/testnet.pid
    echo "Synergy Testnet stopped."
else
    echo "No running Synergy Testnet instance found."
fi
