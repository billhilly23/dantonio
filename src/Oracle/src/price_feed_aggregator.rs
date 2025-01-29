use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct PriceFeedAggregator {
    pub web3: Web3,
    pub token_address: Address,
    pub price_feed_address: Address,
}

impl PriceFeedAggregator {
    pub fn new(web3: Web3, token_address: Address, price_feed_address: Address) -> Self {
        PriceFeedAggregator {
            web3,
            token_address,
            price_feed_address,
        }
    }

    pub fn add_token_price_feed(&self, token_address: Address, price_feed_address: Address) {
        // Add the price feed to the token's price feeds
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.token_address,
                to: self.price_feed_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_add_token_price_feed(token_address, price_feed_address),
            },
        );
        self.web3.eth().wait_for_transaction_receipt(tx_hash);
    }

    pub fn remove_token_price_feed(&self, token_address: Address, price_feed_address: Address) {
        // Remove the price feed from the token's price feeds
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.token_address,
                to: self.price_feed_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_remove_token_price_feed(token_address, price_feed_address),
            },
        );
        self.web3.eth().wait_for_transaction_receipt(tx_hash);
    }

    pub fn get_token_price_feed(&self, token_address: Address) -> Vec<Address> {
        // Get the token's price feeds
        let tx_hash = self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.token_address,
                to: self.price_feed_address,
                value: U256::from(0),
                gas: U256::from(2000000),
                gas_price: U256::from(20000000000),
                data: encode_get_token_price_feed(token_address),
            },
        );
        let receipt = self.web3.eth().wait_for_transaction_receipt(tx_hash);
        let price_feeds = decode_get_token_price_feed(receipt);
        price_feeds
    }
}

fn encode_add_token_price_feed(token_address: Address, price_feed_address: Address) -> Vec<u8> {
    // Encode the add token price feed function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    data.extend_from_slice(&token_address.to_vec());
    data.extend_from_slice(&price_feed_address.to_vec());
    data
}

fn encode_remove_token_price_feed(token_address: Address, price_feed_address: Address) -> Vec<u8> {
    // Encode the remove token price feed function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]);
    data.extend_from_slice(&token_address.to_vec());
    data.extend_from_slice(&price_feed_address.to_vec());
    data
}

fn encode_get_token_price_feed(token_address: Address) -> Vec<u8> {
    // Encode the get token price feed function call
    let mut data = Vec::new();
    data.extend_from_slice(&[0x00, 0x00, 0x00, 0x02]);
    data.extend_from_slice(&token_address.to_vec());
    data
}

fn decode_get_token_price_feed(receipt: TransactionReceipt) -> Vec<Address> {
    // Decode the get token price feed function call
    let data = receipt.logs[0].data;
    let mut price_feeds = Vec::new();
    for i in (0..data.len()).step_by(20) {
        let price_feed_address = Address::from_slice(&data[i..i + 20]);
        price_feeds.push(price_feed_address);
    }
    price_feeds
}

