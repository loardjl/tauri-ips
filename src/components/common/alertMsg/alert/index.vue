<!--
 * @Author: chunlaizhang chunlai.zhang@ujoin-tech.com
 * @Date: 2024-06-04 17:18:11
 * @LastEditors: chunlaizhang
 * @LastEditTime: 2024-06-07 10:57:31
 * @FilePath: \kws\src\renderer\components\common\alertMsg\alert\index.vue
-->
<template>
  <div class="alert-warp">
    <van-notify v-model:show="visible" :type="$attrs.type">
      <van-icon :name="$attrs.icons" style="margin-right: 4px" class="colse" />
      <span class="alert-text">{{ $attrs.title }}</span>
    </van-notify>
  </div>
</template>

<script setup>
import { computed, useAttrs, ref, onMounted } from 'vue'
import { _public } from '@src/utils/common'
const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
})

onMounted(() => {
  document.querySelectorAll('.alert-warp').forEach((item, index) => {
    item.style.top = `${_public.screenPx(10 + index * 60)}px`
  })
})

const attrs = useAttrs()
const emit = defineEmits(['update:modelValue'])
const visible = computed({
  get: () => props.modelValue,
  set: val => emit('update:modelValue', val)
})
// const handleClose = () => {
//   emit('update:modelValue', false)
// }
const color = ref(
  attrs.type === 'danger'
    ? 'rgb(252, 12, 3)'
    : attrs.type === 'success'
    ? 'rgb(43, 193, 85)'
    : attrs.type === 'warning'
    ? '#e6a23c'
    : '#909399'
)
</script>

<style lang="scss" scoped>
.alert-warp {
  --van-notify-success-background: rgb(195, 239, 173);
  --van-notify-danger-background: rgb(250, 205, 205);
  .alert-text {
    font-size: 22px;
    color: v-bind('color');
  }

  :deep(.van-popup--top) {
    top: 24px;
    left: 656px;
    width: 600px !important;
  }
  .colse {
    font-size: 22px;
    color: v-bind('color');
  }
  :deep(.van-notify) {
    padding: 12px;
    display: flex;
    justify-content: start;
  }
}
</style>
