// src/strategies/mod.rs
pub mod arbitrage;
pub mod liquidation;
pub mod flash_loan;
pub mod gas_optimization;

use async_trait::async_trait;
use ethers::types::{Block, H256};
use anyhow::Result;

#[async_trait]
pub trait Strategy: Send + Sync {
    async fn execute(&self, block: &Block<H256>) -> Result<()>;
}

pub use arbitrage::ArbitrageStrategy;
pub use liquidation::LiquidationStrategy;
pub use flash_loan::FlashLoanStrategy;
pub use gas_optimization::GasOptimizationStrategy;

// src/strategies/arbitrage.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    signers::LocalWallet,
};
use std::sync::Arc;
use tracing::{info, error};
use anyhow::Result;

pub struct ArbitrageStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
    dexes: Vec<Dex>,
}

impl ArbitrageStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
        dexes: Vec<Dex>,
    ) -> Self {
        Self {
            provider,
            wallet,
            dexes,
        }
    }

    async fn find_opportunities(&self) -> Result<Vec<ArbitrageOpportunity>> {
        let mut opportunities = Vec::new();

        for pair in &self.get_token_pairs().await? {
            let prices = self.get_prices(pair).await?;
            
            if let Some(opportunity) = self.analyze_prices(pair, &prices) {
                opportunities.push(opportunity);
            }
        }

        Ok(opportunities)
    }

    async fn get_token_pairs(&self) -> Result<Vec<TokenPair>> {
        // Get token pairs from DEXes
        let mut pairs = Vec::new();
        for dex in &self.dexes {
            let pairs_from_dex = dex.get_token_pairs().await?;
            pairs.extend(pairs_from_dex);
        }
        Ok(pairs)
    }

    async fn get_prices(&self, pair: &TokenPair) -> Result<Vec<(String, U256)>> {
        // Get prices from DEXes
        let mut prices = Vec::new();
        for dex in &self.dexes {
            let price = dex.get_price(pair).await?;
            prices.push((dex.name.clone(), price));
        }
        Ok(prices)
    }

    async fn analyze_prices(&self, pair: &TokenPair, prices: &Vec<(String, U256)>) -> Option<ArbitrageOpportunity> {
        // Analyze prices to find arbitrage opportunities
        let mut best_opportunity = None;

        for (dex, price) in prices {
            if let Some(opportunity) = self.calculate_arbitrage(pair, dex, price) {
                if best_opportunity.is_none() || opportunity.profit_percentage > best_opportunity.unwrap().profit_percentage {
                    best_opportunity = Some(opportunity);
                }
            }
        }

        best_opportunity
    }

    async fn calculate_arbitrage(&self, pair: &TokenPair, dex: &str, price: &U256) -> Option<ArbitrageOpportunity> {
        // Calculate arbitrage opportunity
        let buy_dex = "Uniswap V2";
        let sell_dex = "Sushiswap";

        let profit_percentage = (price - self.get_uniswap_v2_price(pair).await?) as f64 / self.get_uniswap_v2_price(pair).await? as f64 * 100.0;

        if profit_percentage > 0.0 {
            Some(ArbitrageOpportunity {
                buy_dex: buy_dex.to_string(),
                sell_dex: sell_dex.to_string(),
                profit_percentage,
                token_pair: (pair.token0, pair.token1),
            })
        } else {
            None
        }
    }

    async fn execute_arbitrage(&self, opportunity: &ArbitrageOpportunity) -> Result<()> {
        // Execute arbitrage trade
        let buy_dex = self.get_dex_by_name(&opportunity.buy_dex).await?;
        let sell_dex = self.get_dex_by_name(&opportunity.sell_dex).await?;

        let buy_tx = buy_dex.method("swapExactTokensForTokens", vec![opportunity.token_pair.0, opportunity.token_pair.1]).await?;
        let sell_tx = sell_dex.method("swapExactTokensForTokens", vec![opportunity.token_pair.0, opportunity.token_pair.1]).await?;

        buy_tx.send().await?;
        sell_tx.send().await?;

        Ok(())
    }
}

#[async_trait]
impl Strategy for ArbitrageStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        let opportunities = self.find_opportunities().await?;

        for opportunity in opportunities {
            self.execute_arbitrage(&opportunity).await?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub buy_dex: String,
    pub sell_dex: String,
    pub profit_percentage: f64,
    pub token_pair: (Address, Address),
}

#[derive(Debug)]
pub struct TokenPair {
    pub token0: Address,
    pub token1: Address,
}

// src/strategies/liquidation.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    signers::LocalWallet,
};
use std::sync::Arc;
use tracing::{info, error};
use anyhow::Result;

pub struct LiquidationStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
    lending_pools: Vec<LendingPool>,
}

