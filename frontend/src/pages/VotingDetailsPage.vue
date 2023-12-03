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

    <voting-details-header
      :proposal-id="proposal?.id"
      :proposal-status="statusState"
    />

    <voting-details-info :remark="proposal?.remark" />

    <div class="voting-details-page__cards">
      <quorum-card :proposal="proposal" />

      <majority-card :proposal="proposal" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ethers } from 'ethers'

import { computed, onMounted, onUnmounted, ref, watch } from 'vue'

import { useRoute } from 'vue-router'

import { bus, BUS_EVENTS } from '@/helpers'

import AppButton from '@/common/AppButton.vue'

import { getProposal } from '@/gateway/proposals'

import { ProposalBaseInfo } from '@/typings/proposals'
import { router } from '@/router'
import { ROUTE_NAMES } from '@/enums'
import VotingDetailsHeader from '@/components/VotingDetailsHeader.vue'
import VotingDetailsInfo from '@/components/VotingDetailsInfo.vue'
import { getProposalStatus, getStatusState } from '@/utils/proposals'
import QuorumCard from '@/components/QuorumCard.vue'
import MajorityCard from '@/components/MajorityCard.vue'

const route = useRoute()
const proposalId = ref(String(route.params.id))

const proposal = ref<ProposalBaseInfo | undefined>(undefined)

const proposalStatus = computed(() => getProposalStatus(proposal.value!))
const statusState = computed(() => getStatusState(proposalStatus.value))

onMounted(async () => {
  bus.on(BUS_EVENTS.success, handleSuccessEvent)

  try {
    proposal.value = await getProposal(ethers.toBigInt(proposalId.value))
    if (!proposal.value) {
      await router.push({ name: ROUTE_NAMES.notFound })
    }
  } catch (error) {
    await router.push({ name: ROUTE_NAMES.notFound })
  }
})

onUnmounted(() => {
  bus.off(BUS_EVENTS.success, handleSuccessEvent)
})

function handleSuccessEvent() {
  // Handle the success event, such as opening a modal for proposal creation
}

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

    @media (max-width: 860px) {
      grid-template-columns: 1fr;
    }
  }
}
</style>
