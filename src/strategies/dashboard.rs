use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Dashboard {
  pub web3: Web3,
}

impl Dashboard {
  pub fn new(web3: Web3) -> Self {
    Dashboard { web3 }
  }

  pub fn display_dashboard(&self) {
    // Display the project's dashboard
    println!("Project Dashboard");
    println!("------------------");
    println!("Balance: {}", self.web3.eth().get_balance(Address::from("0x1234567890abcdef")).unwrap());
    println!("Price: {}", self.get_price("BTCUSDT".to_string()));
    println!("Transactions: {}", self.web3.eth().get_transactions(Address::from("0x1234567890abcdef")).unwrap());
  }

  pub fn get_price(&self, symbol: String) -> U256 {
    // Get the price of the specified symbol
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let response = reqwest::get(url).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let price = json["price"].as_str().unwrap();
    U256::from_dec(price).unwrap()
  }
}
