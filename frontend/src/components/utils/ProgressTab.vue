<template>
  <div class="progress" v-bind="$attrs">
    <div class="progress__track" :style="{ backgroundColor: '#e0e0e0' }">
      <div
        class="progress__track__value"
        :style="{ width: valuePercentage, backgroundColor: '#4caf50' }"
      ></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    value: number
    max: number
  }>(),
  {
    value: 0,
    max: 100,
  },
)

const valuePercentage = computed(() => {
  const validValue = Math.min(props.value, props.max)
  return `${(validValue / props.max) * 100}%`
})
</script>

<style lang="scss" scoped>
.progress {
  width: 100%;
  height: toRem(6);
  background-color: #e0e0e0;
  border-radius: toRem(2);
  overflow: hidden;

  &__track {
    width: 100%;
    height: 100%;
    background-color: inherit;
    border-radius: inherit;

    &__value {
      height: 100%;
      background-color: #4caf50;
      transition: width 0.3s ease;
    }
  }
}
</style>
