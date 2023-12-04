export interface SecretPair {
  secret: string
  nullifier: string
}

export type SecretStatus = 'active' | 'used' | 'free'

export interface ActiveSecret {
  proposalId: string
  secret: SecretPair
  status: SecretStatus
}

export interface ProofRequestBody {
  commitment: string
}

export interface ProofResponse {
  contract_address: string
  tree_height: number
  commitment: string
  tree_root: string
  proof_hashes: string[]
  proof_indices: number[]
}
