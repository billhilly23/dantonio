// src/main.rs
use std::sync::Arc;
use ethers::{
    providers::{Provider, Ws},
    signers::LocalWallet,
    types::{Block, H256, U256},
    middleware::SignerMiddleware,
};
use tokio::sync::RwLock;
use anyhow::Result;
use tracing::{info, error};

mod strategies;
mod config;
mod protocols;
mod metrics;

pub struct Bot {
    client: Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>,
    strategies: Vec<Box<dyn Strategy>>,
    config: Arc<RwLock<Config>>,
}

impl Bot {
    pub async fn new(config_path: &str) -> Result<Self> {
        let config = Config::load(config_path)?;
        
        let provider = Arc::new(Provider::<Ws>::connect(&config.network.rpc_url).await?);
        let wallet = LocalWallet::from_bytes(&config.wallet.private_key)?
            .with_chain_id(config.network.chain_id);
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        let strategies = Self::initialize_strategies(client.clone(), &config).await?;

        Ok(Self {
            client,
            strategies,
            config: Arc::new(RwLock::new(config)),
        })
    }

    async fn initialize_strategies(
        client: Arc<SignerMiddleware<Provider<Ws>, LocalWallet>>,
        config: &Config,
    ) -> Result<Vec<Box<dyn Strategy>>> {
        let mut strategies: Vec<Box<dyn Strategy>> = Vec::new();

        // Initialize DEX router and price oracle
        let dex_router = Arc::new(DexRouter::new(client.clone(), &config.protocols.dexes));
        let price_oracle = Arc::new(PriceOracle::new(client.clone(), &config.protocols.oracles));

        // Initialize lending protocols
        let lending_protocols = Arc::new(LendingProtocolManager::new(
            client.clone(),
            &config.protocols.lending,
        ));

        // Add strategies based on configuration
        if config.strategies.arbitrage.enabled {
            strategies.push(Box::new(ArbitrageStrategy::new(
                client.clone(),
                config.strategies.arbitrage.clone(),
                dex_router.clone(),
                price_oracle.clone(),
            )));
        }

        if config.strategies.liquidation.enabled {
            strategies.push(Box::new(LiquidationStrategy::new(
                client.clone(),
                config.strategies.liquidation.clone(),
                lending_protocols.clone(),
                price_oracle.clone(),
            )));
        }

        if config.strategies.flash_loan.enabled {
            strategies.push(Box::new(FlashLoanStrategy::new(
                client.clone(),
                config.strategies.flash_loan.clone(),
                lending_protocols.clone(),
            )));
        }

        Ok(strategies)
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting MEV bot...");
        
        let mut block_stream = self.client.provider().subscribe_blocks().await?;
        
        while let Some(block) = block_stream.next().await {
            let context = self.create_execution_context(&block).await?;
            
            for strategy in &self.strategies {
                if strategy.validate(&context).await? {
                    let estimated_profit = strategy.estimate_profit(&context).await?;
                    
                    if self.is_profitable(estimated_profit, &context).await? {
                        let strategy_name = strategy.name().to_string();
                        let client = self.client.clone();
                        let strategy = strategy.clone();
                        
                        tokio::spawn(async move {
                            match strategy.execute(&block).await {
                                Ok(_) => {
                                    info!("Successfully executed strategy: {}", strategy_name);
                                    metrics::gauge!(
                                        "strategy_profit",
                                        estimated_profit.as_u128() as f64 / 1e18,
                                        "strategy" => strategy_name
                                    );
                                }
                                Err(e) => {
                                    error!("Strategy execution failed: {:?}", e);
                                    metrics::counter!(
                                        "strategy_errors",
                                        1,
                                        "strategy" => strategy_name
                                    );
                                }
                            }
                        });
                    }
                }
            }
        }

        Ok(())
    }

    async fn create_execution_context(&self, block: &Block<H256>) -> Result<ExecutionContext> {
        let gas_price = self.client.provider().get_gas_price().await?;
        let base_fee = block.base_fee_per_gas.unwrap_or_default();
        
        Ok(ExecutionContext {
            block_number: block.number.unwrap_or_default(),
            timestamp: block.timestamp,
            gas_price,
            base_fee,
            priority_fee: self.config.read().await.gas.priority_fee,
        })
    }

    async fn is_profitable(&self, profit: U256, context: &ExecutionContext) -> Result<bool> {
        let config = self.config.read().await;
        let gas_cost = context.gas_price.mul(U256::from(500000)); // Estimated gas usage
        
        Ok(profit > (gas_cost + config.min_profit_threshold))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Create and run bot
    let bot = Bot::new("config.yaml").await?;
    bot.run().await?;

    Ok(())
}




