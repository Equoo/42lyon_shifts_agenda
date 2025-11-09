import { defineStore } from 'pinia'
import { apiLogin, apiLogout } from '@/api.ts'
import { User } from '@/types/user.ts'

export const useAuthStore = defineStore('auth', {
  state: () => {
    return { user: null as User | null }
  },
  actions: {
    async login() {
      await apiLogin().then((uri) => {
        window.location.href = uri
      })
    },
  },
})
