use std::collections::HashMap;

pub struct Arbitrage {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Arbitrage {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Arbitrage {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn arbitrage(&self) {
    // Implement arbitrage logic here
  }
}
