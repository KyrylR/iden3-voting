import { useSecretStore } from '@/store/secrets-store'

export function useLocalStorage() {
  const secretStore = useSecretStore()

  function loadSecrets() {
    const storedSecrets = localStorage.getItem('secrets')
    if (storedSecrets) {
      secretStore.setSecrets(JSON.parse(storedSecrets))
    }

    const storedActiveSecrets = localStorage.getItem('activeSecrets')

    if (storedActiveSecrets) {
      secretStore.setActiveSecrets(JSON.parse(storedActiveSecrets))
    }
  }

  function saveSecrets() {
    const secretsToSave = Array.from(secretStore.secrets.values())
    const activeSecretsToSave = Array.from(secretStore.activeSecrets.values())

    localStorage.setItem('secrets', JSON.stringify(secretsToSave))
    localStorage.setItem('activeSecrets', JSON.stringify(activeSecretsToSave))
  }

  return { loadSecrets, saveSecrets }
}
