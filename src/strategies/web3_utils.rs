// src/utils/web3_utils.rs
use ethers::{
    prelude::*,
    providers::{Provider, Ws, Http},
    signers::{LocalWallet, Signer},
    types::{Address, U256, TransactionRequest, Bytes},
};
use std::{str::FromStr, sync::Arc, time::Duration};
use anyhow::{Result, Context, anyhow};
use tracing::{info, error};

/// Configuration for Web3 provider connection
#[derive(Debug, Clone)]
pub struct Web3ProviderConfig {
    pub rpc_url: String,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub max_retries: usize,
}

/// Wallet management utilities
pub struct WalletManager;

impl WalletManager {
    /// Create a new wallet from a private key
    pub fn create_wallet_from_private_key(private_key: &str) -> Result<LocalWallet> {
        let wallet = private_key
            .parse::<LocalWallet>()
            .context("Failed to parse private key")?;
        
        Ok(wallet)
    }

    /// Generate a new random wallet
    pub fn generate_random_wallet() -> LocalWallet {
        LocalWallet::new(&mut rand::thread_rng())
    }

    /// Get wallet address
    pub fn get_wallet_address(wallet: &LocalWallet) -> Address {
        wallet.address()
    }
}

/// Provider management utilities
pub struct ProviderManager;

impl ProviderManager {
    /// Create a WebSocket provider with retry and timeout configurations
    pub async fn create_ws_provider(config: &Web3ProviderConfig) -> Result<Arc<Provider<Ws>>> {
        let provider = Provider::<Ws>::connect_with_retries(
            &config.rpc_url, 
            config.max_retries
        )
        .await
        .context("Failed to create WebSocket provider")?;

        // Set timeouts
        let provider = provider
            .interval(Duration::from_millis(2000))
            .timeout(config.request_timeout);

        Ok(Arc::new(provider))
    }

    /// Create an HTTP provider with retry and timeout configurations
    pub async fn create_http_provider(config: &Web3ProviderConfig) -> Result<Arc<Provider<Http>>> {
        let provider = Provider::<Http>::try_from(&config.rpc_url)
            .context("Failed to create HTTP provider")?
            .interval(Duration::from_millis(2000))
            .timeout(config.request_timeout);

        Ok(Arc::new(provider))
    }
}

/// Transaction utility functions
pub struct TransactionUtils;

impl TransactionUtils {
    /// Estimate gas for a transaction
    pub async fn estimate_gas<P: JsonRpcClient>(
        provider: &Provider<P>,
        tx: &TransactionRequest,
    ) -> Result<U256> {
        let gas_estimate = provider
            .estimate_gas(tx)
            .await
            .context("Failed to estimate gas")?;

        // Add 20% buffer to gas estimate
        Ok(gas_estimate * 120 / 100)
    }

    /// Get current gas price
    pub async fn get_gas_price<P: JsonRpcClient>(
        provider: &Provider<P>
    ) -> Result<U256> {
        provider
            .get_gas_price()
            .await
            .context("Failed to fetch gas price")
    }

    /// Build a basic transaction request
    pub fn build_transaction_request(
        from: Address,
        to: Address,
        value: U256,
        data: Option<Bytes>,
    ) -> TransactionRequest {
        TransactionRequest {
            from: Some(from),
            to: Some(to),
            value: Some(value),
            data,
            ..Default::default()
        }
    }

    /// Send a raw transaction
    pub async fn send_raw_transaction<P: JsonRpcClient>(
        provider: &Provider<P>,
        signed_tx: Signature,
    ) -> Result<H256> {
        provider
            .send_raw_transaction(signed_tx)
            .await
            .context("Failed to send raw transaction")
    }
}

/// Contract interaction utilities
pub struct ContractUtils;

impl ContractUtils {
    /// Load a contract from ABI and address
    pub fn load_contract<P: JsonRpcClient>(
        address: Address,
        abi_path: &str,
        provider: Arc<Provider<P>>,
    ) -> Result<Contract<Provider<P>>> {
        let abi = std::fs::read(abi_path)
            .context("Failed to read contract ABI")?;

        let contract = Contract::new(address, abi, provider);
        Ok(contract)
    }

    /// Call a contract view function
    pub async fn call_view_function<P, R, A>(
        contract: &Contract<Provider<P>>,
        method_name: &str,
        args: A,
    ) -> Result<R>
    where
        P: JsonRpcClient,
        R: Decode,
        A: Tokenize,
    {
        contract
            .method::<A, R>(method_name, args)?
            .call()
            .await
            .context("Failed to call view function")
    }

    /// Encode contract method call data
    pub fn encode_method_call<A: Tokenize>(
        contract: &Contract<Provider<impl JsonRpcClient>>,
        method_name: &str,
        args: A,
    ) -> Result<Bytes> {
        contract
            .method::<A, Bytes>(method_name, args)?
            .calldata()
            .context("Failed to encode method call")
    }
}

/// Block and chain utilities
pub struct ChainUtils;

impl ChainUtils {
    /// Get the latest block number
    pub async fn get_latest_block_number<P: JsonRpcClient>(
        provider: &Provider<P>
    ) -> Result<U64> {
        provider
            .get_block_number()
            .await
            .context("Failed to fetch latest block number")
    }

    /// Get block details by number
    pub async fn get_block_by_number<P: JsonRpcClient>(
        provider: &Provider<P>,
        block_number: U64,
    ) -> Result<Block<H256>> {
        provider
            .get_block(block_number)
            .await
            .context("Failed to fetch block details")?
            .ok_or_else(|| anyhow!("Block not found"))
    }

    /// Check if a network is supported
    pub fn is_supported_network(chain_id: u64) -> bool {
        matches!(chain_id, 1 | 5 | 137 | 80001) // Mainnet, Goerli, Polygon, Mumbai
    }
}

/// Error handling and logging utilities
pub struct Web3ErrorHandler;

impl Web3ErrorHandler {
    /// Log and handle Web3 related errors
    pub fn handle_error(error: &anyhow::Error, context: &str) {
        error!(
            "Web3 Error - Context: {}, Error: {}",
            context, error
        );
    }

    /// Retry mechanism for Web3 operations
    pub async fn retry_operation<F, T>(
        operation: F,
        max_retries: usize,
        delay: Duration,
    ) -> Result<T>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>>>>,
    {
        let mut retry_count = 0;

        while retry_count < max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    error!("Retry {} failed: {:?}", retry_count, e);
                    retry_count += 1;
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(anyhow!("Max retries exceeded"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wallet_creation() {
        // Test wallet creation and address generation
    }

    #[tokio::test]
    async fn test_provider_creation() {
        // Test WebSocket and HTTP provider creation
    }

    #[tokio::test]
    async fn test_transaction_utilities() {
        // Test transaction-related utilities
    }

    #[tokio::test]
    async fn test_contract_interactions() {
        // Test contract loading and method calls
    }
}


