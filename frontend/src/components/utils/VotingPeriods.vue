<template>
  <div class="voting-container" v-bind="$attrs">
    {{ $t('proposal-card.voting-end-time') + ': ' + formattedVotingEndTime }}
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

import { ProposalBaseInfo } from '@/types/proposals'
import { unixToDate, formatDate } from '@/utils/date'

const props = withDefaults(
  defineProps<{
    proposal?: ProposalBaseInfo
  }>(),
  {
    proposal: undefined,
  },
)

const { t } = useI18n()

const votingEndTime = computed(() => unixToDate(props.proposal!.params.votingEndTime).getTime())

const formattedVotingEndTime = computed(() => formatDate(String(votingEndTime.value), t('common.date-format')))
</script>

<style lang="scss" scoped>
.voting-container {
  display: flex;
  justify-content: space-between;
  margin-top: toRem(8);
}
</style>
