<template>
  <div class="loading-container" :class="{ 'loading-overlay': overlay, 'loading-inline': !overlay }">
    <div class="loading-content">
      <div class="loading-spinner" :class="sizeClass">
        <div class="spinner-ring"></div>
        <div class="spinner-ring"></div>
        <div class="spinner-ring"></div>
        <div class="spinner-ring"></div>
      </div>
      <div v-if="text" class="loading-text">{{ text }}</div>
    </div>
  </div>
</template>

<script>
import { computed } from 'vue'

export default {
  name: 'Loading',
  props: {
    text: {
      type: String,
      default: ''
    },
    size: {
      type: String,
      default: 'medium',
      validator: (value) => ['small', 'medium', 'large'].includes(value)
    },
    overlay: {
      type: Boolean,
      default: false
    }
  },
  setup(props) {
    const sizeClass = computed(() => `loading-${props.size}`)

    return {
      sizeClass
    }
  }
}
</script>

<style scoped>
.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.9);
  z-index: 999;
}

.loading-inline {
  padding: 2rem;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.loading-spinner {
  position: relative;
  display: inline-block;
}

.loading-small {
  width: 24px;
  height: 24px;
}

.loading-medium {
  width: 40px;
  height: 40px;
}

.loading-large {
  width: 60px;
  height: 60px;
}

.spinner-ring {
  position: absolute;
  border: 3px solid transparent;
  border-top: 3px solid var(--primary-color);
  border-radius: 50%;
  animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}

.loading-small .spinner-ring {
  width: 24px;
  height: 24px;
  border-width: 2px;
  border-top-width: 2px;
}

.loading-medium .spinner-ring {
  width: 40px;
  height: 40px;
  border-width: 3px;
  border-top-width: 3px;
}

.loading-large .spinner-ring {
  width: 60px;
  height: 60px;
  border-width: 4px;
  border-top-width: 4px;
}

.spinner-ring:nth-child(1) {
  animation-delay: -0.45s;
}

.spinner-ring:nth-child(2) {
  animation-delay: -0.3s;
}

.spinner-ring:nth-child(3) {
  animation-delay: -0.15s;
}

.loading-text {
  color: var(--text-secondary);
  font-size: 0.9rem;
  text-align: center;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* 暗色主题支持已通过 CSS 变量统一处理 */
.theme-dark .loading-overlay {
  background-color: rgba(26, 26, 26, 0.9);
}
</style>