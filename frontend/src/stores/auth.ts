import { defineStore } from 'pinia'
import { apiLogin, apiLogout, getMe } from '@/api.ts'
import { User } from '@/types/user.ts'

export const useAuthStore = defineStore('auth', {
  state: () => {
    let cookieWasSet = localStorage.getItem('cookieWasSet') == 'true'
    return {
      user: null as User | null,
      cookieWasSet,
    }
  },
  actions: {
    async attemptAutoLogin(): Promise<boolean> {
      return getMe()
        .then((user) => {
          this.user = user
          return true
        })
        .catch(() => false)
    },
    unsetAutoLogin() {
      localStorage.removeItem('cookieWasSet')
      this.cookieWasSet = false
    },
    async login() {
      await apiLogin().then((uri) => {
        localStorage.setItem('cookieWasSet', 'true')
        window.location.href = uri
      })
    },
    async logout() {
      await apiLogout()
      this.user = null
    },
  },
})
