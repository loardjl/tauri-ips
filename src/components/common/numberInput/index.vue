<template>
  <div class="number-input">
    <VanButton type="primary" @click="sub">-</VanButton>
    <VanField v-model="inputValue" v-limitInput:num="[min, max]" :disabled="isDisabled"></VanField>
    <VanButton type="primary" @click="add">+</VanButton>
  </div>
</template>

<script setup>
import { computed } from 'vue'
const props = defineProps({
  modelValue: {
    type: [String, Number],
    default: ''
  },
  max: {
    type: Number,
    default: 99
  },
  min: {
    type: Number,
    default: 1
  },
  isDisabled: {
    type: Boolean,
    default: false
  }
})
const inputValue = computed({
  get: () => props.modelValue ?? props.min,
  set: val => emit('update:modelValue', val)
})
const emit = defineEmits(['update:modelValue', 'input'])
const add = () => {
  if (inputValue.value < props.max && !props.isDisabled) {
    inputValue.value = Number(inputValue.value) + 1
  }
}
const sub = () => {
  if (inputValue.value > props.min && !props.isDisabled) {
    inputValue.value = Number(inputValue.value) - 1
  }
}
</script>

<style lang="scss" scoped>
.number-input {
  height: 64px;
  display: grid;
  grid-template-columns: 1fr 1.5fr 1fr;
  .van-button {
    height: 100%;
    border: none;
    border-radius: 0;
    font-size: 28px;
    font-weight: 400;
  }
  .van-cell {
    padding: 0;
  }
  :deep(.van-field) {
    height: 100%;
    border: none;
    border-radius: 0;
    .van-field__body {
      height: 100%;
      .van-field__control {
        font-size: 28px;
        text-align: center;
      }
    }
  }
}
</style>
