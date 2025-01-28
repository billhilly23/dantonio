const { ethers } = require("hardhat");

async function deploySandwich() {
  try {
    // Deploy the Sandwich contract
    const Sandwich = await ethers.getContractFactory("Sandwich");
    const sandwich = await Sandwich.deploy();

    // Wait for the contract to be deployed
    await sandwich.deployed();

    console.log(`Sandwich contract deployed to ${sandwich.address}`);

    // Verify the contract on Etherscan
    await verifyContract(sandwich.address);
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

deploySandwich();

