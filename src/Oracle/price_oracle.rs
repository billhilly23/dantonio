// src/oracle/price_oracle.rs
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    types::{Address, U256},
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use anyhow::Result;

pub struct PriceOracle {
    provider: Arc<Provider<Ws>>,
    chainlink_feeds: HashMap<Address, Address>,
    cache: Arc<RwLock<HashMap<Address, (U256, u64)>>>,
    cache_duration: u64,
}

impl PriceOracle {
    pub fn new(
        provider: Arc<Provider<Ws>>,
        chainlink_feeds: HashMap<Address, Address>,
        cache_duration: u64,
    ) -> Self {
        Self {
            provider,
            chainlink_feeds,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration,
        }
    }

    pub async fn get_price(&self, token: Address) -> Result<U256> {
        // Check cache first
        if let Some((price, timestamp)) = self.cache.read().await.get(&token) {
            if self.is_cache_valid(*timestamp) {
                return Ok(*price);
            }
        }

        let price = self.fetch_chainlink_price(token).await?;
        
        // Update cache
        self.cache.write().await.insert(token, (price, self.current_timestamp()?));
        
        Ok(price)
    }

    async fn fetch_chainlink_price(&self, token: Address) -> Result<U256> {
        let feed_address = self.chainlink_feeds.get(&token)
            .ok_or_else(|| anyhow::anyhow!("No price feed for token"))?;

        let feed = ChainlinkFeed::new(*feed_address, self.provider.clone());
        let (_, price, _, updated_at, _) = feed.latest_round_data().await?;

        // Check for stale prices
        if self.current_timestamp()? - updated_at > 3600 {
            return Err(anyhow::anyhow!("Price data is stale"));
        }

        Ok(price)
    }

    fn is_cache_valid(&self, timestamp: u64) -> bool {
        match self.current_timestamp() {
            Ok(current_time) => current_time - timestamp <= self.cache_duration,
            Err(_) => false,
        }
    }

    fn current_timestamp(&self) -> Result<u64> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs())
    }
}

// src/lending/lending_pool.rs
#[async_trait]
pub trait LendingPool {
    async fn get_user_account_data(&self, user: Address) -> Result<UserAccountData>;
    async fn liquidate_position(&self, params: LiquidationParams) -> Result<()>;
}

#[derive(Debug)]
pub struct UserAccountData {
    pub collateral_value: U256,
    pub debt_value: U256,
    pub health_factor: U256,
    pub liquidation_threshold: U256,
}

#[derive(Debug)]
pub struct LiquidationParams {
    pub user: Address,
    pub collateral_token: Address,
    pub debt_token: Address,
    pub debt_to_cover: U256,
    pub receive_underlying: bool,
}

pub struct AaveLendingPool {
    contract: Contract<Provider<Ws>>,
    provider: Arc<Provider<Ws>>,
}

impl AaveLendingPool {
    pub fn new(address: Address, provider: Arc<Provider<Ws>>) -> Self {
        let contract = Contract::new(
            address,
            include_bytes!("../abi/AaveLendingPool.json"),
            provider.clone(),
        );
        
        Self { contract, provider }
    }
}

#[async_trait]
impl LendingPool for AaveLendingPool {
    async fn get_user_account_data(&self, user: Address) -> Result<UserAccountData> {
        let result = self.contract
            .method("getUserAccountData", user)?
            .call()
            .await?;

        Ok(UserAccountData {
            collateral_value: result.0,
            debt_value: result.1,
            health_factor: result.2,
            liquidation_threshold: result.3,
        })
    }

    async fn liquidate_position(&self, params: LiquidationParams) -> Result<()> {
        let tx = self.contract
            .method(
                "liquidationCall",
                (
                    params.collateral_token,
                    params.debt_token,
                    params.user,
                    params.debt_to_cover,
                    params.receive_underlying,
                )
            )?
            .send()
            .await?;

        tx.await?;
        Ok(())
    }
}

