// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import {TypeCaster} from "@solarity/solidity-lib/libs/utils/TypeCaster.sol";

import {VerifierHelper} from "@solarity/solidity-lib/libs/zkp/snarkjs/VerifierHelper.sol";

import {PoseidonSMT} from "./utils/PoseidonSMT.sol";

/**
 * @title Voting Contract
 * @notice Implements a decentralized voting system with privacy guarantees.
 */
contract Voting is PoseidonSMT {
    using TypeCaster for *; // TypeCaster library for type conversions.
    using VerifierHelper for address; // VerifierHelper library for zk-SNARK proof verification.

    // Enum to represent the voting options: None, For, Against.
    enum VotingOption {
        NONE,
        FOR,
        AGAINST
    }

    // Enum to represent the proposal status: Commitment, Voting, Execution, Rejected, Executed.
    enum ProposalStatus {
        NONE,
        COMMITMENT,
        VOTING,
        EXECUTION,
        REJECTED,
        EXECUTED
    }

    /**
     * @notice Struct to store proposal information.
     * @param id Unique identifier of the proposal.
     * @param remark Remark or description of the proposal.
     * @param params Parameters of the proposal including time periods and quorum.
     * @param counters Counters for tracking votes and commitments.
     * @param callData Call data to execute if the proposal passes.
     * @param isExecuted Flag to check if the proposal has been executed.
     */
    struct ProposalInfo {
        uint256 id;
        string remark;
        ProposalParams params;
        ProposalCounters counters;
        bytes callData;
        bool isExecuted;
    }

    /**
     * @notice Struct to define proposal creation parameters.
     * @param commitmentPeriod Time period for commitments.
     * @param votingPeriod Time period for voting.
     * @param proposalExecutionPeriod Time period for proposal execution.
     * @param requiredQuorum Quorum required for the proposal to pass.
     * @param requiredMajority Majority required for the proposal to pass.
     */
    struct ProposalData {
        uint256 commitmentPeriod;
        uint256 votingPeriod;
        uint256 proposalExecutionPeriod;
        uint256 requiredQuorum;
        uint256 requiredMajority;
    }

    /**
     * @notice Struct to define proposal parameters during its lifecycle.
     * @param votingEndTime Timestamp when voting ends.
     * @param commitmentEndTime Timestamp when commitments end.
     * @param proposalExecutionEndTime Timestamp when proposal execution ends.
     * @param requiredQuorum Quorum required for the proposal to pass.
     * @param requiredMajority Majority required for the proposal to pass.
     */
    struct ProposalParams {
        uint256 votingEndTime;
        uint256 commitmentEndTime;
        uint256 proposalExecutionEndTime;
        uint256 requiredQuorum;
        uint256 requiredMajority;
    }

    /**
     * @notice Struct to store counters related to a proposal.
     * @param votesFor Count of votes in favor.
     * @param votesAgainst Count of votes against.
     * @param commitments Count of commitments made.
     */
    struct ProposalCounters {
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 commitments;
    }

    uint256 public constant PERCENTAGE_100 = 10 ** 27; // Used for percentage calculations.

    /// Address of the verifier contract for zk-SNARKs.
    address public verifier;

    /// Count of proposals, used as a unique identifier for new proposals.
    uint256 public proposalsCount;

    /// Commitments mapping to avoid double voting.
    mapping(bytes32 => bool) public commitments;

    /// Nullifiers mapping to ensure vote privacy.
    mapping(bytes32 => bool) public nullifies;

    /// Mapping of roots for historical verification.
    mapping(bytes32 => bool) public rootsHistory;

    /// Mapping of proposal IDs to their corresponding proposal information.
    mapping(uint256 => ProposalInfo) public proposals;

    /**
     * @notice Event emitted when a proposal is created.
     */
    event ProposalCreated(uint256 indexed proposalId, ProposalInfo proposal);

    /**
     * @notice Event emitted when a commitment is added.
     */
    event AddedCommitment(uint256 indexed proposalId, bytes32 commitment, uint256 blockNumber);

    modifier onlyThis() {
        require(msg.sender == address(this), "Voting: caller is not this contract");
        _;
    }

    /**
     * @notice Constructor for the Voting contract.
     * @param treeHeight_ Height of the incremental Merkle Tree, affecting the maximum number of votes.
     * @param verifier_ Address of the verifier contract.
     *
     * The tree height used in the verifier contract must match `treeHeight_`.
     */
    constructor(uint256 treeHeight_, address verifier_) {
        __PoseidonSMT_init(treeHeight_);

        verifier = verifier_;
    }

    receive() external payable {}

    /**
     * @notice Distributes funds to a specified recipient
     * Restricted to being called only by this contract (enforced by the `onlyThis` modifier)
     * @param recipient_ The address of the recipient receiving the funds
     * @param amount_ The amount of Ether (in wei) to be sent
     *
     * Purpose: This function is used to transfer funds from the contract to a specified address.
     *          It is primarily used for fund management within the contract, ensuring that funds
     *          can be sent to different parties as dictated by the contract's logic.
     *
     * Constraints:
     * - The amount must be greater than 0 to prevent empty transactions.
     * - The function call to transfer funds must be successful; otherwise, it reverts to ensure
     *   integrity of the transaction.
     */
    function distributeFunds(address recipient_, uint256 amount_) external onlyThis {
        require(amount_ > 0, "Voting: amount must be greater than 0");

        (bool success_, ) = recipient_.call{value: amount_}("");

        require(success_, "Voting: funds distribution failed");
    }

    /**
     * @notice Creates a new proposal in the contract
     * Increments the `proposalsCount` to assign a unique ID to the new proposal
     * @param remark_ A brief description or remark about the proposal
     * @param proposalData_ Struct containing essential parameters for the proposal
     * @param callData_ Execution data for the proposal, if any
     * @return id_ The unique identifier for the newly created proposal
     *
     * Purpose: To initiate a new proposal within the voting system. This function is crucial for
     *          starting the voting process on various issues or decisions within the contract's
     *          domain. It sets up all necessary parameters and data for a fair and transparent
     *          voting process.
     *
     * Constraints:
     * - All periods defined in `proposalData_` (commitment, voting, and execution periods) must
     *   be greater than 0 to ensure a valid timeframe for each phase of the proposal.
     */
    function createProposal(
        string memory remark_,
        ProposalData memory proposalData_,
        bytes memory callData_
    ) external returns (uint256 id_) {
        require(
            proposalData_.commitmentPeriod > 0,
            "Voting: commitment period must be greater than 0"
        );
        require(proposalData_.votingPeriod > 0, "Voting: voting period must be greater than 0");
        require(
            proposalData_.proposalExecutionPeriod > 0,
            "Voting: proposal execution period must be greater than 0"
        );

        id_ = ++proposalsCount;

        ProposalInfo storage proposal_ = proposals[id_];

        proposal_.id = id_;
        proposal_.remark = remark_;

        proposal_.params.commitmentEndTime = block.timestamp + proposalData_.commitmentPeriod;
        proposal_.params.votingEndTime =
            proposal_.params.commitmentEndTime +
            proposalData_.votingPeriod;
        proposal_.params.proposalExecutionEndTime =
            proposal_.params.votingEndTime +
            proposalData_.proposalExecutionPeriod;

        proposal_.params.requiredQuorum = proposalData_.requiredQuorum;
        proposal_.params.requiredMajority = proposalData_.requiredMajority;

        proposal_.callData = callData_;

        emit ProposalCreated(id_, proposal_);
    }

    /**
     * @notice Commits to a proposal by sending a unique commitment hash
     * Used for voting anonymously via commitments
     * @param proposalId_ The ID of the proposal to commit to
     * @param commitment_ A unique hash representing the voter's commitment
     *
     * Purpose: This function allows a voter to commit to a proposal anonymously, using a
     *          unique hash. It is a critical part of ensuring privacy and integrity in the
     *          voting process.
     *
     * Constraints:
     * - The sent value must be exactly 1 ether to participate
     * - The commitment hash must be unique and not already registered
     * - The current time must be before the proposal's commitment end time
     */
    function commitOnProposal(uint256 proposalId_, bytes32 commitment_) external payable {
        require(msg.value == 1 ether, "Voting: value must be 1 ether");
        require(!commitments[commitment_], "Voting: commitment already exists");

        ProposalInfo storage proposal_ = proposals[proposalId_];

        ProposalStatus status_ = getProposalStatus(proposalId_);
        require(status_ == ProposalStatus.COMMITMENT, "Voting: status is not COMMITMENT");

        _add(commitment_);
        commitments[commitment_] = true;
        rootsHistory[getRoot()] = true;

        ++proposal_.counters.commitments;

        emit AddedCommitment(proposalId_, commitment_, block.number);
    }

    /**
     * @notice Casts a vote on a specific proposal using zk-SNARK proof for privacy
     * Voting is based on previous commitments and uses zk-SNARK for privacy
     * @param proposalId_ The ID of the proposal being voted on
     * @param nullifierHash_ Hash used to ensure a voter votes only once
     * @param root_ Merkle root of the voting tree
     * @param proof_ zk-SNARK proof data
     * @param votingOption_ The voter's choice (For, Against, None)
     *
     * Purpose: Enables voters to cast their vote on proposals in a private and secure manner,
     *          using cryptographic proofs. It is a key function for maintaining the integrity
     *          and confidentiality of the voting process.
     *
     * Constraints:
     * - The nullifier hash must be unique (not used before)
     * - The provided Merkle root must exist in the contract's history
     * - The zk-SNARK proof must be valid and verified by the verifier contract
     * - Voting must occur after the commitment period and before the voting period ends
     */
    function voteOnProposal(
        uint256 proposalId_,
        bytes32 nullifierHash_,
        bytes32 root_,
        VerifierHelper.ProofPoints calldata proof_,
        VotingOption votingOption_
    ) external {
        require(!nullifies[nullifierHash_], "Voting: nullifier already exists");
        require(rootsHistory[root_], "Voting: root does not exist");

        require(
            verifier.verifyProofSafe(
                [
                    uint256(root_),
                    uint256(nullifierHash_),
                    uint256(uint160(msg.sender)),
                    proposalId_
                ].asDynamic(),
                proof_,
                4
            ),
            "Voting: Invalid vote proof"
        );

        nullifies[nullifierHash_] = true;

        ProposalInfo storage proposal_ = proposals[proposalId_];

        ProposalStatus status_ = getProposalStatus(proposalId_);
        require(status_ == ProposalStatus.VOTING, "Voting: status is not VOTING");

        if (votingOption_ == VotingOption.FOR) {
            ++proposal_.counters.votesFor;
        } else {
            ++proposal_.counters.votesAgainst;
        }
    }

    /**
     * @notice Executes a proposal after the voting period, if it meets required conditions
     * Checks for quorum and majority before executing a proposal
     * @param proposalId_ The ID of the proposal to execute
     *
     * Purpose: This function is critical for concluding the voting process of a proposal.
     *          It executes the action(s) proposed if the voting outcome meets the predefined
     *          criteria of quorum and majority.
     *
     * Constraints:
     * - The current time must be after the voting end time
     * - The proposal must not have been executed before
     * - Requires that the voting results meet or exceed specified thresholds for quorum and majority.
     * - The execution of the proposal's actions must succeed for the process to complete.
     */
    function executeProposal(uint256 proposalId_) external {
        ProposalInfo storage proposal_ = proposals[proposalId_];

        ProposalStatus status_ = getProposalStatus(proposalId_);
        require(status_ == ProposalStatus.EXECUTION, "Voting: status is not EXECUTION");

        proposal_.isExecuted = true;

        if (proposal_.callData.length == 0) {
            return;
        }

        (bool success_, ) = address(this).call(proposal_.callData);
        require(success_, "Voting: proposal execution failed");
    }

    function getProposalStatus(uint256 proposalId_) public view returns (ProposalStatus) {
        ProposalInfo storage proposal_ = proposals[proposalId_];

        if (proposal_.id == 0) {
            return ProposalStatus.NONE;
        }

        if (proposal_.isExecuted) {
            return ProposalStatus.EXECUTED;
        }

        if (block.timestamp < proposal_.params.commitmentEndTime) {
            return ProposalStatus.COMMITMENT;
        }

        if (block.timestamp < proposal_.params.votingEndTime) {
            return ProposalStatus.VOTING;
        }

        uint256 quorum_ = _calculatePercentage(
            proposal_.counters.votesFor + proposal_.counters.votesAgainst,
            proposal_.counters.commitments
        );
        uint256 majority_ = _calculatePercentage(
            proposal_.counters.votesFor,
            proposal_.counters.votesFor + proposal_.counters.votesAgainst
        );

        if (
            quorum_ < proposal_.params.requiredQuorum ||
            majority_ < proposal_.params.requiredMajority
        ) {
            return ProposalStatus.REJECTED;
        }

        if (block.timestamp < proposal_.params.proposalExecutionEndTime) {
            return ProposalStatus.EXECUTION;
        }

        return ProposalStatus.REJECTED;
    }

    function _calculatePercentage(uint256 part, uint256 amount) internal pure returns (uint256) {
        if (amount == 0) {
            return 0;
        }

        return (part * PERCENTAGE_100) / amount;
    }
}
