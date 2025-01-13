// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

import {Script} from "forge-std/Script.sol";
import {ServiceManager} from "../src/ServiceManager.sol";
import {IDelegationManager} from "eigenlayer-contracts/src/contracts/interfaces/IDelegationManager.sol";
import {AVSDirectory} from "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {ISignatureUtils} from "eigenlayer-contracts/src/contracts/interfaces/ISignatureUtils.sol";
import {console} from "forge-std/console.sol";

/// @title DeployServiceManager
/// @notice Script to deploy and configure ServiceManager for EigenLayer AVS
contract DeployServiceManager is Script {
    // --- Constants ---
    /// @notice Address of the EigenLayer AVS Directory contract
    address private constant AVS_DIRECTORY = 0xdAbdB3Cd346B7D5F5779b0B614EdE1CC9DcBA5b7;
    /// @notice Address of the EigenLayer Delegation Manager contract
    address private constant DELEGATION_MANAGER = 0x39053D51B77DC0d36036Fc1fCc8Cb819df8Ef37A;

    // --- State Variables ---
    /// @notice Address that will deploy the contracts
    address private deployer;
    /// @notice Address that will be registered as operator
    address private operator;
    /// @notice Instance of the deployed ServiceManager
    ServiceManager private serviceManager;

    // --- Setup ---
    function setUp() public virtual {
        deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        operator = vm.rememberKey(vm.envUint("OPERATOR_PRIVATE_KEY"));
        vm.label(deployer, "Deployer");
        vm.label(operator, "Operator");
    }

    // --- Main Deployment Function ---
    function run() public {
        _deployServiceManager();
        _registerOperator();
        _registerOperatorToAVS();
    }

    // --- Internal Functions ---
    /// @notice Deploys the ServiceManager contract
    function _deployServiceManager() private {
        vm.startBroadcast(deployer);
        serviceManager = new ServiceManager(AVS_DIRECTORY);
        console.log("ServiceManager deployed at:", address(serviceManager));
        vm.stopBroadcast();
    }

    /// @notice Registers the operator with EigenLayer if not already registered
    function _registerOperator() private {
        IDelegationManager delegationManager = IDelegationManager(DELEGATION_MANAGER);

        // Check if already registered
        if (delegationManager.isOperator(operator)) {
            console.log("Operator already registered");
            return;
        }

        // Prepare operator details
        IDelegationManager.OperatorDetails memory operatorDetails = IDelegationManager.OperatorDetails({
            __deprecated_earningsReceiver: operator,
            delegationApprover: address(0),
            stakerOptOutWindowBlocks: 0
        });

        // Register operator
        vm.startBroadcast(operator);
        delegationManager.registerAsOperator(operatorDetails, "");
        console.log("Successfully registered operator:", operator);
        vm.stopBroadcast();
    }

    /// @notice Registers the operator with the AVS
    function _registerOperatorToAVS() private {
        AVSDirectory avsDirectory = AVSDirectory(AVS_DIRECTORY);
        
        // Prepare registration parameters
        bytes32 salt = keccak256(abi.encodePacked(block.timestamp, operator));
        uint256 expiry = block.timestamp + 1 hours;

        // Calculate registration digest
        bytes32 operatorRegistrationDigestHash = avsDirectory.calculateOperatorAVSRegistrationDigestHash(
            operator,
            address(serviceManager),
            salt,
            expiry
        );

        // Sign the registration
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            vm.envUint("OPERATOR_PRIVATE_KEY"),
            operatorRegistrationDigestHash
        );
        
        // Prepare signature data
        ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature = ISignatureUtils.SignatureWithSaltAndExpiry({
            signature: abi.encodePacked(r, s, v),
            salt: salt,
            expiry: expiry
        });

        // Register to AVS
        vm.startBroadcast(operator);
        serviceManager.registerOperatorToAVS(operator, operatorSignature);
        console.log("Successfully registered operator to AVS");
        vm.stopBroadcast();
    }
}