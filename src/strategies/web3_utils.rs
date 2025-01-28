use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Web3Utils {
  pub web3: Web3,
}

impl Web3Utils {
  pub fn new(web3: Web3) -> Self {
    Web3Utils { web3 }
  }

  pub fn get_balance(&self, address: Address) -> U256 {
    // Get the balance of the specified address
    let balance = self.web3.eth().get_balance(address).unwrap();
    balance
  }

  pub fn get_price(&self, symbol: String) -> U256 {
    // Get the price of the specified symbol
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let response = reqwest::get(url).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let price = json["price"].as_str().unwrap();
    U256::from_dec(price).unwrap()
  }

  pub fn send_transaction(&self, from: Address, to: Address, amount: U256) -> bool {
    // Send a transaction from the specified address to the specified address
    let tx = self.web3.eth().send_transaction(from, to, amount).unwrap();
    tx.is_ok()
  }

  pub fn approve(&self, owner: Address, spender: Address, amount: U256) -> bool {
    // Approve the specified spender to spend the specified amount of tokens
    let tx = self.web3.eth().approve(owner, spender, amount).unwrap();
    tx.is_ok()
  }
}
