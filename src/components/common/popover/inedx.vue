<template>
  <div class="strategypopover">
    <van-popover
      v-model:show="showValue"
      :placement="placement"
      :close-on-click-outside="false"
      :close-on-click-overlay="false"
      :overlay="true"
      overlay-class="overlay"
      :trigger="trigger ? 'click' : 'manual'"
    >
      <div className="strategyText">
        <div className="header">
          <span>提示</span>
          <span @click="() => (showValue = false)">
            <van-icon name="cross" />
          </span>
        </div>
        <div className="contText">{{ contentText }}</div>
        <div className="btn">
          <slot name="operate"></slot>
        </div>
      </div>
      <template #reference>
        <div className="strategysave">
          <div>
            <span v-if="trigger">{{ trigger }}</span>
            <slot name="referBtn" v-else></slot>
          </div>
        </div>
      </template>
    </van-popover>
  </div>
</template>

<script setup lang="jsx">
import { ref } from 'vue'
defineProps({
  placement: {
    type: String,
    default: 'left-start'
  },
  contentText: {
    type: String,
    default: ''
  },
  trigger: {
    type: String,
    default: ''
  }
})
const showValue = ref(false)
const cancelFun = () => {
  showValue.value = false
}
const confirmFun = () => {
  showValue.value = false
}
const handleClick = () => {
  showValue.value = !showValue.value
}
defineExpose({
  cancelFun,
  confirmFun,
  handleClick
})
</script>

<style lang="scss">
.overlay {
  background-color: rgba(0, 0, 0, 0);
}
.strategysave {
  color: white !important;
  font-size: 24px;
  width: 200px;
  height: 72px;
  text-align: center;
  line-height: 72px;
  & > div {
    height: 100%;
  }
}
.van-popover__content {
  border-radius: 4px;
}
.strategyText {
  width: 312px !important;
  .header {
    display: flex;
    justify-content: space-between;
    font-size: 22px;
    padding: 8px 16px 0px;
    & > :last-child {
      cursor: pointer;
    }
  }
  .contText {
    font-size: 22px;
    text-align: center;
    margin-top: 25px;
    padding-bottom: 45px;
    border-bottom: 1px solid rgba(126, 134, 142, 0.16);
  }
  .btn {
    display: flex;
    justify-content: space-around;
    height: 64px;
    span {
      display: inline-block;
      font-size: 24px;
      font-weight: 500;
      height: 64px;
      line-height: 64px;
      width: calc(312px / 2);
      text-align: center;
      cursor: pointer;
    }
    & > :first-child {
      border-right: 1px solid rgba(126, 134, 142, 0.16);
      color: rgba(40, 45, 67, 0.5);
    }
    & > :last-child {
      color: rgb(66, 180, 210);
    }
  }
}
</style>
