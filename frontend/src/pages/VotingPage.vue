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
import { onMounted, onUnmounted, ref } from 'vue'
import { bus, BUS_EVENTS } from '@/helpers'

import VotingHeader from '@/components/VotingHeader.vue'
import VotingStats from '@/components/VotingStats.vue'
import ProposalCard from '@/components/ProposalCard.vue'

import { ProposalBaseInfo } from '@/typings/proposals'
import { ethers } from 'ethers'
import { PERCENTAGE_100 } from '@/utils/proposals'

const proposals = ref<ProposalBaseInfo[]>([
  {
    id: 1n,
    remark: 'Funding for Q3 Marketing Campaign',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100,
      requiredQuorum: PERCENTAGE_100,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 86400),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 172800),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.commitment',
    executed: false,
  },
  {
    id: 2n,
    remark: 'Marketing Campaign',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100 / 2n,
      requiredQuorum: PERCENTAGE_100 / 2n,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 10000),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 1000),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.commitment',
    executed: false,
  },
  {
    id: 3n,
    remark: 'Some other proposal',
    callData: '0x...',
    params: {
      proposalExecutionPeriod: 86400n,
      requiredMajority: PERCENTAGE_100 / 2n,
      requiredQuorum: PERCENTAGE_100 / 2n,
      commitmentEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) - 1000),
      votingEndTime: ethers.toBigInt(Math.floor(Date.now() / 1000) + 172800),
      votingStartTime: ethers.toBigInt(Math.floor(Date.now() / 1000)),
      totalCommitment: 100n,
    },
    counters: {
      votedAgainst: 25n,
      votedFor: 75n,
      commitments: 150n,
    },
    status: 'voting-status.execution',
    executed: false,
  },
])

onMounted(() => {
  bus.on(BUS_EVENTS.success, handleSuccessEvent)
})

onUnmounted(() => {
  bus.off(BUS_EVENTS.success, handleSuccessEvent)
})

function handleSuccessEvent() {
  // Handle the success event, such as opening a modal for proposal creation
}
</script>

<style lang="scss" scoped>
.proposal-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr); // Creates two columns
  gap: toRem(12); // Keeps the existing gap
  margin: toRem(24) var(--voting-app-padding-left) toRem(0)
    var(--voting-app-padding-right);

  @media (max-width: 860px) {
    grid-template-columns: 1fr; // Adjusts to a single column on smaller screens
  }
}
</style>
