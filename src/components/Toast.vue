<template>
  <Teleport to="body">
    <div class="toast-container">
      <Transition
        v-for="toast in toasts"
        :key="toast.id"
        name="toast"
        appear
      >
        <div
          class="toast"
          :class="[`toast-${toast.type}`, { 'toast-closable': toast.closable }]"
        >
          <div class="toast-icon">
            {{ getIcon(toast.type) }}
          </div>
          <div class="toast-content">
            <div class="toast-title" v-if="toast.title">{{ toast.title }}</div>
            <div class="toast-message">{{ toast.message }}</div>
          </div>
          <button
            v-if="toast.closable"
            @click="removeToast(toast.id)"
            class="toast-close"
          >
            ✕
          </button>
        </div>
      </Transition>
    </div>
  </Teleport>
</template>

<script>
import { ref, onMounted } from 'vue'

// 全局 toast 状态
const toasts = ref([])
let toastId = 0

// Toast 管理器
export const useToast = () => {
  const addToast = (options) => {
    const toast = {
      id: ++toastId,
      type: 'info',
      title: '',
      message: '',
      duration: 3000,
      closable: true,
      ...options
    }

    toasts.value.push(toast)

    // 自动移除
    if (toast.duration > 0) {
      setTimeout(() => {
        removeToast(toast.id)
      }, toast.duration)
    }

    return toast.id
  }

  const removeToast = (id) => {
    const index = toasts.value.findIndex(toast => toast.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  const clearToasts = () => {
    toasts.value = []
  }

  // 便捷方法
  const success = (message, options = {}) => {
    return addToast({ ...options, type: 'success', message })
  }

  const error = (message, options = {}) => {
    return addToast({ ...options, type: 'error', message, duration: 5000 })
  }

  const warning = (message, options = {}) => {
    return addToast({ ...options, type: 'warning', message })
  }

  const info = (message, options = {}) => {
    return addToast({ ...options, type: 'info', message })
  }

  return {
    addToast,
    removeToast,
    clearToasts,
    success,
    error,
    warning,
    info
  }
}

export default {
  name: 'Toast',
  setup() {
    const { removeToast } = useToast()

    const getIcon = (type) => {
      const icons = {
        success: '✅',
        error: '❌',
        warning: '⚠️',
        info: 'ℹ️'
      }
      return icons[type] || icons.info
    }

    return {
      toasts,
      removeToast,
      getIcon
    }
  }
}
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 1rem;
  right: 1rem;
  z-index: 1100;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  max-width: 400px;
}

.toast {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 1rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border-left: 4px solid;
  min-width: 300px;
}

.toast-success {
  border-left-color: #28a745;
  background-color: #f8fff9;
}

.toast-error {
  border-left-color: #dc3545;
  background-color: #fff8f8;
}

.toast-warning {
  border-left-color: #ffc107;
  background-color: #fffdf5;
}

.toast-info {
  border-left-color: #007bff;
  background-color: #f8fbff;
}

.toast-icon {
  font-size: 1.2rem;
  flex-shrink: 0;
  margin-top: 0.1rem;
}

.toast-content {
  flex: 1;
}

.toast-title {
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
}

.toast-message {
  color: #495057;
  font-size: 0.85rem;
  line-height: 1.4;
}

.toast-close {
  background: none;
  border: none;
  color: #6c757d;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
  font-size: 0.9rem;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.toast-close:hover {
  background-color: rgba(0, 0, 0, 0.1);
  color: #495057;
}

/* 动画效果 */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

/* 响应式设计 */
@media (max-width: 640px) {
  .toast-container {
    left: 1rem;
    right: 1rem;
    max-width: none;
  }
  
  .toast {
    min-width: auto;
  }
}

/* 暗色主题支持 */
.theme-dark .toast {
  background: #2d3748;
  color: #e2e8f0;
}

.theme-dark .toast-success {
  background-color: #1a2e1a;
}

.theme-dark .toast-error {
  background-color: #2e1a1a;
}

.theme-dark .toast-warning {
  background-color: #2e2a1a;
}

.theme-dark .toast-info {
  background-color: #1a1e2e;
}

.theme-dark .toast-title {
  color: #e2e8f0;
}

.theme-dark .toast-message {
  color: #a0aec0;
}

.theme-dark .toast-close {
  color: #a0aec0;
}

.theme-dark .toast-close:hover {
  background-color: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
}
</style>