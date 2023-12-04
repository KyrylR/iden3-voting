<template>
  <div class="voting-details-page">
    <app-button
      class="voting-details-page__back-button"
      size="medium"
      scheme="flat"
      :icon-left="$icons.arrowLeft"
      :text="$t('voting-details.back-btn')"
      @click="() => $router.back()"
    />

    <voting-details-header :proposal="proposal" />

    <voting-details-info :remark="proposal?.remark" />

    <div class="voting-details-page__cards">
      <quorum-card :proposal="proposal" />

      <majority-card :proposal="proposal" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ethers } from 'ethers'

import { onMounted, onUnmounted, ref, watch } from 'vue'

import { useRoute } from 'vue-router'

import AppButton from '@/common/AppButton.vue'

import {
  getProposal,
  getProposalStatus,
  ProposalStatus,
} from '@/gateway/proposals'

import { ProposalBaseInfo } from '@/types/proposals'
import { router } from '@/router'
import { ROUTE_NAMES } from '@/enums'
import VotingDetailsHeader from '@/components/VotingDetailsHeader.vue'
import VotingDetailsInfo from '@/components/VotingDetailsInfo.vue'
import { getStatusState, TagState } from '@/utils/proposals'
import QuorumCard from '@/components/QuorumCard.vue'
import MajorityCard from '@/components/MajorityCard.vue'

const route = useRoute()
const proposalId = ref(String(route.params.id))

const proposal = ref<ProposalBaseInfo | undefined>(undefined)

const proposalStatus = ref<ProposalStatus>('voting-status.none')
const statusState = ref<TagState>('none')

onMounted(async () => {
  try {
    proposal.value = await getProposal(ethers.toBigInt(proposalId.value))
    if (!proposal.value) {
      await router.push({ name: ROUTE_NAMES.notFound })
    }
  } catch (error) {
    await router.push({ name: ROUTE_NAMES.notFound })
  }

  proposalStatus.value = await getProposalStatus(
    ethers.toBigInt(proposalId.value),
  )
  statusState.value = getStatusState(proposalStatus.value)
})

let statusPollingInterval: NodeJS.Timer | undefined

onMounted(async () => {
  statusPollingInterval = setInterval(updateProposal, 3000)

  await updateProposal()
})

async function updateProposal() {
  if (proposal.value) {
    proposal.value = await getProposal(ethers.toBigInt(proposalId.value))
    proposalStatus.value = await getProposalStatus(
      ethers.toBigInt(proposalId.value),
    )
    statusState.value = getStatusState(proposalStatus.value)
  }
}

onUnmounted(() => {
  if (statusPollingInterval) {
    clearInterval(statusPollingInterval)
  }
})

watch(proposal, newVal => {
  if (newVal === undefined) {
    router.push({ name: ROUTE_NAMES.notFound })
  }
})
</script>

<style lang="scss" scoped>
.voting-details-page {
  padding: toRem(24) var(--voting-app-padding-left) toRem(36)
    var(--voting-app-padding-right);

  &__back-button {
    font-size: toRem(15);
  }

  &__cards {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: toRem(12);
    padding: toRem(8) var(--voting-app-padding-left) toRem(36)
      var(--voting-app-padding-right);
    margin-top: toRem(24);

    @media (width <= 53.75rem) {
      grid-template-columns: 1fr;
    }
  }
}
</style>
