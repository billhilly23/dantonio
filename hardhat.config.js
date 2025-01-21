require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-etherscan");
require("hardhat-gas-reporter");
require("dotenv").config();

// Load environment variables from .env file
const { INFURA_PROJECT_ID, DEPLOYER_PRIVATE_KEY, ETHERSCAN_API_KEY, COINMARKETCAP_API_KEY } = process.env;

// Ensure required environment variables are set
if (!INFURA_PROJECT_ID) {
  throw new Error("INFURA_PROJECT_ID is missing in the .env file");
}

if (!DEPLOYER_PRIVATE_KEY) {
  throw new Error("DEPLOYER_PRIVATE_KEY is missing in the .env file");
}

// Set up gas price strategies
const gasPriceStrategies = {
  rinkeby: 20e9, // Set gas price to 20 Gwei for Rinkeby testnet
  mainnet: 20e9, // Set gas price to 20 Gwei for Ethereum mainnet
  kovan: 20e9, // Set gas price to 20 Gwei for Kovan testnet
};

// Set up contract verification settings
const verificationSettings = {
  etherscan: {
    apiKey: ETHERSCAN_API_KEY,
  },
};

// Additional recommendations
const additionalRecommendations = {
  // Use a secure method to store sensitive data
  secureSensitiveData: true,
  // Monitor gas costs
  monitorGasCosts: true,
  // Test and validate
  testAndValidate: true,
};

module.exports = {
  // Network configurations
  networks: {
    // Hardhat local network
    hardhat: {
      chainId: 1337,
    },
    // Rinkeby testnet
    rinkeby: {
      url: `https://rinkeby.infura.io/v3/${INFURA_PROJECT_ID}`, // Uses Infura
      accounts: [`0x${DEPLOYER_PRIVATE_KEY}`],  // Deployer's private key
      gasPrice: gasPriceStrategies.rinkeby, // Set gas price for Rinkeby testnet
    },
    // Ethereum mainnet
    mainnet: {
      url: `https://mainnet.infura.io/v3/${INFURA_PROJECT_ID}`, // Uses Infura
      accounts: [`0x${DEPLOYER_PRIVATE_KEY}`],  // Deployer's private key
      gasPrice: gasPriceStrategies.mainnet, // Set gas price for Ethereum mainnet
    },
    // Kovan testnet
    kovan: {
      url: `https://kovan.infura.io/v3/${INFURA_PROJECT_ID}`, // Uses Infura
      accounts: [`0x${DEPLOYER_PRIVATE_KEY}`],  // Deployer's private key
      gasPrice: gasPriceStrategies.kovan, // Set gas price for Kovan testnet
    },
  },

  // Solidity compiler settings
  solidity: {
    version: "0.8.10",  // Updated Solidity version
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,  // Optimization settings to reduce gas costs
      },
    },
  },

  // Path settings (optional)
  paths: {
    sources: "./contracts",            // Default: './contracts'
    tests: "./test",                   // Default: './test'
    cache: "./cache",                  // Default: './cache'
    artifacts: "./artifacts",          // Default: './artifacts'
  },

  // Etherscan API key (optional)
  etherscan: verificationSettings.etherscan,

  // Gas reporter configuration
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,  // Enable gas reporting if REPORT_GAS is set in .env
    currency: 'USD',  // Currency for gas cost reports
    coinmarketcap: COINMARKETCAP_API_KEY,  // Required if you want to fetch the latest gas prices
  },

  // Additional recommendations
  additionalRecommendations: additionalRecommendations,
};

