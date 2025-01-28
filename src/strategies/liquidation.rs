use std::collections::HashMap;

pub struct Liquidation {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Liquidation {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Liquidation {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn liquidation(&self) {
    // Implement liquidation logic here
  }
}

