import { createRouter, createWebHashHistory } from 'vue-router'
import routerMap from './router'

export default createRouter({
  history: createWebHashHistory(),
  routes: routerMap
})
