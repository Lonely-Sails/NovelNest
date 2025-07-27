import { ref } from 'vue'

// 全局Toast状态
const toastState = ref({
  show: false,
  message: '',
  type: 'info', // 'info', 'success', 'warning', 'error'
  duration: 3000
})

let toastTimer = null

/**
 * Toast消息提示组合式函数
 * 提供统一的消息提示功能
 */
export function useToast() {
  /**
   * 显示Toast消息
   * @param {string} message - 消息内容
   * @param {string} type - 消息类型 ('info', 'success', 'warning', 'error')
   * @param {number} duration - 显示时长（毫秒）
   */
  const showToast = (message, type = 'info', duration = 3000) => {
    // 清除之前的定时器
    if (toastTimer) {
      clearTimeout(toastTimer)
    }

    toastState.value = {
      show: true,
      message,
      type,
      duration
    }

    // 自动隐藏
    toastTimer = setTimeout(() => {
      hideToast()
    }, duration)
  }

  /**
   * 隐藏Toast消息
   */
  const hideToast = () => {
    toastState.value.show = false
    if (toastTimer) {
      clearTimeout(toastTimer)
      toastTimer = null
    }
  }

  /**
   * 显示成功消息
   * @param {string} message - 消息内容
   */
  const showSuccess = (message) => {
    showToast(message, 'success')
  }

  /**
   * 显示错误消息
   * @param {string} message - 消息内容
   */
  const showError = (message) => {
    showToast(message, 'error')
  }

  /**
   * 显示警告消息
   * @param {string} message - 消息内容
   */
  const showWarning = (message) => {
    showToast(message, 'warning')
  }

  /**
   * 显示信息消息
   * @param {string} message - 消息内容
   */
  const showInfo = (message) => {
    showToast(message, 'info')
  }

  return {
    toastState,
    showToast,
    hideToast,
    showSuccess,
    showError,
    showWarning,
    showInfo
  }
}