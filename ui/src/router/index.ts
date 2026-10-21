import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import StageView from '../views/StageView.vue'
import ConfigView from '../views/ConfigView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/stage/:id',
    name: 'stage',
    component: StageView,
    props: true
  },
  {
    path: '/config',
    name: 'config',
    component: ConfigView
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

export default router