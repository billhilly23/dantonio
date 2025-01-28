const { ethers } = require("hardhat");

async function deployArbitrage() {
  try {
    // Deploy the Arbitrage contract
    const Arbitrage = await ethers.getContractFactory("Arbitrage");
    const arbitrage = await Arbitrage.deploy();

    // Wait for the contract to be deployed
    await arbitrage.deployed();

    console.log(`Arbitrage contract deployed to ${arbitrage.address}`);

    // Verify the contract on Etherscan
    await verifyContract(arbitrage.address);
  } catch (error) {
    console.error(error);
  }
}

async function verifyContract(contractAddress) {
  try {
    // Verify the contract on Etherscan
    await run("verify:verify", {
      address: contractAddress,
      constructorArguments: [],
    });
  } catch (error) {
    console.error(error);
  }
}

deployArbitrage();
