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

    <basic-modal
      class="option-modal"
      v-model:is-shown="isModalShown"
      :title="`Voting Option`"
    >
      <div class="option-modal__modal-options">
        <app-button
          class="option-modal__modal-options__modal-option-button"
          size="large"
          modification="border-rounded"
          :color="'success'"
          :icon-left="$icons.plus"
          :scheme="'flat'"
          :text="$t('common.yes')"
          @click="handleVoteOption(true)"
        />
        <app-button
          class="option-modal__modal-options__modal-option-button"
          size="large"
          modification="border-rounded"
          :color="'error'"
          :icon-left="$icons.minus"
          :scheme="'flat'"
          :text="$t('common.no')"
          @click="handleVoteOption(false)"
        />
      </div>
    </basic-modal>
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

import { useLocalStorage } from '@/composables/use-local-storage'

import { bus, BUS_EVENTS } from '@/helpers'
import { BasicModal } from '@/common'

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

const { getActiveSecret, useActiveSecret, getSecret, useSecret } =
  useLocalStorage()

const isModalShown = ref(false)

async function handlePendingClick() {
  isModalShown.value = true

  await updateProposalStatus()
}

const handleVoteOption = async (voteYes: boolean) => {
  const voteValue = voteYes ? 1 : 0
  const activeSecret = getActiveSecret(props.proposal.id)

  if (!activeSecret) {
    bus.emit(BUS_EVENTS.error, t('voting-details-header.no-active-secret'))
    return
  }

  if (
    await (
      await voteOnProposal(props.proposal.id, activeSecret.secret, voteValue)
    ).wait()
  ) {
    useActiveSecret(props.proposal.id, activeSecret)
  }

  await updateProposalStatus()

  isModalShown.value = false
}

async function handleCommitmentClick() {
  const secrets = getSecret()
  if (await (await commitOnProposal(props.proposal.id, secrets)).wait()) {
    useSecret(props.proposal.id, secrets)
  }

  await updateProposalStatus()
}

async function handleExecutionClick() {
  await executeProposal(props.proposal.id)

  await updateProposalStatus()
}

onMounted(async () => {
  await updateProposalStatus()
})

async function updateProposalStatus() {
  if (props.proposal) {
    proposalStatus.value = await getProposalStatus(props.proposal.id)
    statusState.value = getStatusState(proposalStatus.value)
  }
}

watch(
  () => props.proposal,
  async () => {
    await updateProposalStatus()
  },
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

.option-modal {
  &::v-global(.basic-modal__pane) {
    --basic-modal-max-width: #{toRem(452)};
  }

  &__modal-options {
    display: flex;
    justify-content: space-around;
    padding: toRem(8);
    gap: toRem(20);
  }
}
</style>
