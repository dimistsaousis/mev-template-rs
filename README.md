# Mev Project

This project is a Rust-based application that interacts with the Ethereum blockchain to monitor and interact with Uniswap's decentralized exchanges. The application monitors the Ethereum mempool for transactions related to Uniswap, inspects new blocks, fetches pairs from the Uniswap contracts, and sends alerts via Discord.

## Directory Structure

```
.
├── src/
│   ├── address_book.rs    # Contains ABIs and generated bindings for interacting with Uniswap contracts
│   ├── alert.rs           # Sends alert messages to a specified Discord channel
│   ├── block_scanner.rs   # Monitors the blockchain for new blocks
│   ├── config.rs          # Configuration for blockchain network connections
│   ├── dex.rs             # Interactions with Uniswap
│   ├── helpers.rs         # Helper functions
│   ├── lib.rs
│   ├── main.rs
│   └── mempool.rs         # Monitors the Ethereum mempool for transactions
├── json_comparison.py     # Script to compare two JSON files
└── env.example            # Example environment configuration file
```

## Getting Started

1. **Environment Setup**:

   - Rename `env.example` to `.env` and fill in the necessary details.
   - `PRIVATE_KEY`: Your private key for signing transactions.
   - `NETWORK_RPC`: RPC URL of the Ethereum network.
   - `NETWORK_WSS`: WebSocket URL of the Ethereum network (optional, for subscribing to event streams).
   - `DISCORD_WEBHOOK`: Webhook URL for sending alerts to a Discord channel.

2. **Run the Rust Project**:

   - From the project root, run the following command to start the Rust application:
     ```bash
     cargo run
     ```

3. **Run the JSON Comparison Script**:

   - From the project root, run the following command to compare two JSON files:
     ```bash
     python json_comparison.py <path_to_json1> <path_to_json2>
     ```
     Replace `<path_to_json1>` and `<path_to_json2>` with the paths to the JSON files you want to compare. The script will output `EQUAL` if the JSON files are identical, or `NOT EQUAL` if they differ.
