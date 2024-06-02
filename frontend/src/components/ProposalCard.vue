<template>
  <router-link :to="proposalLink" v-bind="$attrs" class="proposal-card">
    <div class="proposal-card__head">
      <p class="proposal-card__id">
        <span>{{ $t('proposal-card.id') }}</span>
        <span>{{ proposal!.id }}</span>
      </p>
      <tag v-if="proposal!.status" :state="statusState" :class-name="``">
        <!-- eslint-disable-next-line vue-i18n/no-dynamic-keys -->
        {{ $t(proposalStatus) }}
      </tag>
    </div>
    <h2 class="proposal-card__title" :title="$t('proposal-card.title')">
      {{ $t('proposal-card.title') }}
    </h2>
    <div class="proposal-card__voting">
      <div class="proposal-card__voting__quorum">
        <div class="proposal-card__quorum__current">
          {{ $t('proposal-card.quorum') + ' ' + formatPercent(singlePrecision(currentQuorum)) }}
        </div>
        <div class="proposal-card__quorum__left">
          {{ $t('proposal-card.left-quorum') + ' ' + formatPercent(singlePrecision(leftQuorum)) }}
        </div>
      </div>
      <progress-tab
        :value="Number(singlePrecision(currentQuorum))"
        :max="Number(singlePrecision(requiredQuorum))"
        :track-color="trackColor"
        :value-color="valueColor"
        class="proposal-card__voting__progress"
      />
      <voting-periods :proposal="proposal" />
    </div>
  </router-link>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import { LocationAsRelativeRaw } from 'vue-router'

import { trackColor, valueColor } from '@/constants'

import Tag from '@/components/utils/Tag.vue'

import { ProposalBaseInfo } from '@/types/proposals'
import { bigIntMax, getCurrentQuorum, getStatusState, TagState } from '@/utils/proposals'
import { formatPercent, singlePrecision } from '@/helpers'

import ProgressTab from '@/components/utils/ProgressTab.vue'
import VotingPeriods from '@/components/utils/VotingPeriods.vue'
import { getProposalStatus, ProposalStatus } from '@/gateway/proposals'

const props = withDefaults(
  defineProps<{
    proposal?: ProposalBaseInfo
    route?: LocationAsRelativeRaw
  }>(),
  {
    proposal: undefined,
    route: undefined,
  },
)

const proposalStatus = ref<ProposalStatus>('voting-status.none')
const statusState = ref<TagState>('none')

const proposalLink = computed(() => `/governance/proposal/${props.proposal!.id}`)

const currentQuorum = computed(() => getCurrentQuorum(props.proposal!))
const requiredQuorum = computed(() => props.proposal!.params.requiredQuorum)

const leftQuorum = computed(() => {
  return bigIntMax(requiredQuorum.value - currentQuorum.value, 0n).toString()
})

onMounted(async () => {
  await updateProposalStatus()
})

async function updateProposalStatus() {
  proposalStatus.value = await getProposalStatus(props.proposal!.id)
  statusState.value = getStatusState(proposalStatus.value)
}
</script>

<style lang="scss" scoped>
.proposal-card {
  display: block;
  border: #{toRem(1)} solid var(--secondary-dark);
  background-color: var(--app-button-filled-bg);
  color: var(--app-button-text);
  border-radius: toRem(14);
  padding: var(--field-padding);
  transition:
    background-color 0.3s ease,
    box-shadow 0.3s ease,
    border-color 0.3s ease;

  &:hover,
  &:focus-visible {
    border-color: var(--border-primary-dark);
    box-shadow: 0 toRem(2) toRem(4) rgba(0, 0, 0, 0.1);
  }

  &__head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: toRem(12);
  }

  &__id {
    color: var(--text-primary-light);
    font-size: 0.9rem;
    display: flex;
    gap: toRem(8);
  }

  &__title {
    font-size: toRem(24);
    font-weight: 700;
    color: var(--text-primary-light);
    margin: toRem(0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .proposal-card__voting {
    margin-top: toRem(18);
    display: flex;
    flex-direction: column;
    gap: toRem(8);

    &__quorum {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: toRem(8) 0;

      &__current,
      &__left {
        font-size: toRem(14);
        color: var(--text-secondary-color);
      }
    }

    &__progress {
      width: 100%;
    }
  }
}
</style>
