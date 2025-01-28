const { ethers } = require("hardhat");

async function deployFlashLoan() {
  try {
    // Deploy the FlashLoan contract
    const FlashLoan = await ethers.getContractFactory("FlashLoan");
    const flashLoan = await FlashLoan.deploy();

    // Wait for the contract to be deployed
    await flashLoan.deployed();

    console.log(`FlashLoan contract deployed to ${flashLoan.address}`);

    // Verify the contract on Etherscan
    await verifyContract(flashLoan.address);
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

deployFlashLoan();

