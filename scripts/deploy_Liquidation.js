const { ethers } = require("hardhat");

async function deployLiquidation() {
  try {
    // Deploy the Liquidation contract
    const Liquidation = await ethers.getContractFactory("Liquidation");
    const liquidation = await Liquidation.deploy();

    // Wait for the contract to be deployed
    await liquidation.deployed();

    console.log(`Liquidation contract deployed to ${liquidation.address}`);

    // Verify the contract on Etherscan
    await verifyContract(liquidation.address);
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

deployLiquidation();

