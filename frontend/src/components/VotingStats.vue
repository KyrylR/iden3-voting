<template>
  <div class="voting-stats">
    <div class="voting-stats__header">
      <h2 class="voting-stats__header__title">
        {{ $t('voting-stats.title') }}
      </h2>
      <div class="voting-stats__header__generate">
        <app-button
          class="voting-stats__header__generate"
          size="medium"
          modification="border-rounded"
          color="success"
          :scheme="'filled'"
          :text="$t('voting-stats.generate-secrets')"
          @click="handleGenerateButtonClick"
        />
      </div>
    </div>
    <div class="voting-stats__details">
      <div class="voting-stats__details__item">
        <span class="voting-stats__details__item__label">{{
          $t('voting-stats.total-secrets-number')
        }}</span>
        <span class="voting-stats__details__item__value">
          {{ totalSecretsNumber }}
        </span>
      </div>
      <div class="voting-stats__details__divider"></div>
      <div class="voting-stats__details__item">
        <span class="voting-stats__details__item__label">{{
          $t('voting-stats.active-secrets-number')
        }}</span>
        <span class="voting-stats__details__item__value">
          {{ activeSecretsNumber }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppButton from '@/common/AppButton.vue'
import { bus, BUS_EVENTS } from '@/helpers'

const totalSecretsNumber = 0
const activeSecretsNumber = 0

function handleGenerateButtonClick() {
  bus.emit(BUS_EVENTS.success, 'Generate button clicked')
}
</script>

<style lang="scss" scoped>
.voting-stats {
  display: flex;
  flex-direction: column;
  background-color: #fff; // Light background for the stats card
  border-radius: toRem(12);
  box-shadow: 0 toRem(3) toRem(2) rgba(var(--black-rgb), 0.3),
    0 toRem(2) toRem(6) toRem(2) rgba(var(--black-rgb), 0.15);
  margin: toRem(4) var(--voting-app-padding-left) toRem(4)
    var(--voting-app-padding-right);

  &__header {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    padding: toRem(24) toRem(24);

    &__title {
      font-size: toRem(24);
    }

    &__generate {
      font-size: toRem(16);
      color: #000000;
    }
  }

  &__details {
    display: flex;
    justify-content: space-evenly;
    padding: toRem(18) toRem(36);
    gap: toRem(24);

    &__divider {
      height: toRem(48);
      width: toRem(1);
      background-color: #000000;
    }

    &__item {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      justify-content: center;
      gap: toRem(12);

      &__label {
        font-size: toRem(16);
        color: #333;
      }

      &__value {
        font-size: toRem(18);
        font-weight: 600;
        color: #333;
      }
    }
  }
}
</style>
