// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleStorage - A simple storage contract for testing Warpscan
/// @notice This contract stores a number and allows it to be retrieved and updated
contract SimpleStorage {
    uint256 public storedValue;
    
    event ValueChanged(uint256 newValue, address indexed changedBy);
    
    constructor(uint256 _initialValue) {
        storedValue = _initialValue;
        emit ValueChanged(_initialValue, msg.sender);
    }
    
    function setValue(uint256 _value) public {
        storedValue = _value;
        emit ValueChanged(_value, msg.sender);
    }
    
    function getValue() public view returns (uint256) {
        return storedValue;
    }
}

