import authEnum from '@src/utils/authEnum'
const strategyRoute = [
  {
    path: '/strategy',
    name: '提效策略',
    meta: {
      title: 'menu.strategy',
      order: 3,
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/strategy/index.vue'),
    beforeEnter(to, from, next) {
      const fullPath = from.fullPath
      const regex = /strategy/
      const result = regex.test(fullPath)
      if (!result) {
        sessionStorage.removeItem('strategy_program_num')
      }
      next()
    }
  },
  {
    path: '/strategy/policyManagement',
    name: '策略管理',
    meta: {
      title: 'menu.policyManagement',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/strategy/policyManagement.vue')
  },
  {
    path: '/strategy/learningStyle',
    name: '学习方式',
    meta: {
      title: 'menu.learningStyle',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/strategy/learningStyle.vue')
  },
  {
    path: '/strategy/artifactManagement',
    name: '工件管理',
    meta: {
      title: 'menu.artifactManagement',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/strategy/artifactManagement.vue')
  }
]
export default strategyRoute
