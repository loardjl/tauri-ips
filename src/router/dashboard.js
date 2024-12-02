import authEnum from '@src/utils/authEnum'
const dashboardRoute = [
  {
    path: '/dashboard',
    name: '数据看板',
    meta: {
      title: 'menu.dashboard',
      order: 3,
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/dashboard/index.vue')
  }
]
export default dashboardRoute
