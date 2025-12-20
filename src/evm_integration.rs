//! EVM Integration Module
//!
//! Ethereum Virtual Machine integration for Atlas Sphere

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// EVM contract address type
pub type Address = String;

/// EVM bytecode type
pub type Bytecode = Vec<u8>;

/// EVM contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub address: Address,
    pub bytecode: Bytecode,
    pub deployed: bool,
}

/// EVM integration handler
pub struct EvmIntegration {
    contracts: HashMap<Address, Contract>,
}

impl EvmIntegration {
    /// Create a new EVM integration handler
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    /// Deploy a new EVM contract
    pub fn deploy_contract(&mut self, bytecode: Bytecode) -> Result<Address, String> {
        if bytecode.is_empty() {
            return Err("Invalid bytecode: empty".to_string());
        }

        // Generate a simple contract address
        let address = format!("0x{:040x}", self.contracts.len() + 1);

        let contract = Contract {
            address: address.clone(),
            bytecode,
            deployed: true,
        };

        self.contracts.insert(address.clone(), contract);
        Ok(address)
    }

    /// Execute an EVM contract
    pub fn execute_contract(&self, address: &Address, input: Vec<u8>) -> Result<Vec<u8>, String> {
        let contract = self
            .contracts
            .get(address)
            .ok_or_else(|| "Contract not found".to_string())?;

        if !contract.deployed {
            return Err("Contract not deployed".to_string());
        }

        // Mock execution - in a real implementation, this would run the EVM
        Ok(input)
    }

    /// Get contract at address
    pub fn get_contract(&self, address: &Address) -> Option<&Contract> {
        self.contracts.get(address)
    }

    /// Get number of deployed contracts
    pub fn contract_count(&self) -> usize {
        self.contracts.len()
    }
}

impl Default for EvmIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_contract() {
        let mut evm = EvmIntegration::new();
        let bytecode = vec![0x60, 0x80, 0x60, 0x40];

        let result = evm.deploy_contract(bytecode);
        assert!(result.is_ok());
        assert_eq!(evm.contract_count(), 1);
    }

    #[test]
    fn test_deploy_empty_bytecode() {
        let mut evm = EvmIntegration::new();
        let result = evm.deploy_contract(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_contract() {
        let mut evm = EvmIntegration::new();
        let bytecode = vec![0x60, 0x80];
        let address = evm.deploy_contract(bytecode).unwrap();

        let input = vec![1, 2, 3];
        let result = evm.execute_contract(&address, input.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_execute_nonexistent_contract() {
        let evm = EvmIntegration::new();
        let result = evm.execute_contract(&"0x123".to_string(), vec![]);
        assert!(result.is_err());
    }
}
