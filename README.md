# WarpScan

## Overview

WarpScan is a comprehensive terminal-based Ethereum blockchain explorer designed for developers and power users who prefer command-line interfaces. It brings the functionality of web-based explorers like Etherscan to the terminal, providing real-time access to blockchain data, contract analysis, transaction monitoring, and more in an efficient TUI (Text User Interface) environment.

Built with Rust, WarpScan emphasizes speed, reliability, and security, allowing users to query blocks, transactions, addresses, contracts, and tokens without leaving their terminal.

## Key Features

- **Home Dashboard**: Network status, recent blocks, gas tracker, and universal search interface.
- **Block Explorer**: Detailed block information, transactions, and navigation.
- **Transaction Viewer**: Comprehensive transaction details, logs, traces, and gas analysis.
- **Address Lookup**: Balances, transaction history, token holdings, and contract verification.
- **Contract Explorer**: Source code viewing, ABI inspection, read/write function calls, and event monitoring.
- **Token Information**: Token details, holder analysis, transfers, and metrics.
- **Contract Verification**: Source code upload, compilation, and verification.
- **Test Wallet Manager**: Generate, import, and manage test wallets for contract interactions.
- **Multi-sig Wallet**: Create and manage multi-signature wallets.
- **Gas Tracker**: Real-time gas prices, historical trends, and estimation tools.
- **Search System**: Unified search across addresses, transactions, blocks, and contracts with filtering.

## Technology Stack

- **Language**: Rust
- **UI Framework**: Ratatui (with Crossterm)
- **Blockchain Integration**: Ethers-rs
- **Data Storage**: Local file-based cache and in-memory state
- **Configuration**: TOML
- **Logging**: Env_logger or Tracing

## Architecture

WarpScan follows a layered architecture:

- **Frontend Layer**: TUI with ratatui for rendering and event handling.
- **Application Layer**: Core logic managing screens, state, and navigation.
- **Service Layer**: Blockchain services using ethers-rs, caching, and configuration.
- **External Services**: Ethereum RPC nodes (e.g., Infura, Alchemy).

For more details, see the [technical architecture document](.trae/documents/warpscan_technical_architecture.md).

## Installation

1. Ensure you have Rust installed (via [rustup](https://rustup.rs/)).
2. Clone the repository:
   ```bash
   git clone <repository-url>
   cd warpscan
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Configure the application by editing `~/.warpscan/config.toml` with your RPC endpoint (e.g., Infura project ID).
5. Run the application:
   ```bash
   cargo run
   ```

## Usage

Launch WarpScan with `cargo run`. Use keyboard shortcuts for navigation:
- Arrow keys or `h/j/k/l` for movement.
- `Enter` to select.
- `q` or `Esc` to go back.
- `/` to search.

The home dashboard provides an overview. Search for addresses, transaction hashes, block numbers, or contracts.

## Development Status

The project is in active development. See the [implementation todo list](.trae/documents/warpscan_implementation_todo.md) for current progress and planned features.

## Contributing

Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/amazing-feature`).
3. Commit changes (`git commit -m 'Add amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Documentation

- [Product Requirements](.trae/documents/warpscan_prd.md)
- [Technical Architecture](.trae/documents/warpscan_technical_architecture.md)
- [Implementation Todo](.trae/documents/warpscan_implementation_todo.md)