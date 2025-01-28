const { ethers } = require("hardhat");

async function deployFrontRunning() {
    // Deploy the FrontRunning contract
    const FrontRunning = await ethers.getContractFactory("FrontRunning");
    const frontRunning = await FrontRunning.deploy();

    // Wait for the contract to be deployed
    await frontRunning.deployed();

    console.log(`FrontRunning contract deployed to ${frontRunning.address}`);

    // Verify the contract on Etherscan
    await verifyContract(frontRunning.address);
}

async function verifyContract(contractAddress) {
    // Verify the contract on Etherscan
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: [],
    });
}

deployFrontRunning();
