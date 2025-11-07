//! Atlas Sphere Blockchain Platform
//!
//! A cutting-edge blockchain platform built on Substrate, featuring seamless integration
//! of Ethereum Virtual Machine (EVM) and Solana Virtual Machine (SVM) capabilities.

pub mod evm_integration;
pub mod kernel;
pub mod runtime;
pub mod svm_integration;

pub use evm_integration::EvmIntegration;
pub use kernel::KernelOperations;
pub use svm_integration::SvmIntegration;

/// Version information for Atlas Sphere
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = "Atlas Sphere";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn test_name() {
        assert_eq!(NAME, "Atlas Sphere");
    }
}
