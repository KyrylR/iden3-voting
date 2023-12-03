<template>
  <div class="progress" v-bind="$attrs">
    <div class="progress__track" :style="{ backgroundColor: props.trackColor }">
      <div
        class="progress__track__value"
        :style="{ width: valuePercentage, backgroundColor: props.valueColor }"
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
    trackColor: string
    valueColor: string
  }>(),
  {
    value: 0,
    max: 100,
    trackColor: '#e0e0e0',
    valueColor: '#4caf50',
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
