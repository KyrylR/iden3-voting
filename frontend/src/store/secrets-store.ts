import { defineStore } from 'pinia'

import { ActiveSecret, SecretPair } from '@/types/secrets'
import { id } from 'ethers'
import { generateSecrets } from '@/gateway/secrets'

export const useSecretStore = defineStore('secrets', {
  state: (): {
    secrets: Map<string, SecretPair>
    activeSecrets: Map<string, ActiveSecret>
  } => ({
    secrets: new Map<string, SecretPair>(),
    activeSecrets: new Map<string, ActiveSecret>(),
  }),
  actions: {
    setSecrets(newSecrets: SecretPair[]) {
      for (const secret of newSecrets) {
        const key = id(JSON.stringify(secret))
        this.secrets.set(key, secret)
      }
    },
    addSecret(newSecret: SecretPair) {
      const key = id(JSON.stringify(newSecret))
      this.secrets.set(key, newSecret)
    },
    getSecret(): SecretPair {
      let secret = this.secrets.values().next().value

      if (!secret) {
        secret = generateSecrets()
        this.addSecret(secret)
      }

      return secret
    },
    setActiveSecrets(newActiveSecrets: ActiveSecret[]) {
      for (const activeSecret of newActiveSecrets) {
        this.activeSecrets.set(activeSecret.proposalId, activeSecret)
      }
    },
    addActiveSecret(newActiveSecret: ActiveSecret) {
      this.activeSecrets.set(newActiveSecret.proposalId, newActiveSecret)
    },
    useSecret(proposalId: bigint, secret: SecretPair) {
      this.activeSecrets.set(proposalId.toString(), {
        proposalId: proposalId.toString(),
        secret,
        status: 'active',
      })

      const keyToRemove = id(JSON.stringify(secret))
      this.secrets.delete(keyToRemove)
    },
    getActiveSecret(proposalId: bigint): ActiveSecret | undefined {
      // TODO: check if secret was used
      // TODO: safe array of secrets per proposal
      return this.activeSecrets.get(proposalId.toString())
    },
  },
})
