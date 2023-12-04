<template>
  <div class="not-found-page">
    <app-button
      class="not-found-page__home-button"
      size="large"
      scheme="filled"
      :text="'Home'"
      :icon-left="$icons.home"
      @click="goHome"
    />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

import AppButton from '@/common/AppButton.vue'

import { ROUTE_NAMES } from '@/enums'

const router = useRouter()

const goHome = () => {
  router.push({ name: ROUTE_NAMES.voting })
}
</script>

<style scoped lang="scss">
.not-found-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}
</style>
