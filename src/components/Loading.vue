<template>
  <div class="loading-container" :class="{ 'loading-overlay': overlay }">
    <div class="loading-content">
      <!-- 加载动画 -->
      <div class="loading-spinner" :class="[`spinner-${type}`, `spinner-${size}`]">
        <div v-if="type === 'dots'" class="dots-spinner">
          <div class="dot"></div>
          <div class="dot"></div>
          <div class="dot"></div>
        </div>
        <div v-else-if="type === 'circle'" class="circle-spinner"></div>
        <div v-else-if="type === 'pulse'" class="pulse-spinner"></div>
        <div v-else class="default-spinner"></div>
      </div>
      
      <!-- 加载文本 -->
      <div v-if="message" class="loading-message">
        {{ message }}
      </div>
      
      <!-- 进度条 -->
      <div v-if="showProgress && progress >= 0" class="loading-progress">
        <div class="progress-bar">
          <div 
            class="progress-fill" 
            :style="{ width: progress + '%' }"
          ></div>
        </div>
        <div class="progress-text">{{ Math.round(progress) }}%</div>
      </div>
    </div>
  </div>
</template>

<script setup>
// Props
const props = defineProps({
  message: {
    type: String,
    default: '加载中...'
  },
  type: {
    type: String,
    default: 'circle', // 'dots', 'circle', 'pulse', 'default'
    validator: (value) => ['dots', 'circle', 'pulse', 'default'].includes(value)
  },
  size: {
    type: String,
    default: 'medium', // 'small', 'medium', 'large'
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  },
  overlay: {
    type: Boolean,
    default: false
  },
  progress: {
    type: Number,
    default: -1 // -1 表示不显示进度
  },
  showProgress: {
    type: Boolean,
    default: false
  }
})
</script>

<style scoped>
.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.8);
  z-index: 999;
  backdrop-filter: blur(2px);
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

/* 加载动画基础样式 */
.loading-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
}

.spinner-small {
  width: 24px;
  height: 24px;
}

.spinner-medium {
  width: 40px;
  height: 40px;
}

.spinner-large {
  width: 56px;
  height: 56px;
}

/* 点状加载动画 */
.dots-spinner {
  display: flex;
  gap: 4px;
}

.dots-spinner .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--accent-color);
  animation: dotsBounce 1.4s ease-in-out infinite both;
}

.dots-spinner .dot:nth-child(1) { animation-delay: -0.32s; }
.dots-spinner .dot:nth-child(2) { animation-delay: -0.16s; }

@keyframes dotsBounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

/* 圆形加载动画 */
.circle-spinner {
  width: 100%;
  height: 100%;
  border: 3px solid var(--border-color);
  border-top: 3px solid var(--accent-color);
  border-radius: 50%;
  animation: circleRotate 1s linear infinite;
}

@keyframes circleRotate {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 脉冲加载动画 */
.pulse-spinner {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background-color: var(--accent-color);
  animation: pulseScale 1.5s ease-in-out infinite;
}

@keyframes pulseScale {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

/* 默认加载动画 */
.default-spinner {
  width: 100%;
  height: 100%;
  border: 2px solid transparent;
  border-top: 2px solid var(--accent-color);
  border-right: 2px solid var(--accent-color);
  border-radius: 50%;
  animation: defaultRotate 1s linear infinite;
}

@keyframes defaultRotate {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 加载文本 */
.loading-message {
  color: var(--text-secondary);
  font-size: 0.9rem;
  text-align: center;
  max-width: 200px;
}

/* 进度条 */
.loading-progress {
  width: 200px;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.progress-bar {
  width: 100%;
  height: 4px;
  background-color: var(--border-color);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--accent-color);
  transition: width 0.3s ease;
  border-radius: 2px;
}

.progress-text {
  text-align: center;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .loading-container {
    padding: 1rem;
  }
  
  .loading-message {
    font-size: 0.8rem;
    max-width: 150px;
  }
  
  .loading-progress {
    width: 150px;
  }
}
</style>