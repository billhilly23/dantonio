use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

mod oracle_config;

fn main() {
    // Initialize the Web3 provider
    let web3 = Web3::new("http://localhost:8545");

    // Initialize the Oracle contract configuration
    let oracle_config = oracle_config::OracleConfig::new(web3.clone(), "0x1234567890abcdef".parse().unwrap());

    // Update the configuration
    oracle_config.update_configuration("oracle_contract_address".to_string().into_bytes().into(), "0x1234567890abcdef".parse().unwrap());

    // Get the configuration
    let configuration = oracle_config.get_configuration("oracle_contract_address".to_string().into_bytes().into());
    println!("Configuration: {}", configuration);
}
