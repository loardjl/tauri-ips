import { defineStore } from 'pinia'
export const useChartStore = defineStore('chartStore', {
  state: () => ({
    echarts: new Map()
  }),
  getters: {
    getEcharts() {
      return this.echarts
    }
  },
  actions: {
    setEcharts(id, chart) {
      this.echarts.set(id, chart)
    },
    clearEcahrts() {
      this.echarts.clear()
    }
  }
})
