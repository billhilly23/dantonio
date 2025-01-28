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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dashboard() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let dashboard = dashboard::Dashboard::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(dashboard.dashboard());
  }

  #[test]
  fn test_frontrunning() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let frontrunning = frontrunning::Frontrunning::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(frontrunning.frontrunning());
  }

  #[test]
  fn test_hft() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let hft = hft::Hft::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(hft.hft());
  }

  #[test]
  fn test_liquidation() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let liquidation = liquidation::Liquidation::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(liquidation.liquidation());
  }

  #[test]
  fn test_sandwich() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let sandwich = sandwich::Sandwich::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(sandwich.sandwich());
  }

  #[test]
  fn test_flashloan() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let flashloan = flashloan::Flashloan::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(flashloan.flashloan());
  }

  #[test]
  fn test_arbitrage() {
    let mut dexes = HashMap::new();
    dexes.insert("0x1234567890abcdef".to_string(), "0x1234567890abcdef".to_string());
    dexes.insert("0x234567890abcdef1".to_string(), "0x234567890abcdef1".to_string());

    let token = "0x1234567890abcdef".to_string();
    let gas_price = 20e9;
    let private_key = "0x1234567890abcdef".to_string();

    let arbitrage = arbitrage::Arbitrage::new(dexes.clone(), token.clone(), gas_price, private_key.clone());
    assert!(arbitrage.arbitrage());
  }
}
