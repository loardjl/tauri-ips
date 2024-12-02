<template>
  <div class="page-aside-warp">
    <div class="page-aside">
      <div
        :class="item.text && !item.disable ? 'aside-item cando' : 'aside-item'"
        v-for="item in asideList"
        :key="item.key"
        @click="item.cb && item.cb()"
      >
        <component :is="item.render" v-if="item.render"></component>
        <span v-else class="name">{{ item.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useMenuStore } from '@src/store/useMenu'
const menuStore = useMenuStore()
const asideList = menuStore.getAsideList()
</script>

<style lang="scss" scoped>
.page-aside-warp {
  height: 100%;
  overflow: auto;
  &::-webkit-scrollbar {
    display: none;
  }
  .page-aside {
    height: 100%;
    display: grid;
    grid-template-rows: repeat(6, minmax(72px, 1fr));
    gap: 32px;
    padding: 24px 0;
    .aside-item {
      display: flex;
      justify-content: center;
      align-items: center;
      background-color: $base-bg-color-light-1;
      border-radius: 6px;
      pointer-events: none;
      .name {
        color: #fff;
        font-size: 24px;
      }
    }
    .cando {
      background-color: $base-bg-color;
      cursor: pointer;
      pointer-events: unset;
    }
  }
}
</style>
