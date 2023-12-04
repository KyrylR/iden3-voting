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
  padding: toRem(8) toRem(8);
  border-radius: toRem(4);
  display: flex;
  text-align: justify;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  background-color: var(--background-primary-main);
  color: var(--text-primary-main);

  &.pending {
    background-color: var(--warning-main);
    color: var(--black);
  }

  &.commitment {
    background-color: var(--info-main);
    color: var(--black);
  }

  &.execution {
    background-color: var(--success-light);
    color: var(--black);
  }

  &.rejected {
    background-color: var(--error-main);
    color: var(--black);
  }

  &.executed {
    background-color: var(--success-main);
    color: var(--black);
  }
}
</style>
