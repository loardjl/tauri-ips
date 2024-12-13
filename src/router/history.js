import authEnum from '@src/utils/authEnum'
const historyRoute = [
  {
    path: '/record',
    name: '历史数据',
    meta: {
      title: 'menu.detectionRecord',
      order: 2,
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/record/index.vue'),
    beforeEnter(to, from, next) {
      const fullPath = from.fullPath
      const regex = /record/
      const result = regex.test(fullPath)
      if (!result) {
        sessionStorage.removeItem('record_program_num')
        sessionStorage.removeItem('record_program_dateTiem')
        sessionStorage.removeItem('machiningPage')
      }
      next()
    }
  },
  {
    path: '/record/machiningRecords',
    name: '加工记录',
    meta: {
      title: 'menu.machiningRecords',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/record/machiningRecords.vue')
  },
  {
    path: '/record/machiningRecords/dataDetails',
    name: '数据详情',
    meta: {
      title: 'menu.dataDetails',
      rules: '*',
      auth: authEnum.COMMON
    },
    component: () => import('@src/views/record/dataDetails.vue')
  }
]
export default historyRoute
