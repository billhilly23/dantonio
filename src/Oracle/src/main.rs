use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use web3::types::{Address, U256};
use web3::Web3;

mod price_feed_aggregator;
mod price_feed;

fn main() {
    // Initialize the Web3 provider
    let web3 = Web3::new("http://localhost:8545");

    // Initialize the price feed aggregator
    let price_feed_aggregator = price_feed_aggregator::PriceFeedAggregator::new(web3.clone());

    // Initialize the price feed
    let price_feed = price_feed::PriceFeed::new(web3.clone());

    // Add a token price feed
    price_feed_aggregator.add_token_price_feed("0x1234567890abcdef", "0x1234567890abcdef");

    // Get the token price feed
    let price_feeds = price_feed_aggregator.get_token_price_feed("0x1234567890abcdef");

    // Update the price feed
    price_feed.update_price(U256::from(100));

    // Get the price feed
    let price = price_feed.get_price();

    println!("Price feed: {}", price);
}

