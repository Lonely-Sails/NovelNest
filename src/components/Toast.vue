<template>
  <Teleport to="body">
    <div class="toast-container">
      <Transition
        v-for="toast in toasts"
        :key="toast.id"
        name="toast"
        appear
      >
        <BaseCard
          class="toast"
          :class="[`toast-${toast.type}`, { 'toast-closable': toast.closable }]"
        >
          <template #header>
            <div class="toast-header">
              <BaseBadge 
                :variant="toast.type === 'error' ? 'error' : toast.type === 'success' ? 'success' : toast.type === 'warning' ? 'warning' : 'info'"
                :icon="getIcon(toast.type)"
                size="small"
              >
                {{ toast.title || getTypeText(toast.type) }}
              </BaseBadge>
              <BaseButton
                v-if="toast.closable"
                @click="removeToast(toast.id)"
                variant="ghost"
                size="small"
                icon="✕"
              />
            </div>
          </template>
          
          <div class="toast-message">{{ toast.message }}</div>
        </BaseCard>
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

    const getTypeText = (type) => {
      const texts = {
        success: '成功',
        error: '错误',
        warning: '警告',
        info: '信息'
      }
      return texts[type] || texts.info
    }

    return {
      toasts,
      removeToast,
      getIcon,
      getTypeText
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
  min-width: 300px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.toast-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.toast-message {
  color: var(--text-primary);
  font-size: 0.9rem;
  line-height: 1.4;
  margin-top: 0.5rem;
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

/* 暗色主题支持已通过基础组件处理 */
</style>