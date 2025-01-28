use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

pub struct Arbitrage {
    pub dexes: HashMap<String, String>,
    pub token: String,
    pub gas_price: u64,
    pub private_key: String,
    pub web3: Web3,
}

impl Arbitrage {
    pub fn new(dexes: HashMap<String, String>, token: String, gas_price: u64, private_key: String, web3: Web3) -> Self {
        Arbitrage {
            dexes,
            token,
            gas_price,
            private_key,
            web3,
        }
    }

    pub fn arbitrage(&self) {
        // Identify profitable trades
        let mut opportunities = Vec::new();
        for (dex1, price1) in &self.dexes {
            for (dex2, price2) in &self.dexes {
                if price1 > price2 {
                    // Buy on DEX2 and sell on DEX1
                    opportunities.push((dex2.clone(), dex1.clone()));
                } else if price2 > price1 {
                    // Buy on DEX1 and sell on DEX2
                    opportunities.push((dex1.clone(), dex2.clone()));
                }
            }
        }

        // Execute trades
        for (dex1, dex2) in opportunities {
            self.buy_on_dex(dex2.clone());
            self.sell_on_dex(dex1.clone());
        }
    }

    pub fn buy_on_dex(&self, dex: String) {
        // Set the Uniswap V2 Router address
        let uniswap_v2_router = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";

        // Set the WETH address
        let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

        // Set the token address
        let token = self.token.clone();

        // Set the amount of ETH to swap
        let amount_in = 1 ether;

        // Set the amount of tokens to receive
        let amount_out = self.get_price(dex.clone());

        // Set the deadline for the transaction
        let deadline = block.timestamp + 15 minutes;

        // Create a path for the swap
        let path = vec![weth, token];

        // Use the Uniswap V2 Router to swap ETH for the token
        self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.private_key.clone(),
                to: uniswap_v2_router,
                value: amount_in,
                gas: self.gas_price,
                gas_price: self.gas_price,
                data: encode_swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                    amount_in,
                    amount_out,
                    path,
                    deadline,
                ),
            },
        );
    }

    pub fn sell_on_dex(&self, dex: String) {
        // Set the Uniswap V2 Router address
        let uniswap_v2_router = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";

        // Set the WETH address
        let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

        // Set the token address
        let token = self.token.clone();

        // Set the amount of tokens to swap
        let amount_in = self.get_price(dex.clone());

        // Set the amount of ETH to receive
        let amount_out = 1 ether;

        // Set the deadline for the transaction
        let deadline = block.timestamp + 15 minutes;

        // Create a path for the swap
        let path = vec![token, weth];

        // Use the Uniswap V2 Router to swap the token for ETH
        self.web3.eth().send_transaction(
            TransactionRequest {
                from: self.private_key.clone(),
                to: uniswap_v2_router,
                value: amount_in,
                gas: self.gas_price,
                gas_price: self.gas_price,
                data: encode_swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
                    amount_in,
                    amount_out,
                    path,
                    deadline,
                ),
            },
        );
    }

    pub fn get_price(&self, dex: String) -> U256 {
        // Get the price of the token on the specified DEX
        let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", dex);
        let response = reqwest::get(url).unwrap();
        let json: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
        let price = json["price"].as_str().unwrap();
        U256::from_dec(price).unwrap()
    }
}

