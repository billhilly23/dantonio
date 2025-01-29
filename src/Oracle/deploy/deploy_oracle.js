const { ethers } = require("hardhat");

async function main() {
  // Deploy the PriceFeedAggregator contract
  const PriceFeedAggregator = await ethers.getContractFactory("PriceFeedAggregator");
  const priceFeedAggregator = await PriceFeedAggregator.deploy();

  // Wait for the contract to be deployed
  await priceFeedAggregator.deployed();

  console.log(`PriceFeedAggregator contract deployed to ${priceFeedAggregator.address}`);

  // Deploy the PriceFeed contract
  const PriceFeed = await ethers.getContractFactory("PriceFeed");
  const priceFeed = await PriceFeed.deploy();

  // Wait for the contract to be deployed
  await priceFeed.deployed();

  console.log(`PriceFeed contract deployed to ${priceFeed.address}`);
}

main();

