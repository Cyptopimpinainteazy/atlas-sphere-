# Developer Guide

## Building Atlas Sphere

### Prerequisites

- Rust toolchain (stable)
- Substrate development dependencies

### Build Instructions

```bash
# Build all components
cargo build --release

# Build runtime only
cargo build --release -p atlas-sphere-runtime

# Build node
cargo build --release -p atlas-sphere-node
```

## Running the Node

```bash
# Run the node
cargo run --release -p atlas-sphere-node
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific pallet
cargo test -p pallet-kernel
cargo test -p pallet-evm-integration
cargo test -p pallet-svm-integration
```

## Architecture

### Pallets

- **Kernel Pallet**: Core kernel operations
- **EVM Integration**: Ethereum Virtual Machine support
- **SVM Integration**: Solana Virtual Machine support

### Runtime

The runtime combines all pallets and provides the blockchain logic.

### Node

The node service handles network operations and consensus.

## EVM Integration

Deploy and execute Ethereum smart contracts on Atlas Sphere:

```rust
// Deploy a contract
EvmIntegration::deploy_contract(origin, bytecode)

// Execute a contract
EvmIntegration::execute_contract(origin, contract_address, input)
```

## SVM Integration

Deploy and execute Solana programs on Atlas Sphere:

```rust
// Deploy a program
SvmIntegration::deploy_program(origin, bytecode)

// Execute a program
SvmIntegration::execute_program(origin, program_id, input)
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request
