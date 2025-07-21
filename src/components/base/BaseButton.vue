<template>
  <component
    :is="tag"
    :class="[
      'base-button',
      `btn-${variant}`,
      `btn-${size}`,
      {
        'btn-loading': loading,
        'btn-block': block,
        'btn-rounded': rounded
      }
    ]"
    :disabled="disabled || loading"
    :type="type"
    :href="href"
    :target="target"
    @click="handleClick"
  >
    <!-- 加载状态图标 -->
    <div v-if="loading" class="btn-spinner">
      <div class="spinner"></div>
    </div>
    
    <!-- 前置图标 -->
    <span v-if="$slots.icon || icon" class="btn-icon btn-icon-left">
      <slot name="icon">{{ icon }}</slot>
    </span>
    
    <!-- 按钮文本 -->
    <span class="btn-text">
      <slot></slot>
    </span>
    
    <!-- 后置图标 -->
    <span v-if="$slots.iconRight || iconRight" class="btn-icon btn-icon-right">
      <slot name="iconRight">{{ iconRight }}</slot>
    </span>
  </component>
</template>

<script>
export default {
  name: 'BaseButton',
  props: {
    // 按钮变体：primary, secondary, success, warning, error, outline, ghost, link
    variant: {
      type: String,
      default: 'primary',
      validator: (value) => ['primary', 'secondary', 'success', 'warning', 'error', 'outline', 'ghost', 'link'].includes(value)
    },
    // 按钮大小：small, medium, large
    size: {
      type: String,
      default: 'medium',
      validator: (value) => ['small', 'medium', 'large'].includes(value)
    },
    // 是否禁用
    disabled: {
      type: Boolean,
      default: false
    },
    // 是否加载中
    loading: {
      type: Boolean,
      default: false
    },
    // 是否块级按钮
    block: {
      type: Boolean,
      default: false
    },
    // 是否圆角按钮
    rounded: {
      type: Boolean,
      default: false
    },
    // 按钮类型
    type: {
      type: String,
      default: 'button'
    },
    // 链接地址（当作为链接使用时）
    href: {
      type: String,
      default: ''
    },
    // 链接目标
    target: {
      type: String,
      default: ''
    },
    // 前置图标
    icon: {
      type: String,
      default: ''
    },
    // 后置图标
    iconRight: {
      type: String,
      default: ''
    }
  },
  emits: ['click'],
  setup(props, { emit }) {
    const tag = props.href ? 'a' : 'button'
    
    const handleClick = (event) => {
      if (!props.disabled && !props.loading) {
        emit('click', event)
      }
    }

    return {
      tag,
      handleClick
    }
  }
}
</script>

<style scoped>
.base-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  border: 1px solid transparent;
  border-radius: 6px;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  white-space: nowrap;
  user-select: none;
}

.base-button:focus {
  outline: none;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.base-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

/* 按钮大小 */
.btn-small {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
  min-height: 32px;
}

.btn-medium {
  padding: 0.5rem 1rem;
  font-size: 0.9rem;
  min-height: 40px;
}

.btn-large {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  min-height: 48px;
}

/* 按钮变体 */
.btn-primary {
  background: var(--primary-color);
  color: white;
  border-color: var(--primary-color);
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
  border-color: var(--primary-hover);
  transform: translateY(-1px);
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-color: var(--border-color);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--border-color);
  transform: translateY(-1px);
}

.btn-success {
  background: var(--success-color);
  color: white;
  border-color: var(--success-color);
}

.btn-success:hover:not(:disabled) {
  background: var(--success-hover);
  border-color: var(--success-hover);
  transform: translateY(-1px);
}

.btn-warning {
  background: var(--warning-color);
  color: white;
  border-color: var(--warning-color);
}

.btn-warning:hover:not(:disabled) {
  background: var(--warning-hover);
  border-color: var(--warning-hover);
  transform: translateY(-1px);
}

.btn-error {
  background: var(--error-color);
  color: white;
  border-color: var(--error-color);
}

.btn-error:hover:not(:disabled) {
  background: var(--error-hover);
  border-color: var(--error-hover);
  transform: translateY(-1px);
}

.btn-outline {
  background: transparent;
  color: var(--primary-color);
  border-color: var(--primary-color);
}

.btn-outline:hover:not(:disabled) {
  background: var(--primary-color);
  color: white;
  transform: translateY(-1px);
}

.btn-ghost {
  background: transparent;
  color: var(--text-primary);
  border-color: transparent;
}

.btn-ghost:hover:not(:disabled) {
  background: var(--bg-secondary);
  transform: translateY(-1px);
}

.btn-link {
  background: transparent;
  color: var(--primary-color);
  border-color: transparent;
  text-decoration: underline;
}

.btn-link:hover:not(:disabled) {
  color: var(--primary-hover);
  text-decoration: none;
}

/* 特殊样式 */
.btn-block {
  width: 100%;
}

.btn-rounded {
  border-radius: 50px;
}

.btn-loading .btn-text,
.btn-loading .btn-icon {
  opacity: 0.7;
}

/* 加载动画 */
.btn-spinner {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 图标样式 */
.btn-icon {
  display: flex;
  align-items: center;
  font-size: 1em;
}

.btn-small .btn-icon {
  font-size: 0.9em;
}

.btn-large .btn-icon {
  font-size: 1.1em;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .btn-large {
    padding: 0.625rem 1.25rem;
    font-size: 0.95rem;
  }
}
</style>