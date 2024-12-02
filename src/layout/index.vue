<template>
  <div class="page-warp">
    <div class="page-container">
      <myHeader></myHeader>
      <div class="page-main">
        <div class="page-view">
          <router-view></router-view>
        </div>
        <myAside></myAside>
      </div>
    </div>
    <myFooter></myFooter>
  </div>
</template>

<script setup>
import myHeader from './header.vue'
import myFooter from './footer.vue'
import myAside from './aside.vue'
</script>

<style lang="scss" scoped>
.page-warp {
  height: 100%;
  overflow: hidden;
  .page-container {
    height: calc(100% - 80px);
    overflow: hidden;
    padding: 24px;
    .page-main {
      height: calc(100% - 48px);
      display: grid;
      grid-template-columns: 1fr 200px;
      gap: 24px;
      .page-view {
        height: 100%;
        overflow: auto;
        padding-top: 24px;
        &::-webkit-scrollbar {
          display: none;
        }
      }
    }
  }
}
</style>
