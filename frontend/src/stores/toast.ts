import { defineStore } from 'pinia'

type NotificationType = 'success' | 'info' | 'warning' | 'error'

type ToastNotification = {
  id: number
  type: NotificationType
  msg: string
  timeoutId: number
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
      const id = this.nextId++
      const timeoutId = setTimeout(() => this.dismiss(id), 5000)
      this.toasts.push({ id, type, msg, timeoutId })
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
    pause(id: number) {
      const toast = this.toasts.find((t) => t.id === id)
      if (toast && toast.timeoutId > 0) {
        clearTimeout(toast.timeoutId)
        toast.timeoutId = -1
      }
    },
    resume(id: number) {
      const toast = this.toasts.find((t) => t.id === id)
      if (toast && toast.timeoutId < 0) {
        toast.timeoutId = setTimeout(() => this.dismiss(toast.id), 5000)
      }
    },
  },
})
