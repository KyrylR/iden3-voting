<template>
  <header class="header">
    <div class="header__info">
      <h1 class="header__info__title">
        {{ '#' + props.proposalId + ' ' + $t('voting-details-header.title') }}
      </h1>

      <tag v-if="proposalStatus" :state="proposalStatus" :class-name="``">
        <!-- eslint-disable-next-line vue-i18n/no-dynamic-keys -->
        {{ $t(proposalStatus) }}
      </tag>
    </div>

    <app-button
      class="header__vote-button"
      size="medium"
      modification="border-rounded"
      color="success"
      :icon-left="$icons.cube"
      :scheme="'filled'"
      :text="$t('voting-details-header.vote')"
      @click="handleVoteButtonClick"
    />
  </header>
</template>

<script lang="ts" setup>
import AppButton from '@/common/AppButton.vue'

import { bus, BUS_EVENTS } from '@/helpers'
import Tag from '@/components/utils/Tag.vue'
import { TagState } from '@/utils/proposals'

const props = withDefaults(
  defineProps<{
    proposalId: bigint
    proposalStatus: TagState
  }>(),
  {
    proposalId: undefined,
    proposalStatus: undefined,
  },
)

function handleVoteButtonClick() {
  bus.emit(BUS_EVENTS.success, 'Vote button clicked')
}
</script>

<style lang="scss" scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: toRem(24);
  padding: toRem(36) var(--voting-app-padding-left) toRem(36) var(--voting-app-padding-right);
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
    color: #000000;
    width: toRem(150);
    font-size: toRem(15);
  }
}
</style>