impl LiquidationStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
        lending_pools: Vec<LendingPool>,
    ) -> Self {
        Self {
            provider,
            wallet,
            lending_pools,
        }
    }

    async fn find_opportunities(&self) -> Result<Vec<LiquidationOpportunity>> {
        // Find liquidation opportunities
        let mut opportunities = Vec::new();

        for pool in &self.lending_pools {
            let users = pool.get_users_with_borrows().await?;
            for user in users {
                let health_factor = pool.get_health_factor(user).await?;
                if health_factor < 1.0 {
                    let collateral = pool.get_collateral(user).await?;
                    let debt = pool.get_debt(user).await?;

                    opportunities.push(LiquidationOpportunity {
                        user,
                        collateral,
                        debt,
                        health_factor,
                    });
                }
            }
        }

        Ok(opportunities)
    }

    async fn execute_liquidation(&self, opportunity: &LiquidationOpportunity) -> Result<()> {
        // Execute liquidation
        let pool = self.get_lending_pool_by_address(&opportunity.collateral).await?;
        let tx = pool.method("liquidate", vec![opportunity.user, opportunity.collateral, opportunity.debt]).await?;
        tx.send().await?;

        Ok(())
    }
}

#[async_trait]
impl Strategy for LiquidationStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        let opportunities = self.find_opportunities().await?;

        for opportunity in opportunities {
            self.execute_liquidation(&opportunity).await?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct LiquidationOpportunity {
    pub user: Address,
    pub collateral: U256,
    pub debt: U256,
    pub health_factor: f64,
}

// src/strategies/flash_loan.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    signers::LocalWallet,
};
use std::sync::Arc;
use tracing::{info, error};
use anyhow::Result;

pub struct FlashLoanStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
    flash_loan_contract: Address,
}

impl FlashLoanStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
        flash_loan_contract: Address,
    ) -> Self {
        Self {
            provider,
            wallet,
            flash_loan_contract,
        }
    }

    async fn find_opportunities(&self) -> Result<Vec<FlashLoanOpportunity>> {
        // Find flash loan opportunities
        let mut opportunities = Vec::new();

        let markets = self.get_markets().await?;

        for market in markets {
            let interest_rate = self.get_interest_rate(market).await?;
            if interest_rate > 0.0 {
                opportunities.push(FlashLoanOpportunity {
                    market,
                    interest_rate,
                });
            }
        }

        Ok(opportunities)
    }

    async fn get_markets(&self) -> Result<Vec<Address>> {
        // Get markets
        let contract = self.provider.get_contract(self.flash_loan_contract);
        let markets = contract.call("getMarkets", vec![]).await?;

        Ok(markets)
    }

    async fn get_interest_rate(&self, market: Address) -> Result<f64> {
        // Get interest rate
        let contract = self.provider.get_contract(self.flash_loan_contract);
        let interest_rate = contract.call("getInterestRate", vec![market]).await?;

        Ok(interest_rate)
    }

    async fn execute_flash_loan(&self, opportunity: &FlashLoanOpportunity) -> Result<()> {
        // Execute flash loan
        let contract = self.provider.get_contract(self.flash_loan_contract);
        let tx = contract.method("flashLoan", vec![opportunity.market]).await?;
        tx.send().await?;

        Ok(())
    }
}

#[async_trait]
impl Strategy for FlashLoanStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        let opportunities = self.find_opportunities().await?;

        for opportunity in opportunities {
            self.execute_flash_loan(&opportunity).await?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FlashLoanOpportunity {
    pub market: Address,
    pub interest_rate: f64,
}

// src/strategies/gas_optimization.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    signers::LocalWallet,
};
use std::sync::Arc;
use tracing::{info, error};
use anyhow::Result;

pub struct GasOptimizationStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
}

impl GasOptimizationStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
    ) -> Self {
        Self {
            provider,
            wallet,
        }
    }

    async fn find_opportunities(&self) -> Result<Vec<GasOptimizationOpportunity>> {
        // Find gas optimization opportunities
        let mut opportunities = Vec::new();

        let transactions = self.get_transactions().await?;

        for transaction in transactions {
            let gas_price = self.get_gas_price(transaction).await?;
            if gas_price > 0.0 {
                opportunities.push(GasOptimizationOpportunity {
                    transaction,
                    gas_price,
                });
            }
        }

        Ok(opportunities)
    }

    async fn get_transactions(&self) -> Result<Vec<Transaction>> {
        // Get transactions
        let contract = self.provider.get_contract(Address::zero());
        let transactions = contract.call("getTransactions", vec![]).await?;

        Ok(transactions)
    }

    async fn get_gas_price(&self, transaction: Transaction) -> Result<f64> {
        // Get gas price
        let contract = self.provider.get_contract(Address::zero());
        let gas_price = contract.call("getGasPrice", vec![transaction]).await?;

        Ok(gas_price)
    }

    async fn execute_gas_optimization(&self, opportunity: &GasOptimizationOpportunity) -> Result<()> {
        // Execute gas optimization
        let contract = self.provider.get_contract(Address::zero());
        let tx = contract.method("optimizeGas", vec![opportunity.transaction]).await?;
        tx.send().await?;

        Ok(())
    }
}

#[async_trait]
impl Strategy for GasOptimizationStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        let opportunities = self.find_opportunities().await?;

        for opportunity in opportunities {
            self.execute_gas_optimization(&opportunity).await?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GasOptimizationOpportunity {
    pub transaction: Transaction,
    pub gas_price: f64,
}


