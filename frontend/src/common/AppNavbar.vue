<template>
  <div class="app-navbar">
    <app-logo class="app-navbar__logo" />

    <app-button class="app-navbar__link" :scheme="'flat'" :text="$routes.voting" :route="{ name: $routes.voting }" />

    <!-- MetaMask Connection Button -->
    <app-button
      v-if="!userAddress"
      class="app-navbar__metamask-button"
      :scheme="'flat'"
      :text="$t('navbar.connect-to-metamask')"
      @click="connectToMetaMask"
    />

    <!-- Display User Address -->
    <app-button v-if="userAddress" :scheme="'flat'" :text="truncatedAddress" @click="handleAddressClick" />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { TYPE, useToast } from 'vue-toastification'

import AppButton from '@/common/AppButton.vue'

import { copyToClipboard } from '@/helpers'

import { AppLogo } from '@/common/index'

const toast = useToast()
const { t } = useI18n()

const userAddress = ref<string | null>(null)

const truncatedAddress = computed(() => {
  if (!userAddress.value) return ''
  const length = userAddress.value.length
  const startChars = 6
  const endChars = 4
  if (length <= startChars + endChars) {
    return userAddress.value
  }
  return `${userAddress.value.substring(0, startChars)}...${userAddress.value.substring(length - endChars)}`
})

const handleAccountsChanged = (accounts: string[]) => {
  userAddress.value = accounts[0] || null
}

const fetchAccounts = async () => {
  if (!window.ethereum) {
    return
  }

  try {
    const accounts = await window.ethereum.request({
      method: 'eth_requestAccounts',
    })
    handleAccountsChanged(accounts)
  } catch (error) {
    toast(t('navbar.metamask-connection-rejected'), {
      type: TYPE.ERROR,
    })
  }
}

const connectToMetaMask = async () => {
  if (!window.ethereum) {
    toast(t('navbar.metamask-not-installed'), {
      type: TYPE.ERROR,
    })

    return
  }

  await fetchAccounts()
}

const handleAddressClick = async () => {
  try {
    await copyToClipboard(userAddress.value!)

    toast(t('navbar.address-copied'), { type: TYPE.SUCCESS })
  } catch (error) {
    toast(t('navbar.address-copy-failed'), { type: TYPE.ERROR })
  }
}

onMounted(() => {
  if (!window.ethereum) {
    return
  }

  fetchAccounts()

  window.ethereum.on('accountsChanged', handleAccountsChanged)
})

onUnmounted(() => {
  if (window.ethereum) {
    window.ethereum.removeListener('accountsChanged', handleAccountsChanged)
  }
})
</script>

<style lang="scss" scoped>
.app-navbar {
  display: flex;
  align-items: center;
  gap: toRem(24);
  padding: toRem(24) var(--app-padding-right) toRem(24) var(--app-padding-left);
  background: var(--background-primary-main);
  border-bottom: var(--border-primary-main);

  @include respond-to(tablet) {
    flex-wrap: wrap;
  }
}

.app-navbar__logo {
  margin-right: auto;

  @include respond-to(xsmall) {
    width: 100%;
    margin-bottom: toRem(24);
  }
}

.app-navbar__link {
  opacity: 0.25;

  &:first-child {
    margin-left: auto;
  }

  &.router-link-active {
    opacity: 1;
  }
}
</style>
