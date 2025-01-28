use std::collections::HashMap;

pub struct Dashboard {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Dashboard {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Dashboard {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn dashboard(&self) {
    // Implement dashboard logic here
  }
}
