// @ts-ignore
import * as snarkjs from "snarkjs";

import { ethers } from "hardhat";
import { MerkleTree } from "merkletreejs";

import { poseidonHash } from "@/test/helpers/poseidon-hash";
import { getBytes32PoseidonHash, getPositionalProof } from "@/test/helpers/merkle-tree-helper";
import { VerifierHelper } from "@/generated-types/ethers/contracts/Voting";

export interface SecretPair {
  secret: string;
  nullifier: string;
}

export function generateSecrets(): SecretPair {
  const secret = ethers.randomBytes(28);
  const nullifier = ethers.randomBytes(28);

  return {
    secret: padElement(ethers.hexlify(secret)),
    nullifier: padElement(ethers.hexlify(nullifier)),
  };
}

export function getCommitment(pair: SecretPair): string {
  return poseidonHash(pair.secret + pair.nullifier.replace("0x", ""));
}

export function getNullifierHash(pair: SecretPair): string {
  return poseidonHash(pair.nullifier);
}

export async function getZKP(pair: SecretPair, voter: string, proposalId: string, root: string, tree: MerkleTree) {
  const leaf = getBytes32PoseidonHash(getCommitment(pair));
  const nullifierHash = getNullifierHash(pair);

  const [pathIndices, pathElements] = getPositionalProof(tree, leaf);

  const { proof } = await snarkjs.groth16.fullProve(
    {
      root,
      nullifierHash,
      secret: pair.secret,
      nullifier: pair.nullifier,
      voter,
      proposalId,
      pathElements,
      pathIndices,
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
