// src/modules/liquidations.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    contract::Contract,
    signers::LocalWallet,
    types::{U256, Address, Transaction, TransactionRequest, Bytes},
};
use std::{sync::Arc, collections::HashMap, time::Duration};
use tokio::sync::{RwLock, Semaphore};
use tracing::{info, error, debug, warn};
use anyhow::{Result, Context};

// ABI for lending protocol interactions
const LENDING_PROTOCOL_ABI: &[u8] = include_bytes!("../abis/lending_protocol.json");

#[derive(Debug, Clone)]
pub struct LiquidationConfig {
    pub enabled: bool,
    pub min_profit: U256,
    pub max_gas_price: U256,
    pub gas_multiplier: f64,
    pub target_protocols: Vec<LendingProtocol>,
    pub max_concurrent_liquidations: usize,
    pub liquidation_threshold: f64, // Health factor below which to liquidate
    pub max_slippage: f64, // Maximum acceptable slippage percentage
    pub retry_delay: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub struct LendingProtocol {
    pub address: Address,
    pub name: String,
    pub liquidation_contract: Address,
    pub price_feed: Address,
}

#[derive(Debug, Clone)]
struct LendingPosition {
    borrower: Address,
    collateral_token: Address,
    debt_token: Address,
    collateral_amount: U256,
    debt_amount: U256,
    health_factor: f64,
    liquidatable_amount: U256,
}

pub struct LiquidationStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
    config: LiquidationConfig,
    nonce: Arc<RwLock<U256>>,
    price_oracle: Arc<Contract<Provider<Ws>>>,
    lending_protocol_contract: Arc<Contract<Provider<Ws>>>,
}

impl LiquidationStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
        config: LiquidationConfig,
    ) -> Result<Self> {
        let nonce = provider
            .get_transaction_count(wallet.address(), None)
            .await
            .context("Failed to get initial nonce")?;

        // Initialize price oracle contract
        let price_oracle_contract = Contract::new(
            config.target_protocols[0].price_feed, 
            include_bytes!("../abis/price_feed.json").to_vec(), 
            provider.clone()
        );

        // Initialize lending protocol contract
        let lending_protocol_contract = Contract::new(
            config.target_protocols[0].address, 
            LENDING_PROTOCOL_ABI.to_vec(), 
            provider.clone()
        );

        Ok(Self {
            provider,
            wallet,
            config: config.clone(),
            nonce: Arc::new(RwLock::new(nonce)),
            price_oracle: Arc::new(price_oracle_contract),
            lending_protocol_contract: Arc::new(lending_protocol_contract),
        })
    }

    async fn fetch_protocol_positions(
        &self, 
        protocol: &LendingProtocol
    ) -> Result<Vec<LendingPosition>> {
        // Fetch all user positions from the lending protocol
        let positions: Vec<(Address, U256, U256)> = self.lending_protocol_contract
            .method::<_, Vec<(Address, U256, U256)>>("getAllUserPositions", ())?
            .call()
            .await?;

        let mut lending_positions = Vec::new();

        for (borrower, collateral_amount, debt_amount) in positions {
            // Fetch detailed position information
            let position_details: (Address, Address, U256) = self.lending_protocol_contract
                .method::<_, (Address, Address, U256)>("getUserPositionDetails", borrower)?
                .call()
                .await?;

            let (collateral_token, debt_token, liquidation_threshold) = position_details;

            // Calculate health factor
            let collateral_price = self.get_token_price(&collateral_token).await?;
            let debt_price = self.get_token_price(&debt_token).await?;

            let health_factor = self.calculate_health_factor(
                collateral_amount, 
                debt_amount, 
                collateral_price, 
                debt_price
            );

            lending_positions.push(LendingPosition {
                borrower,
                collateral_token,
                debt_token,
                collateral_amount,
                debt_amount,
                health_factor,
                liquidatable_amount: debt_amount, // Simplified calculation
            });
        }

        Ok(lending_positions)
    }

    async fn get_token_price(&self, token: &Address) -> Result<U256> {
        // Fetch price from a price oracle (Chainlink, etc.)
        let price: U256 = self.price_oracle
            .method::<_, U256>("getPrice", *token)?
            .call()
            .await?;

        Ok(price)
    }

    fn calculate_health_factor(
        &self, 
        collateral_amount: U256, 
        debt_amount: U256, 
        collateral_price: U256, 
        debt_price: U256
    ) -> f64 {
        // Health factor calculation
        let total_collateral_value = collateral_amount * collateral_price;
        let total_debt_value = debt_amount * debt_price;

        // Simplified health factor calculation
        if total_debt_value == U256::zero() {
            return f64::INFINITY;
        }

        (total_collateral_value.as_u128() as f64) / 
        (total_debt_value.as_u128() as f64)
    }

    async fn calculate_liquidation_profit(&self, position: &LendingPosition) -> Result<U256> {
        // Calculate potential profit from liquidation
        let collateral_price = self.get_token_price(&position.collateral_token).await?;
        let debt_price = self.get_token_price(&position.debt_token).await?;

        // Calculate liquidation bonus
        let liquidation_bonus_percentage = 0.05; // 5% liquidation bonus
        
        let liquidation_value = position.liquidatable_amount * debt_price;
        let bonus_value = U256::from_dec_str(&(
            liquidation_value.as_u128() as f64 * liquidation_bonus_percentage
        ).to_string())?;

        // Subtract estimated gas cost
        let estimated_gas_cost = U256::from(200_000) * self.config.max_gas_price;
        
        let profit = bonus_value.saturating_sub(estimated_gas_cost);

        Ok(profit)
    }

    async fn build_liquidation_data(&self, opportunity: &LiquidationOpportunity) -> Result<Bytes> {
        // Build liquidation transaction data
        let liquidation_call = self.lending_protocol_contract
            .method::<_, Bytes>(
                "liquidate", 
                (
                    opportunity.borrower,
                    opportunity.collateral_token,
                    opportunity.liquidation_amount
                )
            )?;

        Ok(liquidation_call.calldata()?)
    }

    async fn execute_liquidation(&self, opportunity: &LiquidationOpportunity) -> Result<TransactionReceipt> {
        // Prepare liquidation transaction
        let tx = TransactionRequest {
            to: Some(opportunity.protocol.liquidation_contract),
            value: Some(U256::zero()),
            gas_price: Some(self.config.max_gas_price),
            gas: Some(300_000), // Estimated gas limit
            data: Some(self.build_liquidation_data(opportunity).await?),
            ..Default::default()
        };

        // Send transaction
        let signed_tx = tx.sign(&self.wallet).await?;
        let tx_hash = self.provider.send_raw_transaction(signed_tx).await?;
        
        // Wait for transaction receipt
        let receipt = self.provider
            .pending_transaction(tx_hash)
            .await?
            .ok_or_else(|| anyhow!("Transaction not found"))?;

        Ok(receipt)
    }
}

