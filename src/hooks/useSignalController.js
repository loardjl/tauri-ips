import { onUnmounted, inject } from 'vue'
import { useMenuStore } from '@src/store/useMenu'
export const useSignalController = () => {
  const menuStore = useMenuStore()
  const _worker = inject('_worker')
  const worker = new Worker(_worker)
  onUnmounted(() => {
    worker.destory()
    menuStore.clearAsideList()
  })
  return { signal: worker.signal, _worker, worker }
}

class Worker {
  constructor(_worker) {
    this._worker = _worker
    this.controller = new AbortController()
    this.signal = this.controller.signal
    this.msgArr = []
    this._init()
  }
  _init() {
    this._worker.addEventListener(
      'message',
      e => {
        this.msgArr.forEach(d => {
          if (d.msg === e.data.type) {
            d.cb(e.data)
          }
        })
      },
      { signal: this.signal }
    )
  }
  destory() {
    this.msgArr = []
    this.controller.abort()
  }
  send(msg, data) {
    this._worker.postMessage({ type: msg, payload: JSON.parse(JSON.stringify(data)) })
  }
  dispatch(msg, cb) {
    const msgRoute = this.msgArr.find(d => d.msg === msg)
    if (msgRoute) {
      msgRoute.cb = cb
      return
    }
    this.msgArr.push({ msg, cb })
  }
}
