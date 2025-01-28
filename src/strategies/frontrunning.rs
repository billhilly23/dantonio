use std::collections::HashMap;

pub struct Frontrunning {
  pub dexes: HashMap<String, String>,
  pub token: String,
  pub gas_price: u64,
  pub private_key: String,
}

impl Frontrunning {
  pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Frontrunning {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  pub fn frontrunning(&self) {
    // Implement frontrunning logic here
  }
}
