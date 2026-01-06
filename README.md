# WarpScan

```
██╗    ██╗ █████╗ ██████╗ ██████╗ ███████╗ ██████╗ █████╗ ███╗   ██╗
██║    ██║██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝██╔══██╗████╗  ██║
██║ █╗ ██║███████║██████╔╝██████╔╝███████╗██║     ███████║██╔██╗ ██║
██║███╗██║██╔══██║██╔══██╗██╔═══╝ ╚════██║██║     ██╔══██║██║╚██╗██║
╚███╔███╔╝██║  ██║██║  ██║██║     ███████║╚██████╗██║  ██║██║ ╚████║
 ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝

    🚀 Terminal-based Ethereum Blockchain Explorer 🚀
```

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)](https://ethereum.org/)
[![Terminal](https://img.shields.io/badge/Terminal-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)](https://github.com/microsoft/terminal)

## 🌟 Overview

**WarpScan** is a comprehensive terminal-based Ethereum blockchain explorer designed for developers and power users who prefer command-line interfaces. It brings the functionality of web-based explorers like Etherscan directly to your terminal, providing real-time access to blockchain data, contract analysis, transaction monitoring, and wallet management in an efficient TUI (Text User Interface) environment.

Built with **Rust** for maximum performance and reliability, WarpScan emphasizes speed, security, and developer productivity, allowing you to query blocks, transactions, addresses, contracts, and tokens without ever leaving your terminal.

## ✨ Key Features

### 🏠 **Core Functionality**

- **📊 Home Dashboard**: Network status, recent blocks, gas tracker, and universal search interface
- **🔍 Block Explorer**: Detailed block information, transactions, and seamless navigation
- **💳 Transaction Viewer**: Comprehensive transaction details, logs, traces, and gas analysis
- **🏠 Address Lookup**: Balances, transaction history, token holdings, and contract verification
- **📜 Contract Explorer**: Source code viewing, ABI inspection, read/write function calls, and event monitoring
- **🪙 Token Information**: Token details, holder analysis, transfers, and comprehensive metrics

### 🛠️ **Advanced Tools**

- **✅ Contract Verification**: Source code upload, compilation, and verification workflows
- **👛 Test Wallet Manager**: Generate, import, and manage test wallets for contract interactions
- **🔐 Multi-sig Wallet**: Create and manage multi-signature wallets with threshold controls
- **⛽ Gas Tracker**: Real-time gas prices, historical trends, and estimation tools
- **🔎 Universal Search**: Unified search across addresses, transactions, blocks, and contracts with advanced filtering

### 🎨 **User Experience**

- **🖥️ Beautiful TUI**: Modern terminal interface with intuitive navigation
- **⚡ Real-time Updates**: Live blockchain data with automatic refresh
- **🎯 Keyboard-driven**: Efficient navigation with vim-like keybindings
- **🌈 Theme Support**: Customizable color schemes and layouts

## 🏗️ Project Structure

```
warpscan/
├── 📁 src/
│   ├── 🦀 main.rs                    # Application entry point
│   ├── 📚 lib.rs                     # Library root with module declarations
│   │
│   ├── 🔗 blockchain/                # Blockchain integration layer
│   │   ├── mod.rs                    # Module declarations
│   │   ├── service.rs                # Core blockchain service implementation
│   │   └── types.rs                  # Blockchain-specific type definitions
│   │
│   ├── 💾 cache/                     # Caching system
│   │   ├── mod.rs                    # Cache module declarations
│   │   ├── manager.rs                # Cache management logic
│   │   └── types.rs                  # Cache type definitions
│   │
│   ├── ⚙️ config/                    # Configuration management
│   │   ├── mod.rs                    # Config module declarations
│   │   ├── manager.rs                # Configuration loading and validation
│   │   └── types.rs                  # Configuration type definitions
│   │
│   ├── ❌ error/                     # Error handling system
│   │   ├── mod.rs                    # Error module declarations
│   │   ├── types.rs                  # Error type definitions
│   │   └── helpers.rs                # Error handling utilities
│   │
│   ├── 📝 logging/                   # Logging infrastructure
│   │   ├── mod.rs                    # Logging module declarations
│   │   ├── setup.rs                  # Logging configuration
│   │   ├── macros.rs                 # Logging macros
│   │   ├── perf.rs                   # Performance logging
│   │   └── utils.rs                  # Logging utilities
│   │
│   ├── 📊 models/                    # Data models and structures
│   │   ├── mod.rs                    # Models module declarations
│   │   ├── wallet_info.rs            # Wallet information models
│   │   ├── multisig.rs               # Multi-signature wallet models
│   │   └── wallet_stats.rs           # Wallet statistics models
│   │
│   ├── 🎨 ui/                        # User interface layer
│   │   ├── mod.rs                    # UI module declarations
│   │   │
│   │   ├── 🏛️ app/                   # Application state management
│   │   │   ├── mod.rs                # App module declarations
│   │   │   ├── core.rs               # Core application state
│   │   │   ├── state.rs              # Application state definitions
│   │   │   ├── navigation.rs         # Navigation logic
│   │   │   ├── input.rs              # Input handling
│   │   │   ├── data.rs               # Data management
│   │   │   ├── address.rs            # Address-specific state
│   │   │   └── ui_state.rs           # UI state management
│   │   │
│   │   ├── 🧩 components/            # Reusable UI components
│   │   │   ├── mod.rs                # Components module declarations
│   │   │   ├── input_field.rs        # Input field component
│   │   │   ├── loading.rs            # Loading indicator component
│   │   │   ├── error.rs              # Error display component
│   │   │   ├── success.rs            # Success message component
│   │   │   ├── progress.rs           # Progress bar component
│   │   │   ├── status_bar.rs         # Status bar component
│   │   │   └── help_popup.rs         # Help popup component
│   │   │
│   │   ├── 🎬 events/                # Event handling system
│   │   │   ├── mod.rs                # Events module declarations
│   │   │   ├── handler.rs            # Event handler implementation
│   │   │   ├── types.rs              # Event type definitions
│   │   │   └── utils.rs              # Event utilities
│   │   │
│   │   ├── 📋 models/                # UI-specific data models
│   │   │   ├── mod.rs                # UI models module declarations
│   │   │   ├── address.rs            # Address display models
│   │   │   ├── block_info.rs         # Block information models
│   │   │   ├── transaction.rs        # Transaction display models
│   │   │   ├── token.rs              # Token information models
│   │   │   ├── dashboard_data.rs     # Dashboard data models
│   │   │   ├── network_stats.rs      # Network statistics models
│   │   │   ├── search_result.rs      # Search result models
│   │   │   ├── internal_transaction.rs # Internal transaction models
│   │   │   └── daily_transaction_data.rs # Daily transaction data models
│   │   │
│   │   ├── 📺 screens/               # Application screens/views
│   │   │   ├── mod.rs                # Screens module declarations
│   │   │   ├── home.rs               # Home dashboard screen
│   │   │   ├── address_lookup.rs     # Address lookup screen
│   │   │   ├── block_explorer.rs     # Block explorer screen
│   │   │   ├── transaction_viewer.rs # Transaction viewer screen
│   │   │   ├── gas_tracker.rs        # Gas tracker screen
│   │   │   └── wallet_manager.rs     # Wallet manager screen
│   │   │
│   │   └── 🎨 theme/                 # Theme and styling system
│   │       ├── mod.rs                # Theme module declarations
│   │       ├── manager.rs            # Theme management
│   │       ├── colors.rs             # Color definitions
│   │       └── styles.rs             # Style definitions
│   │
│   └── 👛 wallet.rs                  # Wallet management functionality
│
├── 📄 Cargo.toml                     # Rust project configuration
├── 📄 Cargo.lock                     # Dependency lock file
├── 📄 README.md                      # This file
├── 📄 .gitignore                     # Git ignore rules
└── 📁 target/                        # Build artifacts (ignored)
```

## 🛠️ Technology Stack

| Component            | Technology                                                                                            | Purpose                                  |
| -------------------- | ----------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| **Language**         | ![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)                | Core language for performance and safety |
| **UI Framework**     | ![Ratatui](https://img.shields.io/badge/Ratatui-FF6B6B?style=flat)                                    | Terminal user interface rendering        |
| **Terminal Backend** | ![Crossterm](https://img.shields.io/badge/Crossterm-4ECDC4?style=flat)                                | Cross-platform terminal manipulation     |
| **Blockchain**       | ![Ethers-rs](https://img.shields.io/badge/Ethers--rs-627EEA?style=flat&logo=ethereum&logoColor=white) | Ethereum blockchain integration          |
| **Async Runtime**    | ![Tokio](https://img.shields.io/badge/Tokio-000000?style=flat)                                        | Asynchronous runtime                     |
| **Serialization**    | ![Serde](https://img.shields.io/badge/Serde-DE3F24?style=flat)                                        | Data serialization/deserialization       |
| **Configuration**    | ![TOML](https://img.shields.io/badge/TOML-9C4221?style=flat)                                          | Configuration file format                |
| **Logging**          | ![Tracing](https://img.shields.io/badge/Tracing-40E0D0?style=flat)                                    | Structured logging and diagnostics       |

## 🏛️ Architecture

WarpScan follows a clean, layered architecture designed for maintainability and extensibility:

```
┌─────────────────────────────────────────────────────────────┐
│                    🎨 Presentation Layer                    │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐ │
│  │   Screens   │ │ Components  │ │      Theme System       │ │
│  │             │ │             │ │                         │ │
│  └─────────────┘ └─────────────┘ └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                   🧠 Application Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐ │
│  │ App State   │ │ Navigation  │ │    Event Handling       │ │
│  │             │ │             │ │                         │ │
│  └─────────────┘ └─────────────┘ └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    🔧 Service Layer                         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐ │
│  │ Blockchain  │ │    Cache    │ │    Wallet Manager       │ │
│  │  Service    │ │   Manager   │ │                         │ │
│  └─────────────┘ └─────────────┘ └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                 🌐 External Services                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐ │
│  │ Ethereum    │ │   Infura    │ │       Alchemy           │ │
│  │ RPC Nodes   │ │             │ │                         │ │
│  └─────────────┘ └─────────────┘ └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Installation

### Prerequisites

- **Rust** (1.70.0 or later) - Install via [rustup](https://rustup.rs/)
- **Git** - For cloning the repository
- **Terminal** - Any modern terminal emulator

### Quick Start

1. **Clone the repository**

   ```bash
   git clone https://github.com/your-username/warpscan.git
   cd warpscan
   ```

2. **Build the project**

   ```bash
   cargo build --release
   ```

3. **Configure your RPC endpoint**

   ```bash
   # Create config directory
   mkdir -p ~/.warpscan

   # Create configuration file
   cat > ~/.warpscan/config.toml << EOF
   [network]
   rpc_url = "https://mainnet.infura.io/v3/YOUR_PROJECT_ID"
   chain_id = 1

   [cache]
   enabled = true
   ttl_seconds = 300

   [ui]
   theme = "default"
   refresh_interval = 5
   EOF
   ```

4. **Run WarpScan**
   ```bash
   cargo run --release
   ```

### Use Etherscan and Local Anvil Mainnet Fork

WarpScan can use both an Ethereum RPC node (Infura/Alchemy/custom) and the Etherscan API (optional). For development, running against a local Anvil mainnet fork is recommended.

1. Etherscan (optional)

- Set an environment variable `ETHERSCAN_API_KEY`. WarpScan auto-detects it.

```bash
export ETHERSCAN_API_KEY=your_key
```

2. Anvil mainnet fork (local testing)

- Requires Foundry Anvil (`curl -L https://foundry.paradigm.xyz | bash`).
- Start a fork using the helper script and an upstream mainnet RPC:

```bash
# Using Infura/Alchemy/etc. as the upstream:
ETH_RPC_URL=https://mainnet.infura.io/v3/YOUR_PROJECT_ID \
./scripts/anvil-mainnet-fork.sh
```

- Update your WarpScan config (`~/.warpscan/config.toml`) to point at the local fork and mark node type as `anvil` so WarpScan skips Etherscan when appropriate:

```toml
[network]
name = "Anvil Mainnet Fork"
rpc_url = "http://127.0.0.1:8545"
chain_id = 1
timeout_seconds = 30
node_type = "anvil"
```

Optional: Pin the fork to a specific block by adding `FORK_BLOCK=21000000` before the script command.

### Development Setup

For development, you can run in debug mode with additional logging:\*\*\*\*

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with trace logging for detailed debugging
RUST_LOG=trace cargo run

# Run tests
cargo test

# Run with code coverage
cargo test --coverage
```

## 🎮 Usage

### Navigation

WarpScan uses intuitive keyboard controls:

| Key              | Action                        |
| ---------------- | ----------------------------- |
| `↑↓←→` or `hjkl` | Navigate menus and lists      |
| `Enter`          | Select item or confirm action |
| `Esc` or `q`     | Go back or quit               |
| `/`              | Open search                   |
| `Tab`            | Switch between tabs           |
| `?`              | Show help                     |
| `Ctrl+C`         | Force quit                    |

### Quick Start Guide

1. **🏠 Home Dashboard**: Start here to see network overview and recent activity
2. **🔍 Search**: Use `/` to search for addresses, transactions, blocks, or contracts
3. **📊 Explore**: Navigate through different sections using arrow keys or vim keys
4. **👛 Wallet**: Manage test wallets for contract interactions
5. **⛽ Gas**: Monitor current gas prices and trends

### Example Workflows

**🔍 Searching for a Transaction:**

```
1. Press '/' to open search
2. Enter transaction hash: 0x1234...
3. Press Enter to search
4. View detailed transaction information
```

**👛 Creating a Test Wallet:**

```
1. Navigate to Wallet Manager
2. Select "Generate New Wallet"
3. Optionally provide a name
4. Save the generated mnemonic securely
```

**📜 Exploring a Contract:**

```
1. Search for contract address
2. View contract details and verified source
3. Interact with read/write functions
4. Monitor contract events
```

## ⚙️ Configuration

WarpScan uses a TOML configuration file located at `~/.warpscan/config.toml`:

```toml
[network]
# Ethereum RPC endpoint
rpc_url = "https://mainnet.infura.io/v3/YOUR_PROJECT_ID"
# Chain ID (1 for mainnet, 5 for goerli, etc.)
chain_id = 1
# Request timeout in seconds
timeout = 30

[cache]
# Enable caching for better performance
enabled = true
# Cache TTL in seconds
ttl_seconds = 300
# Maximum cache size in MB
max_size_mb = 100

[ui]
# UI theme (default, dark, light)
theme = "default"
# Auto-refresh interval in seconds
refresh_interval = 5
# Enable mouse support
mouse_support = true

[logging]
# Log level (error, warn, info, debug, trace)
level = "info"
# Log to file
file_logging = false
# Log file path
log_file = "~/.warpscan/warpscan.log"

[wallet]
# Default derivation path for HD wallets
derivation_path = "m/44'/60'/0'/0/0"
# Enable test mode (generates test wallets)
test_mode = true
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### 🐛 Reporting Issues

- Use GitHub Issues to report bugs
- Include steps to reproduce
- Provide system information and logs

### 💡 Feature Requests

- Describe the feature and its use case
- Check existing issues to avoid duplicates
- Consider contributing the implementation

### 🔧 Development Process

1. **Fork the repository**

   ```bash
   git fork https://github.com/your-username/warpscan.git
   ```

2. **Create a feature branch**

   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **Make your changes**

   - Follow Rust conventions
   - Add tests for new functionality
   - Update documentation as needed

4. **Test your changes**

   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Commit and push**

   ```bash
   git commit -m "Add amazing feature"
   git push origin feature/amazing-feature
   ```

6. **Open a Pull Request**
   - Describe your changes
   - Link related issues
   - Ensure CI passes

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Made with ❤️ by the WarpScan team**

</div>
