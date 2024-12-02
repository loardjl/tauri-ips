import { defineStore } from 'pinia'
import piniaPersistConfig from '@src/common/piniaPersist'
import { computed } from 'vue'
import { useSysStore } from './useSys'

export const useMenuStore = defineStore('useMenu', {
  state: () => ({
    asideList: []
  }),
  actions: {
    setAsideList(list) {
      this.asideList = list
    },
    clearAsideList() {
      this.asideList = new Array(6).fill({}).map(() => ({}))
    },
    getAsideList() {
      const sysStore = useSysStore()
      return computed(() => {
        const list = new Array(6).fill({}).map(() => ({}))
        this.asideList.forEach((d, index) => {
          if (d.sort) {
            list[d.sort] = d
          } else {
            list[index] = d
          }
        })
        list.forEach((d, index) => {
          if (d.auth && d.auth !== '*' && d.auth !== sysStore.getSysInfo()?.role) {
            list[index] = {}
          }
        })
        return list
      })
    }
  },
  persist: piniaPersistConfig('useMenu')
})
