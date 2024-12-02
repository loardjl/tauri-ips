<template>
  <div class="page-footer">
    <div
      :class="item.key === activeKey && activeKey !== 'logo' ? 'footer-item active' : 'footer-item'"
      @click="handleFooterClick(item)"
      v-for="item in footerList"
      :key="item.key"
    >
      <div class="images">
        <img :src="item.fullImg" class="full" v-if="item.fullImg" />
        <i :class="item.img" v-else></i>
      </div>
      <span class="name">{{ item.text }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import logoPng from '@src/assets/icons/svg/footer/logo.png'
import { useRouter } from 'vue-router'
const router = useRouter()
const activeKey = ref('home')
const footerList = ref([
  {
    key: 'home',
    active: false,
    text: '综合看板',
    img: 'icon-dashboard',
    route: '/dashboard'
  },
  {
    key: 'history',
    active: false,
    text: '历史数据',
    img: 'icon-history',
    route: '/record'
  },
  {
    key: 'strategy',
    active: false,
    text: '提效策略',
    img: 'icon-strategy',
    route: '/strategy'
  },
  {
    key: 'logo',
    active: false,
    fullImg: logoPng,
    route: '/collector'
  }
])
onMounted(() => {
  handleFooterClick(footerList.value[0])
})

const handleFooterClick = item => {
  activeKey.value = item.key
  if (item.route) {
    router.push(item.route)
  }
}
</script>

<style lang="scss" scoped>
.page-footer {
  height: 80px;
  background-color: rgb(59, 61, 76);
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 1;
  .footer-item {
    z-index: 2;
    &:first-child {
      margin-left: 94px;
    }
    height: 100%;
    width: 231px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 40px;
    font-weight: 400;
    color: #fff;
    transition: all 0.3s ease-in;
    cursor: pointer;
    &:has(.full) {
      padding: 0 32px;
    }
    .images {
      position: relative;
      z-index: 2;
      img {
        width: 32px;
        height: 32px;
      }
      .full {
        width: 100%;
        height: 100%;
      }
      i:before {
        width: 32px;
        height: 32px;
        position: absolute;
        transform: translate(0, -50%);
      }
    }
    .name {
      font-size: 24px;
      white-space: nowrap;
    }
    &.active {
      height: 130%;
      background-color: rgb(66, 180, 210);
      border-top-left-radius: 8px;
      border-top-right-radius: 8px;
      font-weight: 500;
    }
  }
}
</style>
