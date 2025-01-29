use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct PriceFeed {
    pub web3: Web3,
    pub token_address: Address,
}

impl PriceFeed {
    pub fn new(web3: Web3, token_address: Address) -> Self {
        PriceFeed {
            web3,
            token_address,
        }
    }

    pub fn update_price(&self, price: U256) {
        // Update the price feed
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.token_address,
                to: self.token_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_update_price(price),
            },
        );
        self.web3.eth().wait_for_transaction_receipt(tx_hash);
    }

    pub fn get_price(&self) -> U256 {
        // Get the price feed
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.token_address,
                to: self.token_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_get_price(),
            },
        );
        let receipt = self.web3.eth().wait_for_transaction_receipt(tx_hash);
        let price = decode_get_price(receipt);
        price
    }
}

fn encode_update_price(price: U256) -> Vec<u8> {
    // Encode the update price function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    data.extend_from_slice(&price.to_vec());
    data
}

fn encode_get_price() -> Vec<u8> {
    // Encode the get price function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    data
}

fn decode_get_price(receipt: TransactionReceipt) -> U256 {
    // Decode the get price function call
    let data = receipt.logs[0].data;
    let price = U256::from_slice(&data);
    price
}

