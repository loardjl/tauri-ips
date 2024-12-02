/*
 * @Author: chunlaizhang chunlai.zhang@ujoin-tech.com
 * @Date: 2024-06-04 17:03:16
 * @LastEditors: chunlaizhang
 * @LastEditTime: 2024-06-05 14:10:25
 * @FilePath: \kws\src\renderer\components\common\alertMsg\index.jsx
 */
import { h, createApp } from 'vue'
import myAlert from './alert/index.vue'
const alertMsg = (icons, content, tips, options = {}) => {
  const div = document.createElement('div')
  document.body.appendChild(div)
  return new Promise(resolve => {
    const app = createApp({
      render() {
        return h(myAlert, {
          icons,
          title: tips,
          description: content,
          modelValue: true,
          onClose: () => {
            resolve(1)
            app.unmount(div)
            div.remove()
          },
          'onUpdate:modelValue': val => {
            if (!val) {
              app.unmount(div)
              document.body.removeChild(div)
              resolve(1)
            }
          },
          ...options
        })
      }
    })
    app.mount(div)
    if (options.autoClose ?? true) {
      setTimeout(() => {
        resolve(1)
        app.unmount(div)
        div.remove()
      }, options.duration || 3000)
    }
  })
}

export default alertMsg
