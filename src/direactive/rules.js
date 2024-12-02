import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
let userAuth = 0
sysStore.auth.forEach(d => {
  userAuth |= 1 << d.business
})
const rules = (el, bind) => {
  const { arg, value } = bind
  if (arg === 'menu') {
    const { rules, auth } = value
    if (!auth) {
      check(el, rules)
      return
    }
    if (Array.isArray(auth)) {
      auth.some(d => (userAuth & d) === d) ? check(el, rules) : el.remove()
      return
    }
    if ((userAuth & auth) !== auth) {
      el.remove()
      return
    }
    check(el, rules)
    return
  }
  check(el, value)
}

const check = (el, value) => {
  if (value === '*' || sysStore.rules.includes(value)) {
    return
  } else {
    el.remove()
  }
}

export { rules }
