use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

mod dashboard;

fn main() {
  let mut dexes = HashMap::new();
  dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
  dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

  let token = "0x1234567890abcdef".to_string();
  let gas_price = 20e9;
  let private_key = "0x1234567890abcdef".to_string();

  let dashboard = dashboard::Dashboard::new(dexes, token, gas_price, private_key);
  dashboard.dashboard();
}
