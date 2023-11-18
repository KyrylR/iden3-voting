# Decentralized Voting System with Privacy Guarantees

This project implements a decentralized voting system leveraging Zero-Knowledge Proofs (ZKPs) to ensure voter privacy and security. It serves as a practical exploration of ZKP in the context of anonymous voting, ensuring that voters can express their opinions without revealing their identity.

## Introduction

### Contract Overview

The `Voting` contract, developed in Solidity, allows users to create proposals, commit to them, vote, and execute these proposals in a decentralized and anonymous manner. This contract uses Zero-Knowledge Proof to maintain voter anonymity and prevent double voting, ensuring a fair and transparent voting process.

### Zero-Knowledge Proofs in Voting

Zero-Knowledge Proofs provide the cryptographic backbone for the voting system. They allow voters to prove their right to vote and the validity of their vote without revealing their identity or the specifics of their vote.

## Key Features

### Proposal Creation and Voting

1. **Proposal Creation**: Users can create proposals, each identified by a unique ID, and define parameters like voting periods and quorum requirements.
2. **Commitment Period**: Voters anonymously commit to a proposal using a unique hash, ensuring their future vote remains secret.
3. **Voting**: After the commitment period, voters anonymously cast their vote for or against proposals using ZKPs.
4. **Proposal Execution**: Proposals that meet the required quorum and majority are executed, and the actions proposed are carried out.

### Privacy and Security

- **Anonymity**: Voter identities remain concealed throughout the process.
- **Double Voting Prevention**: The system uses nullifiers and commitment hashes to prevent double voting.
- **ZKP Verification**: Votes are validated using Zero-Knowledge Proofs, ensuring their legitimacy without revealing voter information.

## How it Works

1. **Committing to a Proposal**: Voters make a commitment by sending a unique hash along with a fixed amount of Ether (1 ETH).
2. **Voting on a Proposal**: After the commitment period, voters prove their right to vote through a ZKP and cast their vote anonymously.
3. **Executing Proposals**: Once the voting period ends, proposals that meet the quorum and majority are executed. If the proposal includes specific actions (encoded in `callData`), they are performed by the contract.

## Use Case

Consider a scenario where a community needs to make decisions collectively without revealing individual preferences. Our voting system enables members to propose actions, vote, and implement decisions in a secure and private manner, ideal for sensitive decisions in decentralized organizations or communities.

## Acknowledgments

This project is inspired by the principles and potential applications of Zero-Knowledge Proofs in enhancing privacy and security in decentralized systems. For more in-depth understanding, refer to resources like [Zero-Knowledge: A Breathtaking Journey](https://www.zeroknowledgeblog.com/).
