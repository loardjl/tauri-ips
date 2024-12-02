import { createI18n } from 'vue-i18n'

export function loadLanguages() {
  const context = import.meta.glob(['../*/languages/*.*s', '../*/*/languages/*.*s'], {
    eager: true
  })

  const languages = {}

  const langs = Object.keys(context)
  for (const key of langs) {
    if (key === './index.js') return
    const lang = context[key].lang
    const list = key.split('/')
    const name = list[list.length - 1].split('.')[0]
    languages[name] = { ...languages[name], ...lang }
  }

  return languages
}

export function i18nt(key) {
  return i18n.global.d(key)
}

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-cn',
  fallbackLocale: 'zh-cn',
  messages: loadLanguages()
})

export function setLanguage(locale) {
  i18n.global.locale.value = locale
}
