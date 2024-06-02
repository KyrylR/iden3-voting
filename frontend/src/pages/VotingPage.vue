<template>
  <div class="voting-page">
    <!-- Voting Stats Section -->
    <voting-header />

    <!-- Voting Power Section -->
    <voting-stats />

    <!-- Proposals Section -->
    <div class="proposal-list">
      <proposal-card v-for="proposal in proposals" :key="proposal.id.toString()" :proposal="proposal" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue'

import VotingHeader from '@/components/VotingHeader.vue'
import VotingStats from '@/components/VotingStats.vue'
import ProposalCard from '@/components/ProposalCard.vue'

import { ProposalBaseInfo } from '@/types/proposals'

import { getProposals } from '@/gateway/proposals'

const proposals = ref<ProposalBaseInfo[]>([])
const itemsPerPage = ref(10)
const currentPage = ref(1)

const loadProposals = async () => {
  const newProposals = await getProposals(itemsPerPage.value, currentPage.value)
  proposals.value.push(...newProposals)
}

const handleScroll = () => {
  const { scrollTop, scrollHeight, clientHeight } = document.documentElement
  if (scrollTop + clientHeight >= scrollHeight - 5) {
    // Check if near the bottom
    currentPage.value++
    loadProposals()
  }
}

onMounted(async () => {
  await loadProposals()
  window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<style lang="scss" scoped>
.proposal-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: toRem(12);
  margin: toRem(24) var(--voting-app-padding-left) toRem(0) var(--voting-app-padding-right);

  @media (width <= 53.75rem) {
    grid-template-columns: 1fr;
  }
}
</style>
