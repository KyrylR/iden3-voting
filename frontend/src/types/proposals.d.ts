export interface DaoProposalVotingParams {
  requiredMajority: bigint
  requiredQuorum: bigint
  votingEndTime: bigint
  commitmentEndTime: bigint
  proposalExecutionEndTime: bigint
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
