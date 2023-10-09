// SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

interface IERC20 {
    function mint(address sender, uint256 amount) external returns(bool);
}

contract Faucet {
    constructor() {}

    function drip(
        address[] calldata tokens,
        uint256[] calldata amounts,
        address who
    ) external returns(bool) {
        require(tokens.length == amounts.length, "Invalid lengths");

        uint256 i = 0;
        uint256 length = tokens.length;
        for(i=0; i< length; i++) {
            IERC20(tokens[i]).mint(who, amounts[0]);
        }

        return true;
    }
}