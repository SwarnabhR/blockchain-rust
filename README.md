# Rust Blockchain

A simple blockchain implementation in Rust with peer-to-peer communication and synchronization.

## Features

- **Blockchain Creation:** Initialize a blockchain with a genesis block.
- **Block Mining:** Add and mine new blocks with a configurable difficulty level.
- **Blockchain Validation:** Validate the integrity of the blockchain.
- **Peer-to-Peer Communication:** Propagate new blocks and synchronize with peers.

## Requirements

- Rust (with `cargo`)
- `tokio` runtime
- `warp` for the HTTP server
- `serde` and `serde_json` for serialization
- `reqwest` for HTTP requests
- `sha2` for hashing

## Setup

1. **Clone the Repository**

```sh
git clone https://github.com/your-username/rust-blockchain.git
cd rust-blockchain
```

2. Install Dependencies

Ensure you have Rust installed. Run the following command to install the necessary dependencies:

```sh
cargo build
```

## Usage

1. Start the Blockchain Node

```sh
cargo run
```

2. Interact with the Blockchain

- Get the Blockchain

```sh
curl http://127.0.0.1:3030/blockchain
```

- Add a new block

```sh
curl -X POST http://127.0.0.1:3030/block -H "Content-Type: application/json" -d '{"data": "New Block Data"}'
```

## Code Structure

- **`src/main.rs`**: Entry point for the application, sets up the server and handles HTTP requests.
- **`src/block.rs`**: Contains the definition of the `Block` struct and its methods.
- **`src/blockchain.rs`**: Contains the definition of the `Blockchain` struct and its methods for managing the blockchain.
- **`Cargo.toml`**: Project configuration file for managing dependencies.

### Key Components

- **Block**: Represents a block in the blockchain with methods for hash calculation and mining.
- **Blockchain**: Manages the chain of blocks, validates the chain, and handles peer-to-peer communication.
- **Server**: Uses `warp` to expose HTTP endpoints for blockchain operations.

## Peer-to-Peer Communication

- **Notification of New Blocks**: When a new block is added, it is propagated to other peers.
- **Synchronization**: Nodes synchronize their blockchain with peers to ensure consistency.

## Contributing

Contributions are welcome! Please adhere to the following guidelines:

1. Fork the repository and create a feature branch.
2. Write tests for your changes.
3. Ensure code style consistency.
4. Submit a pull request with a clear description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or feedback, please reach out to [swarnabh15.roy@gmail.com](mailto:your-swarnabh15.roy@gmail.com).
