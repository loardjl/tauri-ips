/*
 * @Author: 何志祥
 * @Date: 2023-10-11 16:09:06
 * @LastEditors: 何志祥
 * @LastEditTime: 2023-10-11 17:34:14
 * @Description: 采集器信号全局信息储存
 */

import { defineStore } from 'pinia'

export const useStoreSignal = defineStore('store-signals', {
  state: () => {
    return {
      signalEnumList: null // 信号枚举
    }
  },
  actions: {
    // 更新信号值
    changeSignalList(val) {
      this.signalEnumList = val
      sessionStorage.setItem('signalEnumList', JSON.stringify(val))
    }
  }
})
