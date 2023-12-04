import { ethers } from 'ethers'

import { config } from '@config'

import { ProposalBaseInfo } from '@/types/proposals'

import { Voting, Voting__factory } from '@bindings'
import { SecretPair } from '@/types/secrets'
import { getCommitment, getZKP } from '@/gateway/secrets'

export async function getProposals() {
  const voting = await getVotingInstance()

  const proposalCount = await voting.proposalsCount()

  const proposals: ProposalBaseInfo[] = []

  for (let i = 0; i <= proposalCount; i++) {
    const proposal = await voting.proposals(i)

    if (proposal.params.requiredMajority === 0n) {
      continue
    }

    const proposalBaseInfo: ProposalBaseInfo = toProposalBaseInfo(proposal)

    proposalBaseInfo.status = await getProposalStatus(proposal.id)

    proposals.push(proposalBaseInfo)
  }

  return proposals
}

export async function getProposal(id: bigint) {
  const voting = await getVotingInstance()

  const proposal = await voting.proposals(id)

  if (proposal.params.requiredMajority === 0n) {
    return undefined
  }

  const proposalBaseInfo: ProposalBaseInfo = toProposalBaseInfo(proposal)

  proposalBaseInfo.status = await getProposalStatus(proposal.id)

  return proposalBaseInfo
}

export const TWO_PERCENTAGE = '20000000000000000000000000'
export const FIVE_PERCENTAGE = '50000000000000000000000000'
export const FIFTY_PERCENTAGE = '500000000000000000000000000'

const DEFAULT_DATA = {
  commitmentPeriod: 100n,
  votingPeriod: 100n,
  proposalExecutionPeriod: 100n,
  requiredQuorum: FIVE_PERCENTAGE,
  requiredMajority: FIFTY_PERCENTAGE,
}

export async function createProposal(remark: string) {
  const voting = await getVotingInstance()
  const signer = await getDefaultSigner()

  const votingInterface = Voting__factory.createInterface()
  const callData = votingInterface.encodeFunctionData('distributeFunds', [
    await signer.getAddress(),
    ethers.parseEther('1'),
  ])

  return voting.createProposal(remark, DEFAULT_DATA, callData)
}

export async function commitOnProposal(
  proposalId: bigint,
  secrets: SecretPair,
) {
  const voting = await getVotingInstance()
  return voting.commitOnProposal(proposalId, getCommitment(secrets), {
    value: ethers.parseEther('1'),
  })
}

export async function voteOnProposal(
  proposalId: bigint,
  secrets: SecretPair,
  option: number,
) {
  const voting = await getVotingInstance()
  const signer = await getDefaultSigner()

  const dataToVerify = await getZKP(
    secrets,
    await signer.getAddress(),
    proposalId.toString(),
  )

  return voting.voteOnProposal(
    proposalId,
    dataToVerify!.nullifierHash,
    dataToVerify!.tree_root,
    dataToVerify!.formattedProof,
    option,
  )
}

export async function executeProposal(proposalId: bigint) {
  const voting = await getVotingInstance()
  return voting.executeProposal(proposalId)
}

export async function getProposalStatus(id: bigint): Promise<ProposalStatus> {
  const voting = await getVotingInstance()

  const proposal = await voting.getProposalStatus(id)

  return convertProposalStatus(proposal)
}

export type ProposalStatus =
  | 'voting-status.none'
  | 'voting-status.commitment'
  | 'voting-status.voting'
  | 'voting-status.execution'
  | 'voting-status.rejected'
  | 'voting-status.executed'

export function convertProposalStatus(status: bigint): ProposalStatus {
  switch (status) {
    case 0n:
      return 'voting-status.none'
    case 1n:
      return 'voting-status.commitment'
    case 2n:
      return 'voting-status.voting'
    case 3n:
      return 'voting-status.execution'
    case 4n:
      return 'voting-status.rejected'
    case 5n:
      return 'voting-status.executed'
    default:
      return 'voting-status.none'
  }
}

export async function getContractBalance() {
  const provider = new ethers.BrowserProvider(window.ethereum!)

  return provider.getBalance(config.APP_VOTING_CONTRACT_ADDRESS)
}

async function getVotingInstance() {
  if (!window.ethereum) {
    throw new Error('MetaMask is not installed')
  }

  const provider = new ethers.BrowserProvider(window.ethereum)

  await provider.send('eth_requestAccounts', [])

  const signer = await provider.getSigner()

  return Voting__factory.connect(config.APP_VOTING_CONTRACT_ADDRESS, signer)
}

async function getDefaultSigner() {
  if (!window.ethereum) {
    throw new Error('MetaMask is not installed')
  }

  const provider = new ethers.BrowserProvider(window.ethereum)

  await provider.send('eth_requestAccounts', [])

  return provider.getSigner()
}

function toProposalBaseInfo(
  proposal: Voting.ProposalInfoStructOutput,
): ProposalBaseInfo {
  return {
    id: proposal.id,
    remark: proposal.remark,
    callData: proposal.callData,
    params: {
      requiredMajority: proposal.params.requiredMajority,
      requiredQuorum: proposal.params.requiredQuorum,
      votingEndTime: proposal.params.votingEndTime,
      commitmentEndTime: proposal.params.commitmentEndTime,
      proposalExecutionEndTime: proposal.params.proposalExecutionEndTime,
    },
    counters: {
      votedAgainst: proposal.counters.votesAgainst,
      votedFor: proposal.counters.votesFor,
      commitments: proposal.counters.commitments,
    },
    status: '',
    executed: proposal.isExecuted,
  }
}
