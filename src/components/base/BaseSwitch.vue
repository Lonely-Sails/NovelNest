<template>
  <div class="base-switch-wrapper">
    <label :class="switchClasses">
      <input
        type="checkbox"
        :checked="modelValue"
        :disabled="disabled"
        @change="handleChange"
        class="switch-input"
      />
      <span class="switch-slider">
        <span class="switch-thumb">
          <span v-if="showIcon" class="switch-icon">
            {{ modelValue ? checkedIcon : uncheckedIcon }}
          </span>
        </span>
      </span>
    </label>
    
    <!-- 标签文本 -->
    <div v-if="$slots.default || label" class="switch-label">
      <slot>
        <div class="label-content">
          <span class="label-text">{{ label }}</span>
          <span v-if="description" class="label-description">{{ description }}</span>
        </div>
      </slot>
    </div>
  </div>
</template>

<script>
import { computed } from 'vue'

export default {
  name: 'BaseSwitch',
  props: {
    // v-model 绑定值
    modelValue: {
      type: Boolean,
      default: false
    },
    // 标签文本
    label: {
      type: String,
      default: ''
    },
    // 描述文本
    description: {
      type: String,
      default: ''
    },
    // 开关大小：small, medium, large
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
    // 是否显示图标
    showIcon: {
      type: Boolean,
      default: false
    },
    // 选中状态图标
    checkedIcon: {
      type: String,
      default: '✓'
    },
    // 未选中状态图标
    uncheckedIcon: {
      type: String,
      default: '✕'
    }
  },
  emits: ['update:modelValue', 'change'],
  setup(props, { emit }) {
    // 计算开关样式类
    const switchClasses = computed(() => [
      'switch-container',
      `switch-${props.size}`,
      {
        'switch-disabled': props.disabled,
        'switch-checked': props.modelValue
      }
    ])
    
    const handleChange = (event) => {
      const checked = event.target.checked
      emit('update:modelValue', checked)
      emit('change', checked)
    }

    return {
      switchClasses,
      handleChange
    }
  }
}
</script>

<style scoped>
.base-switch-wrapper {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
}

.switch-container {
  position: relative;
  display: inline-block;
  cursor: pointer;
  flex-shrink: 0;
  /* 确保在webkit浏览器中正确显示 */
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  /* 移除webkit浏览器的默认点击高亮 */
  -webkit-tap-highlight-color: transparent;
}

.switch-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.switch-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
  /* 完全隐藏webkit浏览器中的原生样式 */
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  /* 确保不占用任何空间 */
  margin: 0;
  padding: 0;
  border: none;
  outline: none;
  /* 防止在某些浏览器中显示 */
  visibility: hidden;
}

.switch-slider {
  position: relative;
  display: block;
  background-color: var(--border-color);
  border-radius: 50px;
  transition: all 0.3s ease;
  /* 确保在webkit浏览器中正确显示 */
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  /* 防止webkit浏览器的默认样式干扰 */
  -webkit-tap-highlight-color: transparent;
}

.switch-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  background-color: white;
  border-radius: 50%;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  /* 确保在webkit浏览器中正确显示 */
  -webkit-transform: translateZ(0);
  transform: translateZ(0);
  /* 防止webkit浏览器的默认样式 */
  border: none;
  outline: none;
}

.switch-icon {
  font-size: 0.7em;
  color: var(--text-secondary);
  font-weight: bold;
}

/* 开关大小 */
.switch-small .switch-slider {
  width: 36px;
  height: 20px;
}

.switch-small .switch-thumb {
  width: 16px;
  height: 16px;
  font-size: 0.7rem;
}

.switch-medium .switch-slider {
  width: 44px;
  height: 24px;
}

.switch-medium .switch-thumb {
  width: 20px;
  height: 20px;
  font-size: 0.8rem;
}

.switch-large .switch-slider {
  width: 52px;
  height: 28px;
}

.switch-large .switch-thumb {
  width: 24px;
  height: 24px;
  font-size: 0.9rem;
}

/* 选中状态 */
.switch-checked .switch-slider {
  background-color: var(--primary-color);
}

.switch-small.switch-checked .switch-thumb {
  transform: translateX(16px);
}

.switch-medium.switch-checked .switch-thumb {
  transform: translateX(20px);
}

.switch-large.switch-checked .switch-thumb {
  transform: translateX(24px);
}

.switch-checked .switch-icon {
  color: var(--primary-color);
}

/* 悬停效果 */
.switch-container:hover:not(.switch-disabled) .switch-slider {
  box-shadow: 0 0 0 4px rgba(0, 123, 255, 0.1);
}

.switch-container:hover:not(.switch-disabled) .switch-thumb {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

/* 焦点状态 */
.switch-input:focus + .switch-slider {
  box-shadow: 0 0 0 4px rgba(0, 123, 255, 0.2);
}

/* 标签样式 */
.switch-label {
  flex: 1;
  min-width: 0;
}

.label-content {
  display: flex;
  flex-direction: column;
}

.label-text {
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.9rem;
  line-height: 1.4;
}

.label-description {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.3;
  margin-top: 0.25rem;
}

/* webkit浏览器特定样式优化 */
@supports (-webkit-appearance: none) {
  .switch-input {
    /* 在支持webkit的浏览器中进一步确保隐藏 */
    position: absolute !important;
    left: -9999px !important;
    opacity: 0 !important;
    pointer-events: none !important;
  }
  
  .switch-container {
    /* 确保容器不会被原生样式影响 */
    outline: none;
    border: none;
    background: none;
  }
  
  .switch-slider {
    /* 确保滑块样式不被覆盖 */
    border: none;
    outline: none;
  }
}

/* 针对Safari浏览器的特殊处理 */
@media screen and (-webkit-min-device-pixel-ratio: 0) {
  .switch-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
  }
  
  .switch-input::-webkit-slider-runnable-track {
    -webkit-appearance: none;
    appearance: none;
  }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .base-switch-wrapper {
    gap: 0.5rem;
  }
  
  .label-text {
    font-size: 0.85rem;
  }
  
  .label-description {
    font-size: 0.75rem;
  }
}
</style>