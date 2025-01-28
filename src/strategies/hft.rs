use std::collections::HashMap;

pub struct Hft {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Hft {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Hft {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn hft(&self) {
    // Implement HFT logic here
  }
}
