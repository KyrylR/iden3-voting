<template>
  <header class="header">
    <div class="header__info">
      <h1 class="header__info__title">
        {{ '#' + props.proposal!.id + ' ' + $t('voting-details-header.title') }}
      </h1>

      <tag v-if="props.proposal!.status" :state="statusState" :class-name="``">
        <!-- eslint-disable-next-line vue-i18n/no-dynamic-keys -->
        {{ $t(proposalStatus) }}
      </tag>
    </div>

    <app-button
      v-if="shouldShowButton"
      class="header__vote-button"
      size="medium"
      modification="border-rounded"
      :color="buttonColor"
      :icon-left="$icons.cube"
      :scheme="'filled'"
      :text="buttonText"
      @click="handleButtonClick"
    />
  </header>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed, watch } from 'vue'

import { useI18n } from 'vue-i18n'

import AppButton from '@/common/AppButton.vue'

import Tag from '@/components/utils/Tag.vue'

import { ProposalBaseInfo } from '@/types/proposals'

import { getStatusState, TagState } from '@/utils/proposals'

import {
  commitOnProposal,
  executeProposal,
  getProposalStatus,
  ProposalStatus,
  voteOnProposal,
} from '@/gateway/proposals'

import { useSecretStore } from '@/store/secrets-store'
import { useLocalStorage } from '@/composables/use-local-storage'

const secretStore = useSecretStore()
const { loadSecrets, saveSecrets } = useLocalStorage()

const props = withDefaults(
  defineProps<{
    proposal: ProposalBaseInfo
  }>(),
  {
    proposal: undefined,
  },
)

const proposalStatus = ref<ProposalStatus>('voting-status.none')
const statusState = ref<TagState>('none')

const buttonColor = computed(() => {
  switch (statusState.value) {
    case 'pending':
      return 'warning'
    case 'commitment':
      return 'info'
    case 'execution':
      return 'success'
    default:
      return 'none'
  }
})

const t = useI18n().t

const buttonText = computed(() => {
  switch (statusState.value) {
    case 'pending':
      return t('voting-details-header.vote')
    case 'commitment':
      return t('voting-details-header.commit')
    case 'execution':
      return t('voting-details-header.execute')
    default:
      return ''
  }
})

const shouldShowButton = computed(() => {
  return !['rejected', 'executed', 'none'].includes(statusState.value)
})

const handleButtonClick = async () => {
  switch (statusState.value) {
    case 'pending':
      await handlePendingClick()
      break
    case 'commitment':
      await handleCommitmentClick()
      break
    case 'execution':
      await handleExecutionClick()
      break
  }
}

async function handlePendingClick() {
  const activeSecret = secretStore.getActiveSecret(props.proposal.id)

  await voteOnProposal(props.proposal.id, activeSecret!.secret, 1)

  await updateProposalStatus()
}

async function handleCommitmentClick() {
  const secrets = secretStore.getSecret()
  await commitOnProposal(props.proposal.id, secrets)

  secretStore.useSecret(props.proposal.id, secrets)

  saveSecrets()

  await updateProposalStatus()
}

async function handleExecutionClick() {
  await executeProposal(props.proposal.id)

  await updateProposalStatus()
}

onMounted(async () => {
  await updateProposalStatus()

  loadSecrets()
})

async function updateProposalStatus() {
  if (props.proposal) {
    proposalStatus.value = await getProposalStatus(props.proposal.id)
    statusState.value = getStatusState(proposalStatus.value)
  }
}

watch(
  () => props.proposal,
  async (newProposal, oldProposal) => {
    if (newProposal && newProposal.id !== oldProposal?.id) {
      await updateProposalStatus()
    }
  },
  { immediate: true },
)
</script>

<style lang="scss" scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: toRem(24);
  padding: toRem(36) var(--voting-app-padding-left) toRem(36)
    var(--voting-app-padding-right);
  background: var(--background-primary-main);
  border-bottom: var(--border-primary-main);

  @include respond-to(tablet) {
    flex-wrap: wrap;
  }

  &__info {
    display: flex;
    justify-content: flex-start;
    gap: toRem(12);

    &__title {
      font-size: toRem(36);
    }
  }

  &__vote-button {
    color: var(--black);
    width: toRem(150);
    font-size: toRem(15);
  }
}
</style>
