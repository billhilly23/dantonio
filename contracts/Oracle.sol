pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/utils/ReentrancyGuard.sol";
import "https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Router02.sol";

contract Oracle {
    // Mapping of token addresses to their corresponding prices
    mapping(address => uint256) public tokenPrices;

    // Mapping of token addresses to their corresponding price feeds
    mapping(address => address[]) public tokenPriceFeeds;

    // Reentrancy guard
    ReentrancyGuard public reentrancyGuard;

    // Constructor
    constructor() public {
        // Initialize the reentrancy guard
        reentrancyGuard = new ReentrancyGuard();
    }

    // Function to update the price of a token
    function updateTokenPrice(address tokenAddress, uint256 price) public {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Check if the price is valid
        require(price > 0, "Invalid price");

        // Update the token price
        tokenPrices[tokenAddress] = price;
    }

    // Function to add a price feed for a token
    function addTokenPriceFeed(address tokenAddress, address priceFeedAddress) public {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Check if the price feed address is valid
        require(priceFeedAddress != address(0), "Invalid price feed address");

        // Add the price feed to the token's price feeds
        tokenPriceFeeds[tokenAddress].push(priceFeedAddress);
    }

    // Function to remove a price feed for a token
    function removeTokenPriceFeed(address tokenAddress, address priceFeedAddress) public {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Check if the price feed address is valid
        require(priceFeedAddress != address(0), "Invalid price feed address");

        // Remove the price feed from the token's price feeds
        for (uint256 i = 0; i < tokenPriceFeeds[tokenAddress].length; i++) {
            if (tokenPriceFeeds[tokenAddress][i] == priceFeedAddress) {
                tokenPriceFeeds[tokenAddress][i] = tokenPriceFeeds[tokenAddress][tokenPriceFeeds[tokenAddress].length - 1];
                tokenPriceFeeds[tokenAddress].pop();
                break;
            }
        }
    }

    // Function to get the price of a token
    function getTokenPrice(address tokenAddress) public view returns (uint256) {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Return the token price
        return tokenPrices[tokenAddress];
    }

    // Function to get the price feeds for a token
    function getTokenPriceFeeds(address tokenAddress) public view returns (address[] memory) {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Return the token price feeds
        return tokenPriceFeeds[tokenAddress];
    }
}
