// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IFactoryCallback {
    function setup() external;
}

contract MetaphoricProxy {
    address immutable public FACTORY;
    address immutable public IMPLEMENTATION;

    constructor(address _impl) {
        FACTORY = msg.sender;
        IMPLEMENTATION = _impl;
        // You would set the slot for the implementation here so explorers understand you.
        (bool rc,) = FACTORY.delegatecall(abi.encodeWithSelector(IFactoryCallback.setup.selector));
        require(rc);
    }

    function directDelegate(address to) internal {
        assembly {
            // Copy msg.data. We take full control of memory in this inline assembly
            // block because it will not return to Solidity code. We overwrite the
            // Solidity scratch pad at memory position 0.
            calldatacopy(0, 0, calldatasize())

            // Call the implementation.
            // out and outsize are 0 because we don't know the size yet.
            let result := delegatecall(gas(), to, 0, calldatasize(), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }

    fallback() external {
        directDelegate(IMPLEMENTATION);
    }
}