#[derive(Debug, Clone)]
struct LiquidationOpportunity {
    protocol: LendingProtocol,
    borrower: Address,
    collateral_token: Address,
    debt_token: Address,
    liquidation_amount: U256,
    health_factor: f64,
    estimated_profit: U256,
}

#[async_trait]
impl Strategy for LiquidationStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        // Scan for liquidation opportunities across all protocols
        for protocol in &self.config.target_protocols {
            // Fetch positions
            let positions = self.fetch_protocol_positions(protocol).await?;

            // Find liquidatable positions
            for position in positions {
                // Check if position is liquidatable
                if position.health_factor <= self.config.liquidation_threshold {
                    // Calculate potential profit
                    let estimated_profit = self.calculate_liquidation_profit(&position).await?;

                    // Check if profit meets minimum threshold
                    if estimated_profit > self.config.min_profit {
                        let opportunity = LiquidationOpportunity {
                            protocol: protocol.clone(),
                            borrower: position.borrower,
                            collateral_token: position.collateral_token,
                            debt_token: position.debt_token,
                            liquidation_amount: position.liquidatable_amount,
                            health_factor: position.health_factor,
                            estimated_profit,
                        };

                        // Execute liquidation
                        match self.execute_liquidation(&opportunity).await {
                            Ok(receipt) => {
                                info!(
                                    "Liquidation successful - Borrower: {:?}, Profit: {} wei",
                                    opportunity.borrower,
                                    opportunity.estimated_profit
                                );
                            },
                            Err(e) => {
                                error!(
                                    "Liquidation failed - Borrower: {:?}, Error: {:?}",
                                    opportunity.borrower,
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn validate(&self) -> Result<bool> {
        Ok(self.config.enabled && 
           !self.config.target_protocols.is_empty() &&
           self.config.min_profit > U256::zero())
    }

    fn name(&self) -> &'static str {
        "LiquidationStrategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_position_fetching() {
        // Implement test for position fetching logic
    }

    #[tokio::test]
    async fn test_profit_calculation() {
        // Implement test for profit calculation
    }

    #[tokio::test]
    async fn test_liquidation_execution() {
        // Implement test for liquidation execution
    }
}


