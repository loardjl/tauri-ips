export const _public = {
  /**
   * 创建一个防抖函数，该函数会在最后一次调用时刻的wait毫秒之后执行func函数
   * @param {Function} func 需要防抖的函数
   * @param {number} wait 等待时间，单位毫秒
   * @param {boolean} immediate 是否立即执行
   * @return {Function} 返回防抖处理后的函数
   */
  _debounce(func, wait = 200, immediate) {
    let timeout
    return function () {
      const context = this,
        args = arguments
      const later = function () {
        timeout = null
        if (!immediate) func.apply(context, args)
      }
      const callNow = immediate && !timeout
      clearTimeout(timeout)
      timeout = setTimeout(later, wait)
      if (callNow) func.apply(context, args)
    }
  },
  // 节流
  _throttle(fn, interval = 200) {
    let last
    let timer
    return function () {
      const th = this
      const args = arguments
      const now = +new Date()
      if (last && now - last < interval) {
        clearTimeout(timer)
        timer = setTimeout(function () {
          last = now
          fn.apply(th, args)
        }, interval)
      } else {
        last = now
        fn.apply(th, args)
      }
    }
  },
  isPrimitive(value) {
    return value === null || !['object', 'function'].includes(typeof value)
  },
  // 判断两个对象是否相等
  _equals(value1, value2) {
    if (this.isPrimitive(value1) || this.isPrimitive(value2)) {
      return Object.is(value1, value2)
    }
    const entries1 = Object.entries(value1)
    const entries2 = Object.entries(value2)
    if (entries1.length !== entries2.length) {
      return false
    }
    for (const [key, value] of entries1) {
      if (!this._equals(value, value2[key])) {
        return false
      }
    }
    return true
  },
  // 获取两个对象的不同属性并返回完整属性路径
  findDifferentProperties(obj1, obj2, path = '') {
    const differentProperties = []
    const len1 = Object.keys(obj1).length
    const len2 = Object.keys(obj2).length
    const obj = len1 > len2 ? obj1 : obj2
    for (const key in obj) {
      const currentPath = path ? `${path}.${key}` : key
      if (
        typeof obj1[key] === 'object' &&
        Object.prototype.hasOwnProperty.call(obj2, key) &&
        typeof obj2[key] === 'object'
      ) {
        differentProperties.push(...this.findDifferentProperties(obj1[key], obj2[key], currentPath))
      } else {
        if (obj1[key] !== obj2[key]) {
          differentProperties.push(currentPath)
        }
      }
    }
    return differentProperties
  },

  // 把数组里同一属性名的对象push进新的数组
  _sortArr(resData) {
    const tempArr = []
    const Data = []
    for (let i = 0; i < resData.length; i++) {
      if (tempArr.indexOf(resData[i].collCode) === -1) {
        Data.push({
          name: resData[i].collCode,
          tableData: [resData[i]]
        })
        tempArr.push(resData[i].collCode)
      } else {
        for (let j = 0; j < Data.length; j++) {
          if (Data[j].name === resData[i].collCode) {
            Data[j].tableData.push(resData[i])
            break
          }
        }
      }
    }
    // console.log(Data)
    return Data
  },
  //将具有相同属性名的对象推入结果数组
  pushObjectsWithSamePropertyName(arr, propName) {
    const resultMap = new Map()
    arr.forEach(obj => {
      if (Object.hasOwnProperty.call(obj, propName)) {
        const propValue = obj[propName]
        const existingArray = resultMap.get(propValue) || []
        existingArray.push(obj)
        resultMap.set(propValue, existingArray)
      }
    })
    const resultArray = Array.from(resultMap, ([key, value]) => ({ [propName]: key, items: value }))
    resultMap.clear()
    return resultArray
  },
  // 根据传入的函数的返回值，分组
  getGroupWithSamePropertyName(arr, propName) {
    const resultMap = new Map()
    if (typeof propName === 'string') {
      return this.pushObjectsWithSamePropertyName(arr, propName)
    }
    if (Array.isArray(propName)) {
      arr.forEach(obj => {
        const propValue = propName.map(name => obj[name]).join(',')
        const existingArray = resultMap.get(propValue) || []
        existingArray.push(obj)
        resultMap.set(propValue, existingArray)
      })
      const resultArray = Array.from(resultMap, ([key, value]) => ({
        name: key,
        items: value
      }))
      resultMap.clear()
      return resultArray
    }
    if (typeof propName === 'function') {
      arr.forEach(obj => {
        const propValue = propName(obj)
        const existingArray = resultMap.get(propValue) || []
        existingArray.push(obj)
        resultMap.set(propValue, existingArray)
      })
      const resultArray = Array.from(resultMap, ([key, value]) => ({
        name: key,
        items: value
      }))
      resultMap.clear()
      return resultArray
    }
  },
  /**
   * 深拷贝
   * @param {*} obj 拷贝对象(object or array)
   * @param {*} cache 缓存数组
   */
  deepCopy(obj, cache = []) {
    // typeof [] => 'object'
    // typeof {} => 'object'
    if (obj === null || typeof obj !== 'object') {
      return obj
    }
    // 如果传入的对象与缓存的相等, 则递归结束, 这样防止循环
    const hit = cache.filter(c => c.original === obj)[0]
    if (hit) {
      return hit.copy
    }

    const copy = Array.isArray(obj) ? [] : {}
    // 将copy首先放入cache, 因为我们需要在递归deepCopy的时候引用它
    cache.push({
      original: obj,
      copy
    })
    Object.keys(obj).forEach(key => {
      copy[key] = this.deepCopy(obj[key], cache)
    })

    return copy
  },

  // 通过某个key找到数组中的整个obj
  findObj(arr, key, value) {
    for (const val of arr) {
      if (val[key] === value) {
        return val
      }
    }
    return null
  },
  getImageUrl(url) {
    return new URL(url, import.meta.url).href
  },
  imageToBase64(url) {
    return new Promise((resolve, reject) => {
      let canvas = document.createElement('canvas')
      const ctx = canvas.getContext('2d')
      const img = new Image()
      img.crossOrigin = 'Anonymous'
      img.onload = function () {
        canvas.height = img.height
        canvas.width = img.width
        ctx.drawImage(img, 0, 0)
        const dataURL = canvas.toDataURL('image/png')
        resolve(dataURL)
        canvas = null
      }
      img.onerror = function () {
        reject('图片加载失败')
      }
      img.src = url
    })
  },

  // ----------------- 时间戳转时间格式
  timestampToTime(timestamp) {
    const date = new Date(timestamp)
    const year = date.getFullYear()
    const month = (date.getMonth() + 1).toString().padStart(2, '0')
    const day = date.getDate().toString().padStart(2, '0')
    const hours = date.getHours().toString().padStart(2, '0')
    const minutes = date.getMinutes().toString().padStart(2, '0')
    const seconds = date.getSeconds().toString().padStart(2, '0')

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
  },
  pxTovw(px) {
    const dpr = window.devicePixelRatio || 1
    const width = window.screen.width
    return (px * dpr) / (width / 100)
  },
  screenPx(px) {
    return ((px / 1920) * 100 * window.screen.width) / 100
  },
  //秒转 秒 小时 分钟 天
  getTime(time, format, isHour = true) {
    time = time / 1000
    const d = parseInt(time / 60 / 60 / 24)
    const h = isHour ? parseInt((time / 60 / 60) % 24) : parseInt(time / 60 / 60)
    const m = parseInt((time / 60) % 60)
    const s = parseInt(time % 60)
    const replacements = {
      d: d.toString().padStart(2, '0'),
      h: h.toString().padStart(2, '0'), // 格式化为两位数
      m: m.toString().padStart(2, '0'),
      s: s.toString().padStart(2, '0')
    }
    return format.replace(/{(d|h|m|s)}/g, (_, key) => replacements[key])
  }
}
