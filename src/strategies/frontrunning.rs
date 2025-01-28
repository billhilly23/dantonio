use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Frontrunning {
  dexes: HashMap<String, String>,
  token: String,
  gas_price: u64,
  private_key: String,
}

impl Frontrunning {
  fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String) -> Self {
    Frontrunning {
      dexes,
      token,
      gas_price,
      private_key,
    }
  }

  fn frontrunning(&self) {
    // Implement frontrunning logic here
  }
}

fn main() {
  let mut dexes = HashMap::new();
  dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
  dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

  let token = "0x1234567890abcdef".to_string();
  let gas_price = 20e9;
  let private_key = "0x1234567890abcdef".to_string();

  let frontrunning = Frontrunning::new(dexes, token, gas_price, private_key);
  frontrunning.frontrunning();
}


