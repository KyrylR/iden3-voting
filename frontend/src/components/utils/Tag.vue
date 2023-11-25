<template>
  <div :class="['tag', props.state, props.className]" v-bind="$attrs">
    <slot></slot>
  </div>
</template>

<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    state: string
    className: string
  }>(),
  {
    state: '',
    className: '',
  },
)
</script>

<style lang="scss" scoped>
.tag {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  display: inline-block;
  text-align: center;
  font-weight: bold;
  background-color: #f0f0f0; // Default background color
  color: #333; // Default text color

  &.pending {
    background-color: #97cdff;
    color: #333;
  }

  &.execution {
    background-color: #83fe9d;
    color: #333;
  }

  &.commitment {
    background-color: #fedb6a;
    color: #333;
  }
}
</style>
