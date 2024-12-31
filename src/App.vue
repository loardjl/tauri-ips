<template>
  <van-config-provider :theme-vars="themVars" theme-vars-scope="global">
    <router-view v-slot="{ Component }">
      <div class="app">
        <keep-alive v-if="route.meta.KeepAlive">
          <component :is="Component" />
        </keep-alive>
        <component v-else :is="Component" />
      </div>
    </router-view>
  </van-config-provider>
</template>

<script setup>
import { onMounted, reactive, ref } from 'vue'
// import { i18n } from './i18n'
import { useRoute } from 'vue-router'
import { useSignalController } from '@src/hooks/useSignalController'
import { useApi } from '@src/hooks/useApi'
import { invoke } from '@tauri-apps/api/tauri'
import { useSysStore } from '@src/store/useSys'
import { storeToRefs } from 'pinia'
const { fetchPostApi } = useApi()
const route = useRoute()
const { sysInfo } = storeToRefs(useSysStore())
// const i18nt = computed(() => i18n.global.messages.value[i18n.global.locale.value].el)
const primaryColor = 'rgb(40, 45, 67)'
const baseColor = 'rgb(66, 180, 210)'

const themVars = reactive({
  primaryColor,
  radioCheckedIconColor: baseColor,
  baseFont: 'AlibabaPuHuiTi',
  switchOnBackground: baseColor
})

const { signal, worker } = useSignalController()

window.addEventListener(
  'keydown',
  e => {
    if (e.key === 'F5') {
      e.preventDefault()
      return
    }
  },
  { signal }
)

onMounted(() => {
  workpieceStrategy()
  getDcStatus()
  getRole()
})
const getRole = async () => {
  const res = await invoke('get_role')
  sysInfo.value.role = res
}

worker.dispatch('DCStatus', () => {
  getDcStatus()
})

const getDcStatus = () => {
  // 数据中心服务连接状态推送
  worker.send('getToken', {})
  worker.dispatch('GetToken', async ({ payload }) => {
    sessionStorage.setItem('token', payload.Ok.token)
    try {
      const res = await fetchPostApi({
        version: '1.0',
        method: 'get_edge_device_info',
        id: '84',
        params: {}
      })
      const { TID } = res.result
      sessionStorage.setItem('TID', TID)
    } catch (error) {
      console.log(error)
    }
  })
}

//全局所有策略配置
const workpieceStrategy = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_all_workpiece_strategy',
        id: '7',
        params: {}
      },
      'ipsbatch'
    )
    const data = res
    console.log(res)

    strategies.value = data.result.strategies
    sessionStorage.setItem('strategies', JSON.stringify(strategies.value))
  } catch (e) {
    console.log(e)
  }
}
const strategies = ref([])
</script>

<style lang="scss">
.app {
  height: 100vh;
  overflow: hidden;
}
</style>
