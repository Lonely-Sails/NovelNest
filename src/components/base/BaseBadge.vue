<template>
  <span :class="badgeClasses">
    <!-- 点状徽章 -->
    <span v-if="dot" class="badge-dot-indicator"></span>
    
    <!-- 普通徽章内容 -->
    <template v-else>
      <span v-if="$slots.icon || icon" class="badge-icon">
        <slot name="icon">{{ icon }}</slot>
      </span>
      <span class="badge-text">
        <slot>{{ text }}</slot>
      </span>
    </template>
  </span>
</template>

<script>
import { computed } from 'vue'

export default {
  name: 'BaseBadge',
  props: {
    // 徽章文本
    text: {
      type: [String, Number],
      default: ''
    },
    // 徽章变体：default, primary, secondary, success, warning, error, info
    variant: {
      type: String,
      default: 'default',
      validator: (value) => ['default', 'primary', 'secondary', 'success', 'warning', 'error', 'info'].includes(value)
    },
    // 徽章大小：small, medium, large
    size: {
      type: String,
      default: 'medium',
      validator: (value) => ['small', 'medium', 'large'].includes(value)
    },
    // 是否为点状徽章
    dot: {
      type: Boolean,
      default: false
    },
    // 是否为轮廓样式
    outline: {
      type: Boolean,
      default: false
    },
    // 是否为圆角样式
    rounded: {
      type: Boolean,
      default: false
    },
    // 图标
    icon: {
      type: String,
      default: ''
    }
  },
  setup(props) {
    // 计算徽章样式类
    const badgeClasses = computed(() => [
      'base-badge',
      `badge-${props.variant}`,
      `badge-${props.size}`,
      {
        'badge-dot': props.dot,
        'badge-outline': props.outline,
        'badge-rounded': props.rounded
      }
    ])

    return {
      badgeClasses
    }
  }
}
</script>

<style scoped>
.base-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.25rem;
  font-weight: 500;
  white-space: nowrap;
  vertical-align: middle;
  border-radius: 4px;
  border: 1px solid transparent;
  transition: all 0.3s ease;
}

/* 徽章大小 */
.badge-small {
  padding: 0.125rem 0.375rem;
  font-size: 0.7rem;
  min-height: 16px;
}

.badge-medium {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  min-height: 20px;
}

.badge-large {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
  min-height: 24px;
}

/* 点状徽章大小 */
.badge-dot.badge-small {
  width: 8px;
  height: 8px;
  padding: 0;
  min-height: auto;
}

.badge-dot.badge-medium {
  width: 10px;
  height: 10px;
  padding: 0;
  min-height: auto;
}

.badge-dot.badge-large {
  width: 12px;
  height: 12px;
  padding: 0;
  min-height: auto;
}

/* 徽章变体 */
.badge-default {
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border-color: var(--border-color);
}

.badge-primary {
  background-color: var(--primary-color);
  color: white;
}

.badge-secondary {
  background-color: var(--text-secondary);
  color: white;
}

.badge-success {
  background-color: var(--success-color);
  color: white;
}

.badge-warning {
  background-color: var(--warning-color);
  color: white;
}

.badge-error {
  background-color: var(--error-color);
  color: white;
}

.badge-info {
  background-color: var(--info-color);
  color: white;
}

/* 轮廓样式 */
.badge-outline.badge-default {
  background-color: transparent;
  color: var(--text-primary);
  border-color: var(--border-color);
}

.badge-outline.badge-primary {
  background-color: transparent;
  color: var(--primary-color);
  border-color: var(--primary-color);
}

.badge-outline.badge-secondary {
  background-color: transparent;
  color: var(--text-secondary);
  border-color: var(--text-secondary);
}

.badge-outline.badge-success {
  background-color: transparent;
  color: var(--success-color);
  border-color: var(--success-color);
}

.badge-outline.badge-warning {
  background-color: transparent;
  color: var(--warning-color);
  border-color: var(--warning-color);
}

.badge-outline.badge-error {
  background-color: transparent;
  color: var(--error-color);
  border-color: var(--error-color);
}

.badge-outline.badge-info {
  background-color: transparent;
  color: var(--info-color);
  border-color: var(--info-color);
}

/* 圆角样式 */
.badge-rounded {
  border-radius: 50px;
}

.badge-dot {
  border-radius: 50%;
}

/* 点状指示器 */
.badge-dot-indicator {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background-color: currentColor;
}

/* 图标样式 */
.badge-icon {
  display: flex;
  align-items: center;
  font-size: 0.9em;
}

.badge-text {
  line-height: 1;
}

/* 特殊情况：只有图标时的样式 */
.base-badge:not(.badge-dot) .badge-icon:only-child {
  margin: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .badge-large {
    padding: 0.3rem 0.6rem;
    font-size: 0.75rem;
  }
}
</style>