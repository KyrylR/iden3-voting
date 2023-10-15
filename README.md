# Voting service based on the iden3 protocol 

A simple implementation of anonymous voting service based on the iden3 protocol with trusted party.

# Description 

This service provides a functionality where only eligible parties are able to participate in voting. 

We have a party that will set who is able to vote (issuer service -- backend). 
It will be implemented due to [iden3 protocol specification](https://docs.iden3.io/protocol/spec/).

Voting could have a unique feature (e.g. only those who are older than 18 years old can vote).

# Architecture

## Contracts

Core contracts are voting and ZKP verification contracts.

Voting is divided into two parts:
* Commitment phase: where users commit with their ZKP proof from issuer service to be able to vote
* Voting phase: where users vote with their ZKP of commitment that was done in the commitment phase

## Backend

We will have two small services: 
1. Issuer by iden3 protocol
2. Service that will store history of commitments for particular voting

## Frontend

Will provided a simple UI for voting and ZKP generations.

## Circuits

We will have circuits to generate ZKPs for voting commitment phase. 




