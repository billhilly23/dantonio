use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct OracleConfig {
    pub web3: Web3,
    pub oracle_contract_address: Address,
}

impl OracleConfig {
    pub fn new(web3: Web3, oracle_contract_address: Address) -> Self {
        OracleConfig {
            web3,
            oracle_contract_address,
        }
    }

    pub fn update_configuration(&self, key: bytes32, value: bytes32) {
        // Update the configuration
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.oracle_contract_address,
                to: self.oracle_contract_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_update_configuration(key, value),
            },
        );
        self.web3.eth().wait_for_transaction_receipt(tx_hash);
    }

    pub fn get_configuration(&self, key: bytes32) -> bytes32 {
        // Get the configuration
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.oracle_contract_address,
                to: self.oracle_contract_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_get_configuration(key),
            },
        );
        let receipt = self.web3.eth().wait_for_transaction_receipt(tx_hash);
        let configuration = decode_get_configuration(receipt);
        configuration
    }
}

fn encode_update_configuration(key: bytes32, value: bytes32) -> Vec<u8> {
    // Encode the update configuration function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    data.extend_from_slice(&key.to_vec());
    data.extend_from_slice(&value.to_vec());
    data
}

fn encode_get_configuration(key: bytes32) -> Vec<u8> {
    // Encode the get configuration function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    data.extend_from_slice(&key.to_vec());
    data
}

fn decode_get_configuration(receipt: TransactionReceipt) -> bytes32 {
    // Decode the get configuration function call
    let data = receipt.logs[0].data;
    let configuration = bytes32::from_slice(&data);
    configuration
}
