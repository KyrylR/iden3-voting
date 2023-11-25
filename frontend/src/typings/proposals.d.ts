export type ProposalStatus =
  | 'voting-status.voting'
  | 'voting-status.commitment'
  | 'voting-status.execution'

export interface DaoProposalVotingParams {
  proposalExecutionPeriod: bigint
  requiredMajority: bigint
  requiredQuorum: bigint
  commitmentEndTime: bigint
  votingEndTime: bigint
  votingStartTime: bigint
  totalCommitment: bigint
}

export interface DaoProposalVotingCounters {
  votedAgainst: bigint
  votedFor: bigint
  commitments: bigint
}

export interface Proposal {
  id: bigint
  remark: string
  callData: string
  params: DaoProposalVotingParams
  counters: DaoProposalVotingCounters
  status: string
  executed: boolean
}

export type ProposalBaseInfo = Proposal
