// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {SimpleStorage} from "../src/SimpleStorage.sol";

contract CallSimpleStorage is Script {
    // Deployed contract address
    address constant SIMPLE_STORAGE = 0x5FbDB2315678afecb367f032d93F642f64180aa3;
    
    // First Anvil account private key
    // Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
    
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        
        SimpleStorage simpleStorage = SimpleStorage(SIMPLE_STORAGE);
        
        // Get current value
        uint256 currentValue = simpleStorage.getValue();
        console.log("Current value:", currentValue);
        
        // Set a new value (change from 42 to 100)
        uint256 newValue = 100;
        simpleStorage.setValue(newValue);
        console.log("Set new value to:", newValue);
        
        // Verify the value was set
        uint256 updatedValue = simpleStorage.getValue();
        console.log("Updated value:", updatedValue);
        console.log("Contract address:", SIMPLE_STORAGE);
        console.log("Caller address:", msg.sender);
        
        vm.stopBroadcast();
    }
}

