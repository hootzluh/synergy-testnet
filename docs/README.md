# Synergy Network Testnet

## Overview

The **Synergy Network Testnet** is an experimental blockchain environment designed to test and refine the Synergy Network before its mainnet launch. This testnet is built **from scratch**, independent of existing blockchains, using a novel **Proof of Synergy (PoSy)** consensus mechanism.

## Features

- **Custom blockchain protocol** with a focus on scalability and decentralization.
- **Peer-to-peer networking** using Libp2p.
- **Efficient state storage** leveraging RocksDB.
- **JSON-RPC API** for seamless blockchain interaction.
- **Secure cryptographic signing** using Ed25519/ECDSA.

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/YOUR_USERNAME/synergy-testnet.git
cd synergy-testnet
```

### 2. Install Dependencies

Refer to the `dependencies/` directory for installation scripts.

### 3. Start the Testnet

Run the setup script:

```sh
bash scripts/start-testnet.sh
```

### 4. Stop the Testnet

To safely shut down the testnet:

```sh
bash scripts/stop-testnet.sh
```

## Repository Structure

```sh
synergy-testnet/
│── config/                # Blockchain configuration files
│── src/                   # Source code for nodes, consensus, and RPC
│── scripts/               # Utility scripts
│── docs/                  # Documentation
│── dependencies/          # External dependencies
│── tests/                 # Test cases
│── .gitignore             # Ignored files
│── LICENSE                # License file
```

## Contributing

Contributions are welcome! Please follow the [contribution guidelines](docs/contributing.md) when submitting pull requests.

## License

This project is licensed under the [MIT License](LICENSE).
