import SocketClient from '@src/common/socketClient/socket'
import msgList from './module'

const socket = new SocketClient(25572)

self.addEventListener('message', event => {
  const { type, payload } = event.data
  console.log('============', type, payload)
  socket.send(type, payload)
})

msgList.forEach(item => {
  if (typeof item === 'string') {
    socket.on(item, data => {
      self.postMessage({ type: item, payload: data })
    })
  } else {
    socket.on(item.type, async data => {
      if (item.fn) {
        if (data.error) {
          self.postMessage({ type: item.type, payload: data })
          return
        }
        const cb = await item.fn(data)
        self.postMessage({ type: item.type, payload: cb })
      } else {
        self.postMessage({ type: item.type, payload: data })
      }
    })
  }
})

export default self
