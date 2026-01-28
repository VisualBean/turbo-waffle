import { ref } from 'vue'

export type ToastType = 'success' | 'error' | 'info'

export type Toast = {
  id: number
  message: string
  type: ToastType
  timeout: number
}

const toasts = ref<Toast[]>([])
let nextId = 1

function removeToast(id: number) {
  toasts.value = toasts.value.filter((toast) => toast.id !== id)
}

function pushToast(message: string, type: ToastType = 'info', timeout = 3000) {
  const id = nextId++
  toasts.value.push({ id, message, type, timeout })

  if (timeout > 0) {
    window.setTimeout(() => removeToast(id), timeout)
  }

  return id
}

export function useToasts() {
  return { toasts, pushToast, removeToast }
}
