//! Runtime Module
//!
//! Core runtime for the Atlas Sphere blockchain

use crate::{EvmIntegration, KernelOperations, SvmIntegration};
use serde::{Deserialize, Serialize};

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub chain_name: String,
    pub version: u32,
    pub evm_enabled: bool,
    pub svm_enabled: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            chain_name: "Atlas Sphere".to_string(),
            version: 1,
            evm_enabled: true,
            svm_enabled: true,
        }
    }
}

/// Main runtime for Atlas Sphere
pub struct Runtime {
    config: RuntimeConfig,
    kernel: KernelOperations,
    evm: EvmIntegration,
    svm: SvmIntegration,
}

impl Runtime {
    /// Create a new runtime instance
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            kernel: KernelOperations::new(),
            evm: EvmIntegration::new(),
            svm: SvmIntegration::new(),
        }
    }

    /// Get runtime configuration
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Get kernel operations
    pub fn kernel(&self) -> &KernelOperations {
        &self.kernel
    }

    /// Get mutable EVM integration
    pub fn evm_mut(&mut self) -> &mut EvmIntegration {
        &mut self.evm
    }

    /// Get EVM integration
    pub fn evm(&self) -> &EvmIntegration {
        &self.evm
    }

    /// Get mutable SVM integration
    pub fn svm_mut(&mut self) -> &mut SvmIntegration {
        &mut self.svm
    }

    /// Get SVM integration
    pub fn svm(&self) -> &SvmIntegration {
        &self.svm
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::default();
        assert_eq!(runtime.config().chain_name, "Atlas Sphere");
        assert_eq!(runtime.config().version, 1);
        assert!(runtime.config().evm_enabled);
        assert!(runtime.config().svm_enabled);
    }

    #[test]
    fn test_runtime_integrations() {
        let mut runtime = Runtime::default();

        // Test EVM integration
        let bytecode = vec![0x60, 0x80];
        let address = runtime.evm_mut().deploy_contract(bytecode).unwrap();
        assert!(runtime.evm().get_contract(&address).is_some());

        // Test SVM integration
        let bytecode = vec![0x01, 0x02];
        let program_id = runtime.svm_mut().deploy_program(bytecode).unwrap();
        assert!(runtime.svm().get_program(&program_id).is_some());
    }
}
