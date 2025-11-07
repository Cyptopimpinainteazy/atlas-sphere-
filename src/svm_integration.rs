//! SVM Integration Module
//!
//! Solana Virtual Machine integration for Atlas Sphere

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SVM program ID type
pub type ProgramId = String;

/// SVM bytecode type
pub type Bytecode = Vec<u8>;

/// SVM program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub program_id: ProgramId,
    pub bytecode: Bytecode,
    pub deployed: bool,
}

/// SVM integration handler
pub struct SvmIntegration {
    programs: HashMap<ProgramId, Program>,
}

impl SvmIntegration {
    /// Create a new SVM integration handler
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
        }
    }

    /// Deploy a new SVM program
    pub fn deploy_program(&mut self, bytecode: Bytecode) -> Result<ProgramId, String> {
        if bytecode.is_empty() {
            return Err("Invalid bytecode: empty".to_string());
        }

        // Generate a simple program ID
        let program_id = format!("Program{:016x}", self.programs.len() + 1);

        let program = Program {
            program_id: program_id.clone(),
            bytecode,
            deployed: true,
        };

        self.programs.insert(program_id.clone(), program);
        Ok(program_id)
    }

    /// Execute an SVM program
    pub fn execute_program(
        &self,
        program_id: &ProgramId,
        input: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let program = self
            .programs
            .get(program_id)
            .ok_or_else(|| "Program not found".to_string())?;

        if !program.deployed {
            return Err("Program not deployed".to_string());
        }

        // Mock execution - in a real implementation, this would run the SVM
        Ok(input)
    }

    /// Get program by ID
    pub fn get_program(&self, program_id: &ProgramId) -> Option<&Program> {
        self.programs.get(program_id)
    }

    /// Get number of deployed programs
    pub fn program_count(&self) -> usize {
        self.programs.len()
    }
}

impl Default for SvmIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_program() {
        let mut svm = SvmIntegration::new();
        let bytecode = vec![0x01, 0x02, 0x03, 0x04];

        let result = svm.deploy_program(bytecode);
        assert!(result.is_ok());
        assert_eq!(svm.program_count(), 1);
    }

    #[test]
    fn test_deploy_empty_bytecode() {
        let mut svm = SvmIntegration::new();
        let result = svm.deploy_program(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_program() {
        let mut svm = SvmIntegration::new();
        let bytecode = vec![0x01, 0x02];
        let program_id = svm.deploy_program(bytecode).unwrap();

        let input = vec![1, 2, 3];
        let result = svm.execute_program(&program_id, input.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_execute_nonexistent_program() {
        let svm = SvmIntegration::new();
        let result = svm.execute_program(&"InvalidProgram".to_string(), vec![]);
        assert!(result.is_err());
    }
}
