<template>
  <div class="page-header">
    <VanPopover v-model:show="showLogin" placement="bottom-start" className="page-login-warp">
      <div class="user-info">
        <div class="user-login">
          <div class="avator">
            <img src="@src/assets/icons/svg/header/user.svg" alt="" />
          </div>
          <VanField v-model="password" type="password" placeholder="密码" v-if="!isAuth"></VanField>
          <VanButton type="primary" block class="login-btn" @click="logOut" v-else>登出</VanButton>
        </div>
        <VanButton type="primary" block class="login-btn mt16" @click="login" v-if="!isAuth"
          >登录</VanButton
        >
        <div class="device-list">
          <div class="device-item van-hairline--bottom" v-for="item in adapterList" :key="item.id">
            <div :class="item.connect_status === 1 ? 'online' : 'offline'"></div>
            <span>{{ item.name }}</span>
          </div>
        </div>
      </div>
      <template #reference>
        <div class="page-avator">
          <div class="user"></div>
          <div class="connect_status"></div>
        </div>
      </template>
    </VanPopover>
    <div class="page-breadcrumb">
      <span v-for="item in matcheds" :key="item.path" @click="go(item)">{{
        t(item.meta.title)
      }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed, getCurrentInstance } from 'vue'
import routes from '@src/router/router'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useSysStore } from '@src/store/useSys'
import { invoke } from '@tauri-apps/api/tauri'

const { proxy } = getCurrentInstance()

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { sysInfo, adapterList } = storeToRefs(useSysStore())
const isAuth = computed(() => sysInfo.value.role === 'admin')

const matcheds = ref([])
watch(
  () => route,
  () => {
    getRouterMap()
  },
  {
    deep: true
  }
)

onMounted(() => {
  getRouterMap()
})

const showLogin = ref(false)
const password = ref('')

const login = async () => {
  const res = await invoke('change_role', { data: { role: 'admin', password: password.value } })
  if (res === 'success') {
    sysInfo.value.role = 'admin'
    password.value = ''
    showLogin.value = false
    proxy.$alertMsg('check', '', '登录成功', { type: 'success' })
  } else {
    proxy.$alertMsg('clear', '', '密码错误', { type: 'danger' })
  }
}

const logOut = async () => {
  const res = await invoke('change_role', { data: { role: 'user', password: '' } })
  if (res === 'success') {
    sysInfo.value.role = ''
    showLogin.value = false
    proxy.$alertMsg('check', '', '登出成功', { type: 'success' })
  }
}

const go = item => {
  router.push(item.path)
}

const getRouterMap = () => {
  const routesMap = routes.find(d => d.name === 'home').children
  const path = route.path
  deep(path, routesMap, [])
  matcheds.value = matcheds.value.filter(item => item.level !== 0)
}

const deep = (path, arr, barr) => {
  arr.forEach(item => {
    if (item.children) {
      deep(path, item.children, barr)
    } else if (path.includes(item.path)) {
      if (item.path === '/record/history') {
        //特殊处理
        item.meta = route.meta
      }
      barr.push(item)
      matcheds.value = barr
    }
  })
}
</script>

<style lang="scss" scoped>
.page-header {
  height: 45px;
  line-height: 45px;
  font-size: 24px;
  display: flex;
  justify-content: space-between;
  gap: 16px;
  :deep(.van-popover__wrapper) {
    .page-avator {
      width: 72px;
      height: 100%;
      padding: 0 8px;
      background-color: #fff;
      border-radius: 2px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      cursor: pointer;
      %images {
        height: 24px;
        width: 24px;
        background-repeat: no-repeat;
        background-size: 100% 100%;
      }
      .user {
        @extend %images;
        background-image: url('@src/assets/icons/svg/header/user.svg');
      }
      .connect_status {
        @extend %images;
        background-image: url('@src/assets/icons/svg/header/connect_status.svg');
      }
    }
  }
  .page-breadcrumb {
    flex: 1;
    padding: 0 24px;
    background-color: rgb(241, 241, 241);
    border-radius: 2px;
    span {
      &:not(:last-child) {
        cursor: pointer;
        &::after {
          content: '/';
          margin: 0 8px;
        }
      }
      &:last-child {
        pointer-events: none;
      }
    }
  }
}
</style>
<style lang="scss">
.page-login-warp {
  .user-info {
    padding: 24px 16px;
    width: 312px;
    max-height: 477px;
    overflow: auto;
    background-color: #fff;
    .user-login {
      height: 54px;
      display: flex;
      gap: 8px;
      .avator {
        height: 54px;
        width: 54px;
        background-color: rgb(241, 241, 241);
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }
    .login-btn {
      height: 54px;
      font-size: 24px;
    }
    .device-list {
      margin-top: 40px;
      .device-item {
        height: 62px;
        display: flex;
        align-items: center;
        gap: 8px;
        &:not(:first-child) {
          margin-top: 8px;
        }
        &:last-child::after {
          border: none;
        }
        .offline {
          height: 24px;
          width: 24px;
          background-repeat: no-repeat;
          background-size: 100% 100%;
          background-image: url('@src/assets/icons/svg/header/offline.svg');
        }
        .online {
          height: 24px;
          width: 24px;
          background-repeat: no-repeat;
          background-size: 100% 100%;
          background-image: url('@src/assets/icons/svg/header/connect_status.svg');
        }
      }
    }
  }
}
</style>
