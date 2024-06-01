// @ts-ignore
import * as snarkjs from "snarkjs";

import { ethers } from "hardhat";

import { getBytes32PoseidonHash, poseidonHash } from "@/test/helpers/poseidon-hash";
import { VerifierHelper, Voting } from "@/generated-types/ethers/contracts/Voting";

export interface CommitmentFields {
  secret: string;
  nullifier: string;
  proposalId: string;
}

export function generateSecrets(proposalId: number): CommitmentFields {
  const secret = ethers.randomBytes(28);
  const nullifier = ethers.randomBytes(28);

  return {
    secret: padElement(ethers.hexlify(secret)),
    nullifier: padElement(ethers.hexlify(nullifier)),
    proposalId: padElement(ethers.toBeHex(proposalId)),
  };
}

export function getCommitment(pair: CommitmentFields): string {
  return poseidonHash(pair.secret + pair.nullifier.replace("0x", "") + pair.proposalId.replace("0x", ""));
}

export function getNullifierHash(pair: CommitmentFields): string {
  return poseidonHash(pair.nullifier);
}

export async function getZKP(contract: Voting, pair: CommitmentFields, voter: string, proposalId: string) {
  const leaf = getBytes32PoseidonHash(getCommitment(pair));
  const nullifierHash = getNullifierHash(pair);

  const smtProof = await contract.getProof(leaf);

  const { proof } = await snarkjs.groth16.fullProve(
    {
      root: await contract.getRoot(),
      nullifierHash,
      voter,
      proposalId,
      secret: pair.secret,
      nullifier: pair.nullifier,
      siblings: smtProof.siblings,
      auxKey: smtProof.auxKey,
      auxValue: smtProof.auxValue,
      auxIsEmpty: smtProof.auxExistence,
      isExclusion: 0,
    },
    `./circuits/outputs/voting.wasm`,
    `./circuits/outputs/circuit_final.zkey`
  );

  swap(proof.pi_b[0], 0, 1);
  swap(proof.pi_b[1], 0, 1);

  const formattedProof: VerifierHelper.ProofPointsStruct = {
    a: proof.pi_a.slice(0, 2).map((x: any) => padElement(BigInt(x))),
    b: proof.pi_b.slice(0, 2).map((x: any[]) => x.map((y: any) => padElement(BigInt(y)))),
    c: proof.pi_c.slice(0, 2).map((x: any) => padElement(BigInt(x))),
  };

  return {
    formattedProof,
    nullifierHash,
  };
}

export function checkMerkleProof(leaf: string, pathIndices: number[], pathElements: string[], _root: string) {
  for (let i = 0; i < pathIndices.length; i++) {
    const pathElement = pathElements[i];
    const pathIndex = pathIndices[i];

    if (pathIndex === 0) {
      leaf = poseidonHash(pathElement + leaf.replace("0x", ""));
    } else {
      leaf = poseidonHash(leaf + pathElement.replace("0x", ""));
    }
  }
}

// Function to swap two elements in an array
function swap(arr: any, i: number, j: number) {
  const temp = arr[i];
  arr[i] = arr[j];
  arr[j] = temp;
}

function padElement(element: any) {
  return ethers.toBeHex(element, 32);
}
