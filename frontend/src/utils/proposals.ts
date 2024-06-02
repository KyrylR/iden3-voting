import { ProposalBaseInfo } from '@/types/proposals'
import { ethers } from 'ethers'
import { ProposalStatus } from '@/gateway/proposals'

export type TagState = 'none' | 'pending' | 'commitment' | 'execution' | 'rejected' | 'executed'

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

export const getStatusState = (proposalStatus: ProposalStatus): TagState => {
  switch (proposalStatus) {
    case 'voting-status.none':
      return 'none'
    case 'voting-status.commitment':
      return 'commitment'
    case 'voting-status.voting':
      return 'pending'
    case 'voting-status.execution':
      return 'execution'
    case 'voting-status.rejected':
      return 'rejected'
    case 'voting-status.executed':
      return 'executed'
    default:
      return 'none'
  }
}

export function castAmount(value: bigint): string {
  if (value > 0n && value < 10n ** 12n) {
    return ethers.formatUnits(value, 'gwei') + ' GWei'
  }

  return ethers.formatEther(value) + ' ETH'
}

export const bigIntMax = (...args: bigint[]) => args.reduce((m, e) => (e > m ? e : m))
export const bigIntMin = (...args: bigint[]) => args.reduce((m, e) => (e < m ? e : m))
