/**
 * @description: 扫描router目录下的文件，自动加载路由
 */
function loadRouter() {
  const context = import.meta.glob(['../router/*.*s'], {
    eager: true
  })
  const router = []
  const routes = Object.keys(context)
  for (const route of routes) {
    const list = route.split('/')
    const name = list[list.length - 1].split('.')[0]
    if (['router', 'index'].includes(name)) {
      continue
    }
    const module = context[route].default
    router.push(...module)
  }
  return router
}

export const router = loadRouter()
