<template>
  <div class="proposal-majority">
    <h2 class="proposal-majority__title" :title="$t('proposal-majority.title')">
      {{ $t('proposal-majority.title') }}
    </h2>
    <div class="proposal-majority__voting">
      <div class="proposal-majority__voting__header__majority">
        <div class="proposal-majority__voting__header__majority__current">
          {{ $t('proposal-majority.requirement') + ' > ' + formatPercent(singlePrecision(requiredMajority)) }}
        </div>
      </div>
      <progress-tab
        :value="Number(votedFor)"
        :max="Number(combinedVotes)"
        :track-color="trackColor"
        class="proposal-majority__voting__progress"
      />
      <div class="proposal-majority__voting__footer__majority">
        <div class="proposal-majority__voting__footer__majority__item">
          <span class="proposal-majority__voting__footer__majority__label">
            {{ $t('proposal-majority.voted-for') }}
          </span>
          <span class="proposal-majority__voting__footer__majority__value">
            {{ votedFor }}
          </span>
        </div>
        <div class="proposal-majority__voting__footer__majority__item">
          <span class="proposal-majority__voting__footer__majority__label">
            {{ $t('proposal-majority.voted-against') }}
          </span>
          <span class="proposal-majority__voting__footer__majority__value">
            {{ votedAgainst }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'

import { ProposalBaseInfo } from '@/typings/proposals'
import { formatPercent, singlePrecision } from '@/helpers'

import ProgressTab from '@/components/utils/ProgressTab.vue'

const props = withDefaults(
  defineProps<{
    proposal?: ProposalBaseInfo
  }>(),
  {
    proposal: undefined,
  },
)

const trackColor = '#f72828'

const votedFor = computed(() => props.proposal!.counters.votedFor)
const votedAgainst = computed(() => props.proposal!.counters.votedAgainst)
const combinedVotes = computed(() => votedFor.value + votedAgainst.value)

const requiredMajority = computed(() => props.proposal!.params.requiredMajority)
</script>

<style lang="scss" scoped>
.proposal-majority {
  display: block;
  border: #{toRem(1)} solid var(--secondary-dark);
  background-color: var(--app-button-filled-bg);
  color: var(--app-button-text);
  border-radius: toRem(14);
  padding: var(--field-padding);

  &__head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  &__id {
    color: #757575;
    font-size: 0.9rem;
    display: flex;
    gap: 8px;
  }

  &__title {
    font-size: toRem(24);
    font-weight: bold;
    color: #424242;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  &__voting {
    margin-top: toRem(18);
    display: flex;
    flex-direction: column;
    gap: toRem(18);

    &__header {
      &__majority {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: toRem(8) 0;

        &__current,
        &__left {
          font-size: toRem(16);
          color: var(--text-secondary-color);
        }
      }
    }

    &__footer {
      &__majority {
        display: grid;
        grid-template-columns: max-content auto;
        align-items: center;
        gap: toRem(20);

        &__item {
          display: contents;
        }

        &__label {
          font-size: toRem(16);
          color: var(--text-secondary-color);
          padding-right: toRem(8);
        }

        &__value {
          font-size: toRem(16);
          color: var(--text-primary-color);
          text-align: right;
        }
      }
    }

    &__progress {
      width: 100%;
    }
  }
}
</style>
