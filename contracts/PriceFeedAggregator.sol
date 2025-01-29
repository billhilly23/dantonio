pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/utils/ReentrancyGuard.sol";
import "https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Router02.sol";

contract PriceFeedAggregator {
    // Mapping of token addresses to their corresponding price feeds
    mapping(address => address[]) public tokenPriceFeeds;

    // Reentrancy guard
    ReentrancyGuard public reentrancyGuard;

    // Constructor
    constructor() public {
        // Initialize the reentrancy guard
        reentrancyGuard = new ReentrancyGuard();
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

    // Function to get the price feeds for a token
    function getTokenPriceFeeds(address tokenAddress) public view returns (address[] memory) {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Return the token price feeds
        return tokenPriceFeeds[tokenAddress];
    }

    // Function to aggregate prices from multiple sources
    function aggregatePrices(address tokenAddress) public view returns (uint256) {
        // Check if the token address is valid
        require(tokenAddress != address(0), "Invalid token address");

        // Initialize the aggregated price
        uint256 aggregatedPrice = 0;

        // Iterate over the token's price feeds
        for (uint256 i = 0; i < tokenPriceFeeds[tokenAddress].length; i++) {
            // Get the price from the current price feed
            uint256 price = getPriceFromPriceFeed(tokenPriceFeeds[tokenAddress][i]);

            // Add the price to the aggregated price
            aggregatedPrice += price;
        }

        // Return the aggregated price
        return aggregatedPrice / tokenPriceFeeds[tokenAddress].length;
    }

    // Function to get the price from a price feed
    function getPriceFromPriceFeed(address priceFeedAddress) internal view returns (uint256) {
        // Check if the price feed address is valid
        require(priceFeedAddress != address(0), "Invalid price feed address");

        // Get the price from the price feed
        uint256 price = IPriceFeed(priceFeedAddress).getPrice();

        // Return the price
        return price;
    }
}

interface IPriceFeed {
    function getPrice() external view returns (uint256);
}
