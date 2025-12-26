// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {SimpleStorage} from "../src/SimpleStorage.sol";

contract DeploySimpleStorage is Script {
    // First Anvil account private key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    // Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
    
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy with initial value of 42
        SimpleStorage simpleStorage = new SimpleStorage(42);
        
        console.log("SimpleStorage deployed at:", address(simpleStorage));
        console.log("Initial value:", simpleStorage.getValue());
        console.log("Deployer address:", msg.sender);
        
        vm.stopBroadcast();
    }
}

