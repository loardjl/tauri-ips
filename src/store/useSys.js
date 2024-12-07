import { defineStore } from 'pinia'
import piniaPersistConfig from '@src/common/piniaPersist'
export const useSysStore = defineStore('useSys', {
  state: () => ({
    sysInfo: {},
    devId: 0,
    adapterList: []
  }),
  actions: {
    setSysInfo(info) {
      this.sysInfo = info
    },
    getSysInfo() {
      return this.sysInfo
    },
    setDevId(devId) {
      this.devId = devId
    },
    setAdapterList(list) {
      this.adapterList = list
    }
  },
  persist: piniaPersistConfig('useSys')
})
