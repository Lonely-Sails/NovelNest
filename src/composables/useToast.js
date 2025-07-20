import { ref, reactive } from 'vue'

// 全局 toast 状态
const toasts = reactive([])
let toastId = 0

export function useToast() {
  const showToast = (message, type = 'info', duration = 3000) => {
    const id = ++toastId
    const toast = {
      id,
      message,
      type,
      visible: true
    }
    
    toasts.push(toast)
    
    // 自动隐藏
    if (duration > 0) {
      setTimeout(() => {
        hideToast(id)
      }, duration)
    }
    
    return id
  }
  
  const hideToast = (id) => {
    const index = toasts.findIndex(toast => toast.id === id)
    if (index > -1) {
      toasts[index].visible = false
      // 延迟移除以支持动画
      setTimeout(() => {
        const currentIndex = toasts.findIndex(toast => toast.id === id)
        if (currentIndex > -1) {
          toasts.splice(currentIndex, 1)
        }
      }, 300)
    }
  }
  
  const clearAllToasts = () => {
    toasts.forEach(toast => {
      toast.visible = false
    })
    setTimeout(() => {
      toasts.splice(0)
    }, 300)
  }
  
  return {
    toasts,
    showToast,
    hideToast,
    clearAllToasts
  }
}