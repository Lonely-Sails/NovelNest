<template>
  <Teleport to="body">
    <Transition name="toast" appear>
      <div 
        v-if="show" 
        class="toast-container"
        :class="[`toast-${type}`, `toast-${position}`]"
      >
        <div class="toast-content">
          <!-- 图标 -->
          <div class="toast-icon">
            <span v-if="type === 'success'">✅</span>
            <span v-else-if="type === 'error'">❌</span>
            <span v-else-if="type === 'warning'">⚠️</span>
            <span v-else>ℹ️</span>
          </div>
          
          <!-- 消息内容 -->
          <div class="toast-message">
            {{ message }}
          </div>
          
          <!-- 关闭按钮 -->
          <button 
            v-if="closable" 
            class="toast-close"
            @click="$emit('close')"
          >
            ✕
          </button>
        </div>
        
        <!-- 进度条 -->
        <div 
          v-if="showProgress" 
          class="toast-progress"
          :style="{ animationDuration: duration + 'ms' }"
        ></div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { onMounted, onUnmounted } from 'vue'

// Props
const props = defineProps({
  message: {
    type: String,
    required: true
  },
  type: {
    type: String,
    default: 'info', // 'success', 'error', 'warning', 'info'
    validator: (value) => ['success', 'error', 'warning', 'info'].includes(value)
  },
  duration: {
    type: Number,
    default: 3000
  },
  position: {
    type: String,
    default: 'top-right', // 'top-left', 'top-right', 'bottom-left', 'bottom-right', 'top-center', 'bottom-center'
    validator: (value) => [
      'top-left', 'top-right', 'bottom-left', 
      'bottom-right', 'top-center', 'bottom-center'
    ].includes(value)
  },
  closable: {
    type: Boolean,
    default: true
  },
  showProgress: {
    type: Boolean,
    default: true
  },
  show: {
    type: Boolean,
    default: true
  }
})

// Emits
const emit = defineEmits(['close'])

// 自动关闭定时器
let autoCloseTimer = null

// 组件挂载时设置自动关闭
onMounted(() => {
  if (props.duration > 0) {
    autoCloseTimer = setTimeout(() => {
      emit('close')
    }, props.duration)
  }
})

// 组件卸载时清除定时器
onUnmounted(() => {
  if (autoCloseTimer) {
    clearTimeout(autoCloseTimer)
  }
})
</script>

<style scoped>
.toast-container {
  position: fixed;
  z-index: 9999;
  max-width: 400px;
  min-width: 300px;
  margin: 1rem;
  pointer-events: auto;
}

/* 位置样式 */
.toast-top-left {
  top: 0;
  left: 0;
}

.toast-top-right {
  top: 0;
  right: 0;
}

.toast-bottom-left {
  bottom: 0;
  left: 0;
}

.toast-bottom-right {
  bottom: 0;
  right: 0;
}

.toast-top-center {
  top: 0;
  left: 50%;
  transform: translateX(-50%);
}

.toast-bottom-center {
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
}

.toast-content {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 1rem;
  background: var(--bg-primary);
  border-radius: 8px;
  box-shadow: var(--shadow-lg);
  border: 1px solid var(--border-color);
  position: relative;
  overflow: hidden;
}

/* 类型样式 */
.toast-success .toast-content {
  border-left: 4px solid #10b981;
  background: rgba(16, 185, 129, 0.05);
}

.toast-error .toast-content {
  border-left: 4px solid #ef4444;
  background: rgba(239, 68, 68, 0.05);
}

.toast-warning .toast-content {
  border-left: 4px solid #f59e0b;
  background: rgba(245, 158, 11, 0.05);
}

.toast-info .toast-content {
  border-left: 4px solid #3b82f6;
  background: rgba(59, 130, 246, 0.05);
}

.toast-icon {
  flex-shrink: 0;
  font-size: 1.2rem;
  line-height: 1;
}

.toast-message {
  flex: 1;
  color: var(--text-primary);
  font-size: 0.9rem;
  line-height: 1.4;
  word-break: break-word;
}

.toast-close {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  font-size: 1rem;
  line-height: 1;
  transition: color 0.2s ease;
}

.toast-close:hover {
  color: var(--text-primary);
}

.toast-progress {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 3px;
  background: currentColor;
  animation: toastProgress linear forwards;
  opacity: 0.6;
}

.toast-success .toast-progress {
  color: #10b981;
}

.toast-error .toast-progress {
  color: #ef4444;
}

.toast-warning .toast-progress {
  color: #f59e0b;
}

.toast-info .toast-progress {
  color: #3b82f6;
}

/* 动画 */
@keyframes toastProgress {
  from {
    width: 100%;
  }
  to {
    width: 0%;
  }
}

/* 过渡动画 */
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

/* 左侧位置的动画 */
.toast-top-left.toast-enter-from,
.toast-bottom-left.toast-enter-from {
  transform: translateX(-100%);
}

.toast-top-left.toast-leave-to,
.toast-bottom-left.toast-leave-to {
  transform: translateX(-100%);
}

/* 中心位置的动画 */
.toast-top-center.toast-enter-from,
.toast-bottom-center.toast-enter-from {
  transform: translateX(-50%) translateY(-20px);
  opacity: 0;
}

.toast-top-center.toast-leave-to,
.toast-bottom-center.toast-leave-to {
  transform: translateX(-50%) translateY(-20px);
  opacity: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .toast-container {
    max-width: calc(100vw - 2rem);
    min-width: auto;
    margin: 0.5rem;
  }
  
  .toast-content {
    padding: 0.75rem;
  }
  
  .toast-message {
    font-size: 0.85rem;
  }
}
</style>