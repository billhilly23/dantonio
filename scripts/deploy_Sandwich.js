const { ethers } = require("hardhat");

async function deploySandwich() {
    // Deploy the Sandwich contract
    const Sandwich = await ethers.getContractFactory("Sandwich");
    const sandwich = await Sandwich.deploy();

    // Wait for the contract to be deployed
    await sandwich.deployed();

    console.log(`Sandwich contract deployed to ${sandwich.address}`);

    // Verify the contract on Etherscan
    await verifyContract(sandwich.address);
}

async function verifyContract(contractAddress) {
    // Verify the contract on Etherscan
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: [],
    });
}

deploySandwich();
