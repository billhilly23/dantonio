// src/modules/monitoring.rs
use async_trait::async_trait;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    contract::Contract,
    signers::LocalWallet,
    types::{U256, Address, TransactionRequest, Log, Filter, Bytes},
};
use std::{sync::Arc, collections::HashMap};
use tokio::sync::{RwLock, Semaphore};
use tracing::{info, error};
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub target_contracts: Vec<MonitoredContract>,
    pub event_topics: Vec<String>,
    pub max_concurrent_monitors: usize,
    pub block_poll_interval: u64,
    pub max_gas_price: U256,
}

#[derive(Debug, Clone)]
pub struct MonitoredContract {
    pub address: Address,
    pub name: String,
    pub abi_path: String,
}

pub struct MonitoringStrategy {
    provider: Arc<Provider<Ws>>,
    wallet: LocalWallet,
    config: MonitoringConfig,
    contracts: HashMap<Address, Arc<Contract<Provider<Ws>>>>,
    nonce: Arc<RwLock<U256>>,
    monitor_semaphore: Arc<Semaphore>,
}

#[derive(Debug, Clone)]
struct MonitoringEvent {
    contract: Address,
    event_topic: String,
    block_number: U64,
    transaction_hash: H256,
    log: Log,
}

impl MonitoringStrategy {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        wallet: LocalWallet,
        config: MonitoringConfig,
    ) -> Result<Self> {
        let nonce = provider
            .get_transaction_count(wallet.address(), None)
            .await
            .context("Failed to get initial nonce")?;

        // Load contracts with their ABIs
        let mut contracts = HashMap::new();
        for contract_config in &config.target_contracts {
            let abi = std::fs::read(contract_config.abi_path.clone())
                .context("Failed to read contract ABI")?;
            
            let contract = Contract::new(
                contract_config.address, 
                abi, 
                provider.clone()
            );

            contracts.insert(contract_config.address, Arc::new(contract));
        }

        Ok(Self {
            provider,
            wallet,
            config: config.clone(),
            contracts,
            nonce: Arc::new(RwLock::new(nonce)),
            monitor_semaphore: Arc::new(Semaphore::new(config.max_concurrent_monitors)),
        })
    }

    async fn scan_events(&self) -> Result<Vec<MonitoringEvent>> {
        let mut events = Vec::new();
        
        for (contract_address, contract) in &self.contracts {
            for event_topic in &self.config.event_topics {
                let event_logs = self.fetch_contract_events(contract_address, event_topic).await?;
                events.extend(event_logs);
            }
        }

        Ok(events)
    }

    async fn fetch_contract_events(
        &self, 
        contract_address: &Address, 
        event_topic: &String
    ) -> Result<Vec<MonitoringEvent>> {
        // Create a filter for the specific event topic
        let latest_block = self.provider.get_block_number().await?;
        let from_block = latest_block.saturating_sub(1); // Adjust as needed

        let filter = Filter::new()
            .address(*contract_address)
            .topic0(H256::from_slice(event_topic.as_bytes()))
            .from_block(from_block)
            .to_block(latest_block);

        // Fetch logs matching the filter
        let logs = self.provider.get_logs(&filter).await?;

        // Convert logs to MonitoringEvents
        let events = logs.into_iter().map(|log| MonitoringEvent {
            contract: *contract_address,
            event_topic: event_topic.clone(),
            block_number: log.block_number.unwrap_or(latest_block),
            transaction_hash: log.transaction_hash.unwrap_or_default(),
            log,
        }).collect();

        Ok(events)
    }

    async fn process_event(&self, event: &MonitoringEvent) -> Result<()> {
        // Decode event data using the contract's ABI
        let contract = self.contracts.get(&event.contract)
            .ok_or_else(|| anyhow!("Contract not found"))?;

        // Example of decoding an event (customize based on your specific event)
        let decoded_event = contract.decode_event::<Vec<ethabi::Token>>("EventName", &event.log)?;

        // Perform actions based on event data
        info!("Processed event from contract {:?}: {:?}", event.contract, decoded_event);

        Ok(())
    }

    async fn execute_monitor_action(&self, event: &MonitoringEvent) -> Result<TransactionReceipt> {
        let _permit = self.monitor_semaphore.acquire().await?;
        
        let mut retries = 0;
        while retries < self.config.max_retries {
            match self.try_execute_monitor_action(event).await {
                Ok(receipt) => return Ok(receipt),
                Err(e) => {
                    warn!("Monitor action attempt {} failed: {:?}", retries + 1, e);
                    retries += 1;
                    if retries < self.config.max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(anyhow!("Max retries exceeded for monitor action"))
    }

    async fn try_execute_monitor_action(&self, event: &MonitoringEvent) -> Result<TransactionReceipt> {
        // Create transaction request for monitor action
        let tx = self.create_monitor_tx(event).await?;
        self.send_transaction(tx).await
    }

    async fn create_monitor_tx(&self, event: &MonitoringEvent) -> Result<TransactionRequest> {
        let gas_price = self.provider.get_gas_price().await?;
        if gas_price > self.config.max_gas_price {
            return Err(anyhow!("Gas price too high"));
        }

        let gas_limit = self.estimate_gas(event).await?;

        Ok(TransactionRequest {
            to: Some(event.contract),
            value: Some(U256::zero()),
            gas_price: Some(gas_price),
            gas: Some(gas_limit),
            data: Some(self.build_monitor_data(event)?),
            nonce: None,
            ..Default::default()
        })
    }

    async fn send_transaction(&self, tx: TransactionRequest) -> Result<TransactionReceipt> {
        let mut nonce = self.nonce.write().await;
        let mut tx = tx;
        tx.nonce = Some(*nonce);
        *nonce += 1.into();

        let signed_tx = tx.sign(&self.wallet).await?;
        let pending_tx = self.provider.send_raw_transaction(signed_tx).await?;
        
        let receipt = pending_tx.await?;
        
        if receipt.status.unwrap_or_default() == 0.into() {
            return Err(anyhow!("Transaction failed"));
        }

        Ok(receipt)
    }

    async fn estimate_gas(&self, event: &MonitoringEvent) -> Result<U256> {
        // Use contract's gas estimation method
        let contract = self.contracts.get(&event.contract)
            .ok_or_else(|| anyhow!("Contract not found"))?;

        // Example gas estimation (customize based on your specific contract method)
        let gas_estimate: U256 = contract
            .method::<_, U256>("estimateGas", ())? // Replace with actual method
            .call()
            .await?;

        // Add buffer to gas estimate
        Ok(gas_estimate * 120 / 100) // 20% buffer
    }

    fn build_monitor_data(&self, event: &MonitoringEvent) -> Result<Bytes> {
        // Build transaction data based on the event
        let contract = self.contracts.get(&event.contract)
            .ok_or_else(|| anyhow!("Contract not found"))?;

        // Example of building monitor data (customize based on your specific requirements)
        let method_call = contract.method::<_, Bytes>(
            "processMonitoredEvent", 
            (event.transaction_hash, event.block_number)
        )?;

        Ok(method_call.calldata()?)
    }
}

#[async_trait]
impl Strategy for MonitoringStrategy {
    async fn execute(&self, _block: &Block<H256>) -> Result<()> {
        // Scan for events
        let events = self.scan_events().await?;
        
        for event in events {
            // Process the event
            if let Err(e) = self.process_event(&event).await {
                error!("Event processing failed: {:?}", e);
            }

            // Execute monitor action if needed
            match self.execute_monitor_action(&event).await {
                Ok(receipt) => {
                    info!(
                        "Successfully executed monitor action - Contract: {:?}, Block: {}",
                        event.contract,
                        event.block_number
                    );
                }
                Err(e) => error!("Monitor action execution failed: {:?}", e),
            }
        }

        Ok(())
    }

    async fn validate(&self) -> Result<bool> {
        Ok(self.config.enabled && 
           !self.config.target_contracts.is_empty() &&
           !self.config.event_topics.is_empty())
    }

    fn name(&self) -> &'static str {
        "MonitoringStrategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_scanning() {
        // Implement tests for event scanning logic
    }

    #[tokio::test]
    async fn test_event_processing() {
        // Implement tests for event processing
    }

    #[tokio::test]
    async fn test_gas_estimation() {
        // Implement tests for gas estimation
    }
}

