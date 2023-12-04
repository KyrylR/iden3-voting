import { ActiveSecret, SecretPair } from '@/types/secrets'
import { id } from 'ethers'
import { generateSecrets } from '@/gateway/secrets'
import { ErrorHandler } from '@/helpers'

interface SecretStore {
  [key: string]: SecretPair
}

interface ActiveSecretStore {
  [key: string]: ActiveSecret[]
}

export function useLocalStorage() {
  function getSecretStore(): SecretStore {
    const storedSecrets = localStorage.getItem('secrets')

    const unzipped = storedSecrets ? JSON.parse(storedSecrets) : []

    const keys = unzipped.map((pair: string[]) => pair[0])
    const values = unzipped.map((pair: string[]) => pair[1])

    const store: SecretStore = {}

    keys.forEach((key: string, index: number) => (store[key] = values[index]))

    return store
  }

  function setSecrets(secrets: SecretStore) {
    const keys = Object.keys(secrets)
    const values = Object.values(secrets)
    const zipped = keys.map((key, index) => [key, values[index]])

    localStorage.setItem('secrets', JSON.stringify(zipped))
  }

  function saveSecret(secret: SecretPair) {
    const existingSecrets = getSecretStore()
    existingSecrets[id(JSON.stringify(secret))] = secret
    setSecrets(existingSecrets)
  }

  function getSecret(): SecretPair {
    const existingSecrets = getSecretStore()
    let secret = Object.values(existingSecrets)[0]

    if (!secret) {
      secret = generateSecrets()
      saveSecret(secret)
    }

    return secret
  }

  function getSecretNumber(): number {
    return Object.values(getSecretStore()).length
  }

  function deleteSecret(secret: SecretPair) {
    const existingSecrets = getSecretStore()
    delete existingSecrets[id(JSON.stringify(secret))]
    setSecrets(existingSecrets)
  }

  function getActiveSecretStore(): ActiveSecretStore {
    const storedSecrets = localStorage.getItem('activeSecrets')

    const unzipped = storedSecrets ? JSON.parse(storedSecrets) : []

    const keys = unzipped.map((pair: string[]) => pair[0])
    const values = unzipped.map((pair: string[]) => pair[1])

    const store: ActiveSecretStore = {}

    keys.forEach((key: string, index: number) => (store[key] = values[index]))

    return store
  }

  function setActiveSecrets(activeSecrets: ActiveSecretStore) {
    const keys = Object.keys(activeSecrets)
    const values = Object.values(activeSecrets)
    const zipped = keys.map((key, index) => [key, values[index]])

    localStorage.setItem('activeSecrets', JSON.stringify(zipped))
  }

  function saveActiveSecret(activeSecret: ActiveSecret) {
    const existingActiveSecrets = getActiveSecretStore()
    if (!existingActiveSecrets[activeSecret.proposalId]) {
      existingActiveSecrets[activeSecret.proposalId] = []
    }
    existingActiveSecrets[activeSecret.proposalId].push(activeSecret)
    setActiveSecrets(existingActiveSecrets)
  }

  function useSecret(proposalId: bigint, secret: SecretPair) {
    const proposalIdStr = proposalId.toString()
    const existingActiveSecrets = getActiveSecretStore()

    if (!existingActiveSecrets[proposalIdStr]) {
      existingActiveSecrets[proposalIdStr] = []
    }

    existingActiveSecrets[proposalIdStr].push({
      proposalId: proposalIdStr,
      secret,
      status: 'active',
    })

    deleteSecret(secret)
    setActiveSecrets(existingActiveSecrets)
  }

  function getActiveSecret(proposalId: bigint): ActiveSecret | undefined {
    const activeSecrets = getActiveSecretStore()[proposalId.toString()]
    return activeSecrets.find(secret => secret.status === 'active')
  }

  function getActiveSecretsNumber(): number {
    return Object.values(getActiveSecretStore()).reduce(
      (count, secrets) =>
        count + secrets.filter(secret => secret.status === 'active').length,
      0,
    )
  }

  function useActiveSecret(proposalId: bigint, activeSecret: ActiveSecret) {
    if (activeSecret.status !== 'active') {
      ErrorHandler.process('Secret is not active')
      return
    }

    const proposalIdStr = proposalId.toString()
    const activeSecretsStore = getActiveSecretStore()

    const activeSecrets = activeSecretsStore[proposalIdStr]
    const index = activeSecrets.findIndex(
      secret =>
        id(JSON.stringify(secret.secret)) ===
        id(JSON.stringify(activeSecret.secret)),
    )
    activeSecrets[index].status = 'used'

    activeSecretsStore[proposalIdStr] = activeSecrets

    setActiveSecrets(activeSecretsStore)
  }

  return {
    getSecret,
    useSecret,
    saveSecret,
    getSecretNumber,
    getActiveSecret,
    getActiveSecretsNumber,
    useActiveSecret,
    saveActiveSecret,
  }
}
