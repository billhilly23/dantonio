pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/utils/ReentrancyGuard.sol";
import "https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Router02.sol";

contract Sandwich {
  // Mapping of DEX addresses to token prices
  mapping(address => uint256) public dexPrices;

  // Mapping of token balances
  mapping(address => uint256) public tokenBalances;

  // Reentrancy guard
  ReentrancyGuard public reentrancyGuard;

  // Constructor
  constructor(address[] memory _dexes, address _token) public {
    // Initialize DEX addresses and token
    for (uint256 i = 0; i < _dexes.length; i++) {
      dexPrices[_dexes[i]] = 0;
    }
    token = _token;
  }

  // Sandwich function
  function sandwich(address _dex1, address _dex2) public {
    // Check for price discrepancies
    uint256 price1 = dexPrices[_dex1];
    uint256 price2 = dexPrices[_dex2];

    if (price1 > price2) {
      // Buy on DEX2 and sell on DEX1
      buyOnDex(_dex2, price2);
      sellOnDex(_dex1, price1);
    } else if (price2 > price1) {
      // Buy on DEX1 and sell on DEX2
      buyOnDex(_dex1, price1);
      sellOnDex(_dex2, price2);
    }
  }

  // Get price function
  function getPrice(address _dex) public view returns (uint256) {
    return dexPrices[_dex];
  }

  // Get balance function
  function getBalance(address _token) public view returns (uint256) {
    return tokenBalances[_token];
  }

  // Buy on DEX function
  function buyOnDex(address _dex, uint256 _price) internal {
    // Set the Uniswap V2 Router address
    address uniswapV2Router = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    // Set the WETH address
    address weth = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    // Set the token address
    address token = address(this);

    // Set the amount of ETH to swap
    uint256 amountIn = 1 ether;

    // Set the amount of tokens to receive
    uint256 amountOut = _price;

    // Set the deadline for the transaction
    uint256 deadline = block.timestamp + 15 minutes;

    // Create a path for the swap
    address[] memory path = new address[](2);
    path[0] = weth;
    path[1] = token;

    // Use the Uniswap V2 Router to swap ETH for the token
    IUniswapV2Router02(uniswapV2Router).swapExactTokensForTokensSupportingFeeOnTransferTokens(
      amountIn,
      amountOut,
      path,
      _dex,
      deadline
    );
  }

  // Sell on DEX function
  function sellOnDex(address _dex, uint256 _price) internal {
    // Set the Uniswap V2 Router address
    address uniswapV2Router = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    // Set the WETH address
    address weth = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    // Set the token address
    address token = address(this);

    // Set the amount of tokens to swap
    uint256 amountIn = _price;

    // Set the amount of ETH to receive
    uint256 amountOut = 1 ether;

    // Set the deadline for the transaction
    uint256 deadline = block.timestamp + 15 minutes;

    // Create a path for the swap
    address[] memory path = new address[](2);
    path[0] = token;
    path[1] = weth;

    // Use the Uniswap V2 Router to swap the token for ETH
    IUniswapV2Router02(uniswapV2Router).swapExactTokensForTokensSupportingFeeOnTransferTokens(
      amountIn,
      amountOut,
      path,
      _dex,
      deadline
    );
  }
}
