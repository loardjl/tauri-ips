<template>
  <div>
    <van-field
      :label="label"
      :name="name"
      v-model="inputValue"
      readonly
      type="number"
      @click="showPicker = true"
      :rules="rules"
      :disabled="disabled"
    />
    <van-popup v-model:show="showPicker" round position="bottom">
      <van-picker
        :title="title"
        option-height="54px"
        :columns="columns"
        @cancel="showPicker = false"
        @confirm="onConfirm"
        v-model="integerNumArr"
      />
    </van-popup>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
const showPicker = ref(false)
const props = defineProps({
  modelValue: {
    type: [Number, String],
    default: 0
  },
  title: {
    type: [String],
    default: '标题'
  },
  label: {
    type: [String],
    default: ''
  },
  name: {
    type: [String],
    default: ''
  },
  rules: {
    type: Array
  },
  disabled: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue'])

const inputValue = computed({
  get: () => Number(props.modelValue) || 0,
  set: val => {
    padNumber(inputValue.value)
    emit('update:modelValue', Number(val))
  }
})

const integerNumArr = ref([])
const columns = [
  [
    { text: '0', value: '0' },
    { text: '1', value: '1' },
    { text: '2', value: '2' },
    { text: '3', value: '3' },
    { text: '4', value: '4' },
    { text: '5', value: '5' },
    { text: '6', value: '6' },
    { text: '7', value: '7' },
    { text: '8', value: '8' },
    { text: '9', value: '9' }
  ],
  [
    { text: '0', value: '0' },
    { text: '1', value: '1' },
    { text: '2', value: '2' },
    { text: '3', value: '3' },
    { text: '4', value: '4' },
    { text: '5', value: '5' },
    { text: '6', value: '6' },
    { text: '7', value: '7' },
    { text: '8', value: '8' },
    { text: '9', value: '9' }
  ]
]
const onConfirm = ({ selectedOptions }) => {
  showPicker.value = false
  integerNumArr.value = [selectedOptions[0].value, selectedOptions[1].value]
  inputValue.value = Number(selectedOptions[0].value + '.' + selectedOptions[1].value)
}
function padNumber(num) {
  const str = num.toString().split('')
  integerNumArr.value = [str[0], str[2]]
}
padNumber(inputValue.value)
</script>

<style lang="scss" scoped>
:deep(.van-cell) {
  height: 64px;
  display: flex;
  align-items: center;
  .van-field__control {
    font-size: 22px;
  }
}
:deep(.van-popup--bottom.van-popup--round) {
  border-radius: 4px;
}
:deep(.van-picker__cancel) {
  font-size: 22px;
  width: 100px;
}
:deep(.van-picker__confirm) {
  font-size: 22px;
  width: 100px;
}
:deep(.van-picker-column__item) {
  font-size: 22px;
}
:deep(.van-picker__title) {
  font-size: 22px;
  font-weight: normal;
}
</style>
