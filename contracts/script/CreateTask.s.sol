// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

interface IServiceManager {
    struct Task {
        string contents;
        uint32 taskCreatedBlock;
    }

    function createNewTask(
        string memory codeArweaveUri,
        string memory language,
        uint256 responseCount
    ) external returns (Task memory);
}

contract CreateTask is Script {
    function run() public {
        // Get private key from env and setup the deployer account
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        address contractAddress = 0x427EE58a6c574032085AEB90Dd05dEea6F054930;
        IServiceManager serviceManager = IServiceManager(contractAddress);

        // Create new task with the same parameters as the TypeScript version
        IServiceManager.Task memory task = serviceManager.createNewTask(
            "a3sYP7MtFRUS_u-g30s2sMDGvQyqiI1OehEo7ME4XTA", // codeArweaveUri
            "js", // language
            1 // how many nodes to respond
        );

        console2.log("Task created with contents:", task.contents);
        console2.log("Task created at block:", task.taskCreatedBlock);

        vm.stopBroadcast();
    }
}
