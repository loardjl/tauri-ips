// 正则表达式
const reg = {
  num: /[^\d]/g, // 数字
  zh: /[\u4e00-\u9fa5]/g, // 中文
  normal: /[\x21-\x2F\x3A-\x40\x5B-\x60\x7B-\x7E]+/, // 特殊字符
  numZh: /[^\d\u4e00-\u9fa5]/g, // 非数字和中文
  // 只能输入数字和一个小数点
  numPoint: /[^\d.]/g
}
const handleInput = (e, bind) => {
  if (e.target.composing) return
  e.target.value = e.target.value.replace(reg[bind.arg], '')
  // 超过n位小数点去除
  if (e.target.value.indexOf('.') !== -1) {
    const arr = e.target.value.split('.')
    if (arr[1].length > bind.value) {
      e.target.value = arr[0] + '.' + arr[1].slice(0, bind.value)
    }
  }
  // 只能出现一次小数点
  if (e.target.value.indexOf('.') !== e.target.value.lastIndexOf('.')) {
    e.target.value = e.target.value.slice(0, e.target.value.lastIndexOf('.'))
  }
  // 限制输入长度判断中文英文
  if (bind.arg === 'len' && bind.value) {
    e.target.value = e.target.value.replace(' ', '')
    const str = e.target.value
    let len = 0
    for (let i = 0; i < str.length; i++) {
      if (str.charCodeAt(i) > 127 || str.charCodeAt(i) === 94) {
        len += 3
      } else {
        len++
      }
      if (len > bind.value) {
        e.target.value = str.slice(0, i)
        break
      }
    }
  }
  numFunction(e, bind)
  e.target.dispatchEvent(new Event('input'))
}

const numFunction = (e, bind) => {
  if (bind.arg === 'num' && bind.value) {
    const numValue = e.target.value
    if (numValue.length > 1 && numValue.startsWith('0')) {
      e.target.value = numValue.slice(1)
    }
    if (Array.isArray(bind.value)) {
      if (numValue < bind.value[0]) {
        e.target.value = bind.value[0]
      } else if (numValue > bind.value[1]) {
        e.target.value = bind.value[1]
      }
    } else if (numValue > bind.value) {
      e.target.value = bind.value
    }
  }
}
/**
 * 自定义输入框限制指令
 * @param {*} el 当前dom
 * @param {*} bind 指令对象
 * @param {*} vNode 当前虚拟dom
 * @param {*} preNode 之前的虚拟dom
 */
const myInput = {
  mounted: (el, bind) => {
    el.addEventListener('compositionstart', e => {
      e.target.composing = true
    })
    el.addEventListener('compositionend', e => {
      if (!e.target.composing) {
        return
      }
      e.target.composing = false
      handleInput(e, bind)
    })
    el.oninput = e => handleInput(e, bind)
  }
}
export default { myInput }
