# Voting service ensuring anonymity

## Overview

This project comprises four main components:

1. **Circom Schemes**: Utilized for specific cryptographic needs.
2. **Smart Contracts in Solidity**: Deployed for handling blockchain-based operations.
3. **Backend in Rust**: Provides robust and efficient server-side functionality.
4. **Frontend using Vue 3**: Ensures a responsive and interactive user interface.

Together, these components form a unique system known as the **Decentralized Anonymous Voting**.

## Connection Guide

To interact with the system:

1. **MetaMask Requirement**: Ensure you have MetaMask installed. It's crucial as it provides the provider used to retrieve data from the blockchain.
2. **Accessing the System**: Connect via the IP `95.179.198.33`.
3. **Blockchain Network Connection**:
    - Visit the official blockchain website: [https://hq.q.org/](https://hq.q.org/).
    - Click on "Connect MetaMask" to join the network.

The system is deployed on the testnet and functions as follows:

### User Interaction

1. **Creating a Proposal**: A user can initiate a proposal. The target of the proposal can be any context within the Blockchain.
2. **Proposal Commitment**:
    - Others can commit to the proposal by providing a hashed value (using Poseidon hash function) of two large random numbers: `secret` and `nullifier`.
    - This can be represented as `hash(secret | nullifier)`, where `|` is the concatenation.
    - A fee of 1 ETH is required for the proposal commitment.
3. **Voting Phase**:
    - Post the commitment phase, the voting phase commences.
    - Users must provide an inclusion proof that their commitment was counted in the commitment phase.
    - Users can vote from any account, provided they supply the correct ZKP proof.
    - If the voting passes, the proposal can be executed by anyone.

## Chapter: Security Considerations

### Client-Side Security

- Secrets never leave the client device and are always stored locally.

### Circom Circuit Code Example

```circom
// Code for ensuring integrity in the voting process
signal voterSquare <== voter;
signal proposalIdSquare <== proposalId;
```

This code ensures that only the proposal creator can use the generated proof. Each commitment is tied to a specific proposal.
#### Potential Threats and Mitigations


* DDoS via Proposal Commitments: While a DDoS attack is possible by creating numerous commitments, the high fee (1 ETH) and proposal relevance act as deterrents.
* Additional Protection: In real-world scenarios, further safeguards like limiting commitments to a predefined group of experts could be implemented.

### Overall Security

The system is secure, demonstrating the potential of Zero-Knowledge Proofs (ZKP) in future applications. Vulnerabilities like frontrunning or tree root manipulation in smart contracts are not feasible in this setup.
