use std::collections::HashMap;

pub struct Sandwich {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Sandwich {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Sandwich {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn sandwich(&self) {
    // Implement sandwich logic here
  }
}


