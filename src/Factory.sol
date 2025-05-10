// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "./MetaphoricProxy.sol";

contract Factory {
    uint256 version;
    address operator;
    address temp;

    function create(address _owner) external {
        temp = _owner;
        MetaphoricProxy addr = new MetaphoricProxy(_owner);
    }

    function setup() external {
        version = 1;
        operator = temp;
    }
}
