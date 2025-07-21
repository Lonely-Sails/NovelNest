<template>
  <div 
    :class="[
      'base-card',
      `card-${variant}`,
      {
        'card-hoverable': hoverable,
        'card-selected': selected,
        'card-compact': compact,
        'card-clickable': clickable
      }
    ]"
    @click="handleClick"
  >
    <!-- 卡片头部 -->
    <div v-if="$slots.header || title" class="card-header">
      <slot name="header">
        <h3 v-if="title" class="card-title">{{ title }}</h3>
      </slot>
      <div v-if="$slots.actions" class="card-actions">
        <slot name="actions"></slot>
      </div>
    </div>
    
    <!-- 卡片内容 -->
    <div class="card-body">
      <slot></slot>
    </div>
    
    <!-- 卡片底部 -->
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BaseCard',
  props: {
    // 卡片标题
    title: {
      type: String,
      default: ''
    },
    // 卡片变体：default, primary, secondary, success, warning, error
    variant: {
      type: String,
      default: 'default',
      validator: (value) => ['default', 'primary', 'secondary', 'success', 'warning', 'error'].includes(value)
    },
    // 是否可悬停
    hoverable: {
      type: Boolean,
      default: false
    },
    // 是否选中状态
    selected: {
      type: Boolean,
      default: false
    },
    // 紧凑模式
    compact: {
      type: Boolean,
      default: false
    },
    // 是否可点击
    clickable: {
      type: Boolean,
      default: false
    }
  },
  emits: ['click'],
  setup(props, { emit }) {
    const handleClick = (event) => {
      if (props.clickable) {
        emit('click', event)
      }
    }

    return {
      handleClick
    }
  }
}
</script>

<style scoped>
.base-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.3s ease;
}

.card-hoverable:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
}

.card-clickable {
  cursor: pointer;
}

.card-selected {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.2);
}

.card-compact .card-header,
.card-compact .card-body,
.card-compact .card-footer {
  padding: 0.75rem;
}

/* 卡片变体样式 */
.card-primary {
  border-color: var(--primary-color);
}

.card-primary .card-header {
  background: var(--primary-color);
  color: white;
}

.card-secondary {
  border-color: var(--text-secondary);
}

.card-success {
  border-color: var(--success-color);
}

.card-success .card-header {
  background: var(--success-bg);
  color: var(--success-color);
}

.card-warning {
  border-color: var(--warning-color);
}

.card-warning .card-header {
  background: var(--warning-bg);
  color: var(--warning-color);
}

.card-error {
  border-color: var(--error-color);
}

.card-error .card-header {
  background: var(--error-bg);
  color: var(--error-color);
}

/* 卡片结构 */
.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.25rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.card-title {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.card-actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.card-body {
  padding: 1.25rem;
}

.card-footer {
  padding: 1rem 1.25rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .card-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
  
  .card-actions {
    width: 100%;
    justify-content: flex-end;
  }
}
</style>