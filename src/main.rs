use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

mod dashboard;
mod frontrunning;
mod hft;
mod liquidation;
mod sandwich;
mod flashloan;
mod arbitrage;

fn main() {
  let mut dexes = HashMap::new();
  dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
  dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

  let token = "0x1234567890abcdef".to_string();
  let gas_price = 20e9;
  let private_key = "0x1234567890abcdef".to_string();

  let dashboard = dashboard::Dashboard::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let frontrunning = frontrunning::Frontrunning::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let hft = hft::Hft::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let liquidation = liquidation::Liquidation::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let sandwich = sandwich::Sandwich::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let flashloan = flashloan::Flashloan::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
  let arbitrage = arbitrage::Arbitrage::new(dexes.clone(), token.clone(), gas_price, private_key.clone());

  dashboard.dashboard();
  frontrunning.frontrunning();
  hft.hft();
  liquidation.liquidation();
  sandwich.sandwich();
  flashloan.flashloan();
  arbitrage.arbitrage();
}


