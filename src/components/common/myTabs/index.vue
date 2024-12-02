<template>
  <div class="tabs-wrap">
    <div class="tabs flex-start">
      <div
        class="tabs-item"
        v-for="item in tabList"
        :key="item[itemName.value || 'value']"
        :class="item[itemName.value || 'value'] === tabSelected ? 'active' : ''"
        @click="handleChangeTab(item)"
      >
        {{ item[itemName.label || 'label'] }}
      </div>
    </div>
  </div>
</template>
<script setup>
defineProps({
  tabList: {
    type: Array,
    default: () => []
  },
  tabSelected: {
    type: [String, Number],
    default: ''
  },
  itemName: {
    type: Object,
    default: () => ({
      label: 'label',
      value: 'value'
    })
  }
})
const emit = defineEmits(['changeTab'])
const handleChangeTab = item => {
  emit('changeTab', item)
}
</script>
<style lang="scss" scoped>
.tabs-wrap {
  width: 100%;
  background: $base-page-bg-color;
  .tabs {
    display: inline-flex;
    border-radius: 6px 6px 0 0;
    overflow: hidden;
  }
  .tabs-item {
    width: 143px;
    height: 72px;
    line-height: 70px;
    margin-left: 1px;
    border: 1px solid $base-block-bg-color;
    font-size: 24px;
    font-weight: 400;
    text-align: center;
    background: $base-page-bg-color;
    cursor: pointer;
    &:first-child {
      border-radius: 6px 0 0 0;
      margin-left: 0;
    }
    &:last-child {
      border-radius: 0 6px 0 0;
      margin-left: 0;
    }
    &.active {
      background: $base-block-bg-color;
    }
  }
}
</style>
