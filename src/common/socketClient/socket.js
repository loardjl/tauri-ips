import { io } from 'socket.io-client'

class SocketClient {
  constructor(port) {
    console.log('SocketClient createSocket')
    this.socket = io(`ws://localhost:${port}`, {
      transports: ['websocket']
    })
    this.socket.on('connect', () => {
      console.log('SocketClient Connected')
    })
    this.socket.on('disconnect', () => {
      console.log('SocketClient Connection closed')
    })
    this.socket.on('error', error => {
      throw new Error(error)
    })
    this.socket.on('webEvent', data => {
      console.log('SocketClient Data', data)
    })
  }
  send(event, data) {
    this.socket.emit(event, data)
  }
  on(event, data) {
    this.socket.on(event, data)
  }
  close() {
    this.socket.disconnect()
  }
}
function singelton(className) {
  let instance
  return new Proxy(className, {
    construct(target, args) {
      if (!instance) {
        instance = new target(...args)
      }
      return instance
    }
  })
}

export default singelton(SocketClient)
