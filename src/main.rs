//! Atlas Sphere Node
//!
//! Main entry point for the Atlas Sphere blockchain node

use atlas_sphere::{runtime::Runtime, NAME, VERSION};

#[tokio::main]
async fn main() {
    println!("======================================");
    println!("  {} v{}", NAME, VERSION);
    println!("======================================");
    println!();

    // Initialize runtime
    let runtime = Runtime::default();
    let config = runtime.config();

    println!("Chain: {}", config.chain_name);
    println!("Runtime Version: {}", config.version);
    println!(
        "EVM Integration: {}",
        if config.evm_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!(
        "SVM Integration: {}",
        if config.svm_enabled {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!();

    println!("Kernel Version: {}", runtime.kernel().version());
    println!("Deployed Contracts: {}", runtime.evm().contract_count());
    println!("Deployed Programs: {}", runtime.svm().program_count());
    println!();

    println!("Node initialized successfully!");
    println!("Ready to process transactions...");
}

#[cfg(test)]
mod tests {
    use atlas_sphere::runtime::Runtime;

    #[test]
    fn test_node_initialization() {
        let runtime = Runtime::default();
        assert_eq!(runtime.config().version, 1);
    }
}
