/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/ban-ts-comment */

// @ts-ignore
import * as snarkjs from 'snarkjs'

import { ethers } from 'ethers'

import { Poseidon } from '@iden3/js-crypto'

import { SecretPair } from '@/types/secrets'

import { VerifierHelper, Voting } from '@/generated-types/Voting'

export function poseidonHash(data: string): string {
  data = ethers.hexlify(data)
  const chunks = splitHexIntoChunks(data.replace('0x', ''), 64)
  const inputs = chunks.map(v => BigInt(v))
  return ethers.toBeHex(Poseidon.hash(inputs), 32)
}

export function generateSecrets(): SecretPair {
  const secret = ethers.randomBytes(28)
  const nullifier = ethers.randomBytes(28)

  return {
    secret: padElement(ethers.hexlify(secret)),
    nullifier: padElement(ethers.hexlify(nullifier)),
    proposalId: '0',
  }
}

export function getCommitment(pair: SecretPair): string {
  return poseidonHash(
    pair.secret + pair.nullifier.replace('0x', '') + ethers.toBeHex(pair.proposalId, 32).replace('0x', ''),
  )
}

export function getNullifierHash(pair: SecretPair): string {
  return poseidonHash(pair.nullifier + ethers.toBeHex(pair.proposalId, 32).replace('0x', ''))
}

export function getBytes32PoseidonHash(element: string) {
  return poseidonHash(ethers.toBeHex(element, 32))
}

export async function getZKP(contract: Voting, pair: SecretPair, voter: string, proposalId: string) {
  pair.proposalId = proposalId
  const leaf = getBytes32PoseidonHash(getCommitment(pair))
  const nullifierHash = getNullifierHash(pair)

  const smtProof = await contract.getProof(leaf)

  const { proof } = await snarkjs.groth16.fullProve(
    {
      root: smtProof.root,
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
    '/data/voting.wasm',
    '/data/circuit_final.zkey',
  )

  swap(proof.pi_b[0], 0, 1)
  swap(proof.pi_b[1], 0, 1)

  const formattedProof: VerifierHelper.ProofPointsStruct = {
    a: proof.pi_a.slice(0, 2).map((x: any) => padElement(BigInt(x))),
    b: proof.pi_b.slice(0, 2).map((x: any[]) => x.map((y: any) => padElement(BigInt(y)))),
    c: proof.pi_c.slice(0, 2).map((x: any) => padElement(BigInt(x))),
  }

  return {
    tree_root: smtProof.root,
    formattedProof,
    nullifierHash,
  }
}

// Function to swap two elements in an array
function swap(arr: any, i: number, j: number) {
  const temp = arr[i]
  arr[i] = arr[j]
  arr[j] = temp
}

function padElement(element: any) {
  return ethers.toBeHex(element, 32)
}

function splitHexIntoChunks(hexString: string, chunkSize = 64) {
  const regex = new RegExp(`.{1,${chunkSize}}`, 'g')
  const chunks = hexString.match(regex)

  if (!chunks) {
    throw new Error('Invalid hex string')
  }

  return chunks.map(chunk => '0x' + chunk)
}
