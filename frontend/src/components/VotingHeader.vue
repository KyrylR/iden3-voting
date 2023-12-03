<template>
  <header class="header">
    <h1 class="header__title">
      {{ $t('voting-header.title') }}
    </h1>

    <app-button
      class="header__create-proposal-button"
      size="medium"
      modification="border-rounded"
      color="success"
      :icon-left="$icons.plus"
      :scheme="'filled'"
      :text="$t('voting-header.create-proposal')"
      :route="{ name: $routes.createProposal }"
    />
  </header>
</template>

<script lang="ts" setup>
import AppButton from '@/common/AppButton.vue'
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
}

.header__title {
  font-size: toRem(36);
}

.header__create-proposal-button {
  color: #000000;
  font-size: toRem(15);
}
</style>
