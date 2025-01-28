const { ethers } = require("hardhat");

async function deployLiquidation() {
    // Deploy the Liquidation contract
    const Liquidation = await ethers.getContractFactory("Liquidation");
    const liquidation = await Liquidation.deploy();

    // Wait for the contract to be deployed
    await liquidation.deployed();

    console.log(`Liquidation contract deployed to ${liquidation.address}`);

    // Verify the contract on Etherscan
    await verifyContract(liquidation.address);
}

async function verifyContract(contractAddress) {
    // Verify the contract on Etherscan
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: [],
    });
}

deployLiquidation();
