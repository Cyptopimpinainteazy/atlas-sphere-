# Atlas Sphere

Atlas Sphere is a cutting-edge blockchain platform built on Substrate, featuring seamless integration of Ethereum Virtual Machine (EVM) and Solana Virtual Machine (SVM) capabilities. It includes custom pallets for kernel operations, runtime modules for EVM and SVM execution, and developer tools for building decentralized applications.

## Features

- 🔧 **Kernel Pallet**: Core blockchain operations and system management
- ⚡ **EVM Integration**: Deploy and execute Ethereum smart contracts
- 🚀 **SVM Integration**: Run Solana programs natively
- 🏗️ **Substrate Framework**: Built on battle-tested blockchain infrastructure
- 🔒 **Security**: Leverages Substrate's security features

## Project Structure

```
atlas-sphere-/
├── runtime/          # Blockchain runtime with pallet configurations
├── node/             # Node service for network operations
├── pallets/
│   ├── kernel/       # Core kernel operations pallet
│   ├── evm-integration/  # EVM support pallet
│   └── svm-integration/  # SVM support pallet
├── config.toml       # Node configuration
└── DEVELOPER_GUIDE.md    # Development documentation
```

## Quick Start

### Prerequisites

- Rust toolchain (stable)
- Substrate development dependencies

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release -p atlas-sphere-node
```

### Test

```bash
cargo test --workspace
```

## Documentation

See [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) for detailed development instructions.

## License

MIT License - see [LICENSE](LICENSE) for details
