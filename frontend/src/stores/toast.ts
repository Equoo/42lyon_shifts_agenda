import { defineStore } from 'pinia'

type NotificationType = 'success' | 'info' | 'warning' | 'error'

type ToastNotification = {
  id: number
  type: NotificationType
  msg: string
}

export const useToastStore = defineStore('toast', {
  state: () => {
    return {
      nextId: 1,
      toasts: [] as ToastNotification[],
    }
  },
  actions: {
    push(type: NotificationType, msg: string) {
      this.toasts.push({ id: this.nextId++, type, msg })
    },
    success(msg: string) {
      this.push('success', msg)
    },
    info(msg: string) {
      this.push('info', msg)
    },
    warn(msg: string) {
      this.push('warning', msg)
    },
    error(msg: string) {
      this.push('error', msg)
    },
    dismiss(id: number) {
      for (let i = 0; i < this.toasts.length; i++) {
        if (this.toasts[i]?.id === id) {
          this.toasts.splice(i--, 1)
        }
      }
    },
  },
})
