use std::collections::HashMap;

pub struct Flashloan {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Flashloan {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Flashloan {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn flashloan(&self) {
    // Implement flash loan logic here
  }
}
