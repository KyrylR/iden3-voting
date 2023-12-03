import { ethers } from 'ethers'

import { PERCENTAGE_100 } from '@/utils/proposals'

import { ProposalBaseInfo } from '@/typings/proposals'

const proposals: ProposalBaseInfo[] = [
  {
    id: 1n,
    remark: 'Funding for Q3 Marketing Campaign',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100,
      requiredQuorum: PERCENTAGE_100,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 86400),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 172800),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.commitment',
    executed: false,
  },
  {
    id: 2n,
    remark: 'Marketing Campaign',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100 / 2n,
      requiredQuorum: PERCENTAGE_100 / 2n,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 10000),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 1000),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.commitment',
    executed: false,
  },
  {
    id: 3n,
    remark: 'Some other proposal',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100 / 2n,
      requiredQuorum: PERCENTAGE_100 / 2n,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 1000),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 172800),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.execution',
    executed: false,
  },
]

export function getProposals() {
  return proposals
}

export function getProposal(id: bigint) {
  return proposals.find(proposal => proposal.id === id)
}
