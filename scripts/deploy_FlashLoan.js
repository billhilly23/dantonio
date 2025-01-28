const { ethers } = require("hardhat");

async function deployFlashLoan() {
    // Deploy the FlashLoan contract
    const FlashLoan = await ethers.getContractFactory("FlashLoan");
    const flashLoan = await FlashLoan.deploy();

    // Wait for the contract to be deployed
    await flashLoan.deployed();

    console.log(`FlashLoan contract deployed to ${flashLoan.address}`);

    // Verify the contract on Etherscan
    await verifyContract(flashLoan.address);
}

async function verifyContract(contractAddress) {
    // Verify the contract on Etherscan
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: [],
    });
}

deployFlashLoan();
