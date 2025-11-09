import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import CallbackView from '@/views/CallbackView.vue'
import LoginView from '@/views/LoginView.vue'
import { useAuthStore } from '@/stores/auth.ts'
import { useToastStore } from '@/stores/toast.ts'
import ShiftsView from '@/views/ShiftsView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/shifts', component: ShiftsView },
  { path: '/login', component: LoginView },
  { path: '/auth/callback', component: CallbackView },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(async (to) => {
  const publicPages = ['/login', '/auth/callback']
  const authRequired = !publicPages.includes(to.path)
  const auth = useAuthStore()
  const toast = useToastStore()

  if (authRequired && !auth.user) {
    toast.info('You need to login to access this resource')
    return '/login'
  }
})

export default router
