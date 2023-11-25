import { ProposalBaseInfo, ProposalStatus } from '@/typings/proposals'
import { ethers } from 'ethers'

export type TagState = 'pending' | 'commitment' | 'execution'

export const PERCENTAGE_100 = ethers.toBigInt(10n ** 27n)

export function getCurrentQuorum(proposal: ProposalBaseInfo): bigint {
  const { votedFor, votedAgainst, commitments } = proposal.counters

  if (commitments === 0n) {
    return 0n
  }

  return ((votedFor + votedAgainst) * PERCENTAGE_100) / commitments
}

export function getCurrentMajority(proposal: ProposalBaseInfo): bigint {
  const { votedFor, votedAgainst } = proposal.counters

  if (votedFor + votedAgainst === 0n) {
    return 0n
  }

  return (votedFor * PERCENTAGE_100) / (votedFor + votedAgainst)
}

export function getProposalStatus(proposal: ProposalBaseInfo): ProposalStatus {
  const now = Date.now() / 1000

  if (proposal.params.commitmentEndTime > now) {
    return 'voting-status.commitment'
  } else if (proposal.params.votingEndTime > now) {
    return 'voting-status.voting'
  }

  return 'voting-status.execution'
}

export const getStatusState = (proposalStatus: ProposalStatus): TagState => {
  switch (proposalStatus) {
    case 'voting-status.commitment':
      return 'commitment'
    case 'voting-status.voting':
      return 'pending'
    case 'voting-status.execution':
      return 'execution'
    default:
      return 'pending'
  }
}

export const bigIntMax = (...args: bigint[]) =>
  args.reduce((m, e) => (e > m ? e : m))
export const bigIntMin = (...args: bigint[]) =>
  args.reduce((m, e) => (e < m ? e : m))
