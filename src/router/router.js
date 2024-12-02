import { router } from '@src/router/loadRouter'
const routes = [
  { path: '/:pathMatch(.*)*', component: () => import('@src/views/404.vue') },
  { path: '/', redirect: '/home' },
  {
    path: '/home',
    name: 'home',
    component: () => import('@src/layout/index.vue'),
    children: [...router]
  }
]

export default routes
