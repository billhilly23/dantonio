// src/strategies/sandwich.rs
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Sandwich {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
  pub web3: Web3,
}

impl Sandwich {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String, web3: Web3) -> Self {
    Sandwich {
      dexes,
      token,
      gas_price,
      private_key,
      web3,
    }
  }

  pub fn sandwich(&self) {
    // Identify sandwich opportunities
    let mut opportunities = Vec::new();
    for (dex1, price1) in &self.dexes {
      for (dex2, price2) in &self.dexes {
        if price1 > price2 {
          // Buy on DEX2 and sell on DEX1
          opportunities.push((dex2.clone(), dex1.clone()));
        } else if price2 > price1 {
          // Buy on DEX1 and sell on DEX2
          opportunities.push((dex1.clone(), dex2.clone()));
        }
      }
    }

    // Calculate profit
    let mut profits = Vec::new();
    for (dex1, dex2) in opportunities {
      let price1 = self.get_price(dex1.clone());
      let price2 = self.get_price(dex2.clone());
      let profit = price1 - price2;
      profits.push((dex1, dex2, profit));
    }

    // Execute trades
    for (dex1, dex2, profit) in profits {
      if profit > 0 {
        self.buy_on_dex(dex2.clone());
        self.sell_on_dex(dex1.clone());
      }
    }
  }

  pub fn get_price(&self, dex: String) -> U256 {
    // Get the price of the token on the specified DEX
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", dex);
    let response = reqwest::get(url).unwrap();
    let json: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let price = json["price"].as_str().unwrap();
    U256::from_dec(price).unwrap()
  }

  pub fn buy_on_dex(&self, dex: String) {
    // Buy the token on the specified DEX
    let url = format!("https://api.binance.com/api/v3/order?symbol={}&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=100", dex);
    let client = reqwest::Client::new();
    let response = client.post(url)
      .header("X-MBX-APIKEY", "YOUR_API_KEY")
      .header("X-MBX-SECRET-KEY", "YOUR_SECRET_KEY")
      .send().unwrap();
    println!("Buy order placed: {}", response.text().unwrap());
  }

  pub fn sell_on_dex(&self, dex: String) {
    // Sell the token on the specified DEX
    let url = format!("https://api.binance.com/api/v3/order?symbol={}&side=SELL&type=LIMIT&timeInForce=GTC&quantity=1&price=100", dex);
    let client = reqwest::Client::new();
    let response = client.post(url)
      .header("X-MBX-APIKEY", "YOUR_API_KEY")
      .header("X-MBX-SECRET-KEY", "YOUR_SECRET_KEY")
      .send().unwrap();
    println!("Sell order placed: {}", response.text().unwrap());
  }
}


