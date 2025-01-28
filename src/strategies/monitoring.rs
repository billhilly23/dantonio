use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Monitoring {
  pub web3: Web3,
}

impl Monitoring {
  pub fn new(web3: Web3) -> Self {
    Monitoring { web3 }
  }

  pub fn monitor_balance(&self, address: Address) {
    // Monitor the balance of the specified address
    let balance = self.web3.eth().get_balance(address).unwrap();
    println!("Balance: {}", balance);
  }

  pub fn monitor_price(&self, symbol: String) {
    // Monitor the price of the specified symbol
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let response = reqwest::get(url).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let price = json["price"].as_str().unwrap();
    println!("Price: {}", price);
  }

  pub fn monitor_transactions(&self, address: Address) {
    // Monitor the transactions of the specified address
    let txs = self.web3.eth().get_transactions(address).unwrap();
    for tx in txs {
      println!("Transaction: {}", tx);
    }
  }
}

