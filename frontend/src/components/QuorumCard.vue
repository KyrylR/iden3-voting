<template>
  <div class="proposal-quorum">
    <h2 class="proposal-quorum__title" :title="$t('proposal-quorum.title')">
      {{ $t('proposal-quorum.title') }}
    </h2>
    <div class="proposal-quorum__voting">
      <div class="proposal-quorum__voting__header__quorum">
        <div class="proposal-quorum__voting__header__quorum__current">
          {{
            $t('proposal-quorum.quorum') +
            ' ' +
            formatPercent(singlePrecision(currentQuorum))
          }}
        </div>
        <div class="proposal-quorum__voting__header__quorum__left">
          {{
            formatPercent(singlePrecision(leftQuorum)) +
            ' ' +
            $t('proposal-quorum.quorum-required')
          }}
        </div>
      </div>
      <progress-tab
        :value="Number(singlePrecision(currentQuorum))"
        :max="Number(singlePrecision(requiredQuorum))"
        class="proposal-quorum__voting__progress"
      />
      <div class="proposal-quorum__voting__footer__quorum">
        <div class="proposal-quorum__voting__footer__quorum__item">
          <span class="proposal-quorum__voting__footer__quorum__label">
            {{ $t('proposal-quorum.voted') }}
          </span>
          <span class="proposal-quorum__voting__footer__quorum__value">
            {{ voted }}
          </span>
        </div>
        <div class="proposal-quorum__voting__footer__quorum__item">
          <span class="proposal-quorum__voting__footer__quorum__label">
            {{ $t('proposal-quorum.not-voted') }}
          </span>
          <span class="proposal-quorum__voting__footer__quorum__value">
            {{ notVoted }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'

import { ProposalBaseInfo } from '@/types/proposals'
import { bigIntMax, getCurrentQuorum } from '@/utils/proposals'
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

const commitments = computed(() => props.proposal!.counters.commitments)
const voted = computed(
  () =>
    props.proposal!.counters.votedFor + props.proposal!.counters.votedAgainst,
)
const notVoted = computed(() => commitments.value - voted.value)

const currentQuorum = computed(() => getCurrentQuorum(props.proposal!))
const requiredQuorum = computed(() => props.proposal!.params.requiredQuorum)

const leftQuorum = computed(() => {
  return bigIntMax(requiredQuorum.value - currentQuorum.value, 0n).toString()
})
</script>

<style lang="scss" scoped>
.proposal-quorum {
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
    margin-bottom: toRem(12);
  }

  &__id {
    color: var(--text-primary-light);
    font-size: toRem(16);
    display: flex;
    gap: toRem(8);
  }

  &__title {
    font-size: toRem(24);
    font-weight: 700;
    color: var(--text-primary-light);
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
      &__quorum {
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
      &__quorum {
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
