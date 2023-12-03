<template>
  <div class="voting-page">
    <!-- Voting Stats Section -->
    <voting-header />

    <!-- Voting Power Section -->
    <voting-stats />

    <!-- Proposals Section -->
    <div class="proposal-list">
      <proposal-card
        v-for="proposal in proposals"
        :key="proposal.id.toString()"
        :proposal="proposal"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'

import VotingHeader from '@/components/VotingHeader.vue'
import VotingStats from '@/components/VotingStats.vue'
import ProposalCard from '@/components/ProposalCard.vue'

import { ProposalBaseInfo } from '@/typings/proposals'

import { getProposals } from '@/gateway/proposals'

const proposals = ref<ProposalBaseInfo[]>([])

onMounted(async () => {
  proposals.value = await getProposals()
})
</script>

<style lang="scss" scoped>
.proposal-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: toRem(12);
  margin: toRem(24) var(--voting-app-padding-left) toRem(0)
    var(--voting-app-padding-right);

  @media (max-width: 860px) {
    grid-template-columns: 1fr;
  }
}
</style>
