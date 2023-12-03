<template>
  <div class="create-proposal-page">
    <app-button
      class="create-proposal-page__back-button"
      size="medium"
      scheme="flat"
      :icon-left="$icons.arrowLeft"
      :text="$t('voting-details.back-btn')"
      @click="() => $router.back()"
    />

    <div class="block">
      <header class="create-proposal-page__header">
        <div class="create-proposal-page__header__info">
          <h1 class="create-proposal-page__header__info__title">
            {{ $t('create-voting-header.title') }}
          </h1>
        </div>

        <app-button
          class="create-proposal-page__header__submit-button"
          size="medium"
          modification="border-rounded"
          color="success"
          :icon-left="$icons.star"
          :scheme="'filled'"
          :text="$t('create-voting-header.submit')"
          @click="handleSubmitButtonClick"
        />
      </header>

      <div class="create-proposal-page__form">
        <div class="create-proposal-page__form__title">
          {{ $t('create-voting-form.remark-input') }}
        </div>
        <input-field
          v-model="form.input"
          :label="'label'"
          :placeholder="'placeholder'"
          class="create-proposal-page__form__input"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { reactive } from 'vue'

import AppButton from '@/common/AppButton.vue'
import InputField from '@/fields/InputField.vue'
import { bus, BUS_EVENTS } from '@/helpers'

const form = reactive({
  input: '',
})

function handleSubmitButtonClick() {
  bus.emit(BUS_EVENTS.success, 'Submit button clicked')
}
</script>

<style lang="scss" scoped>
.block {
  display: flex;
  flex-direction: column;
  background-color: #fff; // Light background for the stats card
  border-radius: toRem(12);
  box-shadow: 0 toRem(3) toRem(2) rgba(var(--black-rgb), 0.3),
    0 toRem(2) toRem(6) toRem(2) rgba(var(--black-rgb), 0.15);
  padding: toRem(0) toRem(36);
  margin: toRem(36) toRem(36);
}

.create-proposal-page {
  padding: toRem(24) var(--voting-app-padding-left) toRem(36)
    var(--voting-app-padding-right);

  &__back-button {
    font-size: toRem(15);
  }

  &__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: toRem(0);
    padding: toRem(24) toRem(0) toRem(0) toRem(0);
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
        word-wrap: break-word;
      }
    }

    &__submit-button {
      font-size: toRem(15);
      color: var(--text-color);
    }
  }

  &__form {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    gap: toRem(20);
    padding: toRem(18) toRem(0) toRem(24) toRem(0);
    background: var(--background-primary-main);
    border-bottom: var(--border-primary-main);
    margin-top: toRem(18);

    &__title {
      font-size: toRem(24);
    }

    &__input {
      width: 100%;
      font-size: toRem(20);
    }
  }
}
</style>
