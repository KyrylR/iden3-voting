<template>
  <div class="voting-details-info">
    <div class="voting-details-info__header">
      <h2 class="voting-details-info__header__title">
        {{ $t('voting-details-info.title') }}
      </h2>
    </div>
    <div class="voting-details-info__details">
      <div class="voting-details-info__details__item">
        <span class="voting-details-info__details__item__label">{{ $t('voting-details-info.remark') + ':' }}</span>
        <span class="voting-details-info__details__item__value">
          {{ props.remark }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    remark?: string
  }>(),
  {
    remark: '',
  },
)
</script>

<style lang="scss" scoped>
.voting-details-info {
  display: flex;
  flex-direction: column;
  background-color: var(--background-primary-main);
  border-radius: toRem(12);
  box-shadow:
    0 toRem(3) toRem(2) rgba(var(--black-rgb), 0.3),
    0 toRem(2) toRem(6) toRem(2) rgba(var(--black-rgb), 0.15);
  margin: toRem(4) var(--voting-app-padding-left) toRem(4) var(--voting-app-padding-right);

  &__header {
    display: flex;
    justify-content: left;
    align-items: flex-start;
    padding: toRem(24) toRem(36);

    &__title {
      font-size: toRem(30);
    }
  }

  &__details {
    display: flex;
    flex-direction: row;
    justify-content: start;
    padding: toRem(8) toRem(36) toRem(24) toRem(36);
    gap: toRem(12);

    &__item {
      display: flex;
      flex-direction: row;
      justify-content: flex-start;
      color: var(--text-primary-dark);
      gap: toRem(18);
    }
  }
}
</style>
