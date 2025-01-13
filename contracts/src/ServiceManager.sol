// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ISignatureUtils} from "eigenlayer-contracts/src/contracts/interfaces/ISignatureUtils.sol";
import {IAVSDirectory} from "eigenlayer-contracts/src/contracts/interfaces/IAVSDirectory.sol";
import {ECDSA} from "solady/utils/ECDSA.sol";

/// @title ServiceManager
/// @notice Manages code execution tasks and operator responses for EigenLayer AVS
/// @dev Handles operator registration, task creation, and response collection
contract ServiceManager {
    using ECDSA for bytes32;

    // --- State Variables ---
    /// @notice Address of the EigenLayer AVS Directory contract
    address public immutable avsDirectory;
    
    /// @notice Current task counter
    uint32 public latestTaskNum;
    
    /// @notice Tracks registered operators
    mapping(address => bool) public operatorRegistered;
    
    /// @notice Stores task hashes for verification
    mapping(uint256 => bytes32) public allTaskHashes;
    
    /// @notice Stores operator responses to tasks
    /// @dev operator => taskIndex => signature
    mapping(address => mapping(uint256 => bytes)) public allTaskResponses;
    
    /// @notice Tracks number of submissions per task
    /// @dev taskIndex => submissionCount
    mapping(uint256 => uint256) public taskSubmissionCount;

    // --- Events ---
    /// @notice Emitted when a new compute task is created
    event ComputeRequestCreated(
        uint256 indexed taskIndex,
        string codeArweaveUri,
        string language,
        uint256 responseCount,
        uint256 taskCreatedBlock
    );

    /// @notice Emitted when an operator responds to a task
    event TaskResponded(
        uint256 indexed taskIndex,
        Task task,
        string responseString,
        address operator
    );

    // --- Structs ---
    /// @notice Represents a code execution task
    /// @param codeArweaveUri URI of the code snippet on Arweave
    /// @param responseCount Required number of responses for task completion
    /// @param language Programming language of the code (js, go, python, etc.)
    /// @param taskCreatedBlock Block number when task was created
    struct Task {
        string codeArweaveUri;
        uint256 responseCount;
        string language;
        uint32 taskCreatedBlock;
    }

    // --- Modifiers ---
    /// @notice Ensures caller is a registered operator
    modifier onlyOperator() {
        require(operatorRegistered[msg.sender], "Caller is not a registered operator");
        _;
    }

    // --- Constructor ---
    /// @param _avsDirectory Address of the AVS Directory contract
    constructor(address _avsDirectory) {
        avsDirectory = _avsDirectory;
    }

    // --- External Functions ---
    /// @notice Registers an operator with the AVS
    /// @param operator Address of the operator to register
    /// @param operatorSignature Signature data for registration
    function registerOperatorToAVS(
        address operator,
        ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature
    ) external {
        IAVSDirectory(avsDirectory).registerOperatorToAVS(
            operator,
            operatorSignature
        );
        operatorRegistered[operator] = true;
    }

    /// @notice Deregisters an operator from the AVS
    /// @param operator Address of the operator to deregister
    function deregisterOperatorFromAVS(address operator) external onlyOperator {
        require(msg.sender == operator, "Only operator can deregister themselves");
        IAVSDirectory(avsDirectory).deregisterOperatorFromAVS(operator);
        operatorRegistered[operator] = false;
    }

    /// @notice Creates a new code execution task
    /// @return Task The newly created task
    function createNewTask(
        string memory codeArweaveUri,
        string memory language,
        uint256 responseCount
    ) external returns (Task memory) {
        Task memory newTask = Task({
            codeArweaveUri: codeArweaveUri,
            language: language,
            responseCount: responseCount,
            taskCreatedBlock: uint32(block.number)
        });

        allTaskHashes[latestTaskNum] = keccak256(abi.encode(newTask));
        
        emit ComputeRequestCreated(
            latestTaskNum,
            codeArweaveUri,
            language,
            responseCount,
            newTask.taskCreatedBlock
        );
        
        latestTaskNum++;
        return newTask;
    }

    /// @notice Allows operators to respond to a task
    /// @param task Task being responded to
    /// @param referenceTaskIndex Index of the task
    /// @param responseString Response data
    /// @param signature Signature of the response
    function respondToTask(
        Task calldata task,
        uint256 referenceTaskIndex,
        string memory responseString,
        bytes memory signature
    ) external onlyOperator {
        require(
            keccak256(abi.encode(task)) == allTaskHashes[referenceTaskIndex],
            "Task hash mismatch"
        );
        require(
            allTaskResponses[msg.sender][referenceTaskIndex].length == 0,
            "Operator has already responded"
        );

        bytes32 messageHash = keccak256(
            abi.encodePacked(responseString, referenceTaskIndex)
        );
        
        allTaskResponses[msg.sender][referenceTaskIndex] = signature;
        emit TaskResponded(referenceTaskIndex, task, responseString, msg.sender);
    }

    /// @notice Gets the total number of submissions for a task
    /// @param taskIndex Index of the task
    /// @return Number of submissions
    function getSubmissionCountByTaskIndex(uint256 taskIndex) external view returns (uint256) {
        return taskSubmissionCount[taskIndex];
    }
}
