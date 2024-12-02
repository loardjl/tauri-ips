import { defineStore } from 'pinia'
import piniaPersistConfig from '@src/common/piniaPersist'
export const useRouteStore = defineStore('routeStore', {
  state: () => ({
    activeRoute: ''
  }),
  actions: {
    setActiveRoute(route) {
      this.activeRoute = route
    }
  },
  persist: piniaPersistConfig('routeStore')
})
