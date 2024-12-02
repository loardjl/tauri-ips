import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import '@vant/touch-emulator'
import VxeUITable from 'vxe-table'
import 'vxe-table/lib/style.css'

import './styles/index.scss'
import './permission'
import App from './App.vue'
import router from './router'
import { errorHandler } from './error'
import * as echarts from 'echarts'
import { localforage } from '@src/common/database'
import { i18n } from './i18n'
import myInputDireactive from './direactive/myInput'
import filter from '@src/utils/filter'
import alertMsg from '@src/components/common/alertMsg/index'
import zRenderEnv from 'zrender/lib/core/env'
zRenderEnv.touchEventsSupported = true
zRenderEnv.pointerEventsSupported = false
const app = createApp(App)

const worker = new Worker(new URL('./worker/monitor.js', import.meta.url), {
  type: 'module'
})
const filters = {}
Object.keys(filter).forEach(function (key) {
  filters[key] = filter[key]
})
app.config.globalProperties.$filters = {
  ...filters
}
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(router)
app.use(VxeUITable)
app.use(pinia)
app.use(i18n)
errorHandler(app)
app.provide('echarts', echarts)
app.provide('_localforage', localforage)
app.provide('_worker', worker)
app.directive('limitInput', myInputDireactive.myInput)
app.config.globalProperties.$alertMsg = alertMsg
app.mount('#app')
