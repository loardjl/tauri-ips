import authEnum from '@src/utils/authEnum'
const collectorRoute = [
  {
    path: '/collector',
    name: '采集配置',
    meta: {
      title: 'menu.collector',
      order: 2,
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/collector/index.vue')
  },
  {
    path: '/collector/add',
    name: '新增',
    meta: {
      title: 'menu.add',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/collector/add.vue')
  },
  {
    path: '/collector/view',
    name: '查看',
    meta: {
      title: 'menu.view',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/collector/view.vue')
  }
]
export default collectorRoute
