const { ethers } = require("hardhat");

async function deployHft() {
  try {
    // Deploy the Hft contract
    const Hft = await ethers.getContractFactory("Hft");
    const hft = await Hft.deploy();

    // Wait for the contract to be deployed
    await hft.deployed();

    console.log(`Hft contract deployed to ${hft.address}`);

    // Verify the contract on Etherscan
    await verifyContract(hft.address);
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

deployHft();
