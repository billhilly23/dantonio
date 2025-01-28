const { ethers } = require("hardhat");

async function deployHft() {
    // Deploy the Hft contract
    const Hft = await ethers.getContractFactory("Hft");
    const hft = await Hft.deploy();

    // Wait for the contract to be deployed
    await hft.deployed();

    console.log(`Hft contract deployed to ${hft.address}`);

    // Verify the contract on Etherscan
    await verifyContract(hft.address);
}

async function verifyContract(contractAddress) {
    // Verify the contract on Etherscan
    await run("verify:verify", {
        address: contractAddress,
        constructorArguments: [],
    });
}

deployHft();
