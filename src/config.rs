use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub node_url: String,
    pub private_key: String,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub min_profit: f64,
    pub max_slippage: f64,
    pub dex_contracts: DexContracts,
    pub tokens: Tokens,
}

#[derive(Debug, Deserialize)]
pub struct DexContracts {
    pub uniswap_v2_router: String,
    pub uniswap_v3_router: String,
    pub sushiswap_router: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub weth: String,
    pub usdc: String,
    pub dai: String,
    pub usdt: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(Self {
            node_url: env::var("NODE_URL")?,
            private_key: env::var("PRIVATE_KEY")?,
            gas_limit: env::var("GAS_LIMIT")?.parse()?,
            gas_price: env::var("GAS_PRICE")?.parse()?,
            min_profit: env::var("MIN_PROFIT")?.parse()?,
            max_slippage: env::var("MAX_SLIPPAGE")?.parse()?,
            dex_contracts: DexContracts {
                uniswap_v2_router: env::var("UNISWAP_V2_ROUTER")?,
                uniswap_v3_router: env::var("UNISWAP_V3_ROUTER")?,
                sushiswap_router: env::var("SUSHISWAP_ROUTER")?,
            },
            tokens: Tokens {
                weth: env::var("WETH_ADDRESS")?,
                usdc: env::var("USDC_ADDRESS")?,
                dai: env::var("DAI_ADDRESS")?,
                usdt: env::var("USDT_ADDRESS")?,
            },
        })
    }
}

