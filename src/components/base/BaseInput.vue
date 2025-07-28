<template>
  <div class="base-input-wrapper">
    <!-- 标签 -->
    <label v-if="label" :for="inputId" class="input-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <!-- 输入框容器 -->
    <div :class="containerClasses">
      <!-- 前置图标 -->
      <span v-if="prefixIcon || $slots.prefix" class="input-prefix">
        <slot name="prefix">{{ prefixIcon }}</slot>
      </span>

      <!-- 输入框 -->
      <component 
        :class="fieldClasses"
        :is="type === 'textarea' ? 'textarea' : 'input'" 
        :id="inputId" 
        ref="inputRef" 
        :type="type === 'textarea' ? undefined : type"
        :value="modelValue" 
        :placeholder="placeholder" 
        :disabled="disabled" 
        :readonly="readonly" 
        :maxlength="maxlength"
        :rows="rows" 
        :cols="cols" 
        @input="handleInput" 
        @change="handleChange" 
        @focus="handleFocus" 
        @blur="handleBlur"
        @keydown="handleKeydown" 
        v-bind="$attrs" 
      />

      <!-- 后置区域 -->
      <span v-if="suffixIcon || $slots.suffix || (clearable && !disabled && !readonly) || loading" class="input-suffix">
        <!-- 加载指示器 -->
        <span v-if="loading" class="loading-spinner">
          <svg class="spinner" viewBox="0 0 24 24">
            <circle class="spinner-circle" cx="12" cy="12" r="10" fill="none" stroke="currentColor" stroke-width="2" />
          </svg>
        </span>

        <!-- 清除按钮 -->
        <button v-if="showClearButton && modelValue && !loading" @click="clearInput" class="clear-button" type="button"
          tabindex="-1">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M9 3L3 9M3 3L9 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>

        <!-- 后置图标 -->
        <span v-if="($slots.suffix || suffixIcon) && !loading" class="suffix-content">
          <slot name="suffix">{{ suffixIcon }}</slot>
        </span>
      </span>
    </div>

    <!-- 消息提示 -->
    <div v-if="helpText || errorMessage || successMessage" class="input-message">
      <span v-if="errorMessage" class="error-message">{{ errorMessage }}</span>
      <span v-else-if="successMessage" class="success-message">{{ successMessage }}</span>
      <span v-else class="help-message">{{ helpText }}</span>
    </div>
  </div>
</template>

<script>
import { ref, computed, nextTick } from 'vue'

/**
 * BaseInput - 基础输入框组件
 * 支持文本输入、文本域、前后置图标、清除按钮等功能
 */
export default {
  name: 'BaseInput',
  inheritAttrs: false,
  props: {
    modelValue: { type: [String, Number], default: '' },
    type: { type: String, default: 'text' },
    label: { type: String, default: '' },
    placeholder: { type: String, default: '' },
    size: { type: String, default: 'medium', validator: v => ['small', 'medium', 'large'].includes(v) },
    disabled: { type: Boolean, default: false },
    readonly: { type: Boolean, default: false },
    required: { type: Boolean, default: false },
    loading: { type: Boolean, default: false },
    maxlength: { type: [String, Number], default: '' },
    rows: { type: [String, Number], default: 3 },
    cols: { type: [String, Number], default: '' },
    prefixIcon: { type: String, default: '' },
    suffixIcon: { type: String, default: '' },
    clearable: { type: Boolean, default: false },
    helpText: { type: String, default: '' },
    errorMessage: { type: String, default: '' },
    successMessage: { type: String, default: '' }
  },
  emits: ['update:modelValue', 'change', 'focus', 'blur', 'clear', 'keydown'],
  setup(props, { emit }) {
    const inputRef = ref(null)
    const focused = ref(false)

    // 生成唯一ID
    const inputId = `input-${Math.random().toString(36).substring(2, 11)}`

    // 计算容器样式类
    const containerClasses = computed(() => [
      'input-container',
      `input-${props.size}`,
      {
        'input-error': !!props.errorMessage,
        'input-success': !!props.successMessage,
        'input-disabled': props.disabled,
        'input-focused': focused.value
      }
    ])

    // 计算输入框样式类
    const fieldClasses = computed(() => [
      'input-field',
      {
        'has-prefix': props.prefixIcon || props.$slots?.prefix,
        'has-suffix': props.suffixIcon || props.$slots?.suffix || (props.clearable && !props.disabled && !props.readonly) || props.loading
      }
    ])

    // 计算是否显示清除按钮
    const showClearButton = computed(() => props.clearable && !props.disabled && !props.readonly)

    // ===== 事件处理 =====
    const handleInput = (event) => {
      emit('update:modelValue', event.target.value)
    }

    const handleChange = (event) => {
      emit('change', event.target.value)
    }

    const handleFocus = (event) => {
      focused.value = true
      emit('focus', event)
    }

    const handleBlur = (event) => {
      focused.value = false
      emit('blur', event)
    }

    const handleKeydown = (event) => {
      emit('keydown', event)
    }

    const clearInput = () => {
      emit('update:modelValue', '')
      emit('clear')
      nextTick(() => inputRef.value?.focus())
    }

    // ===== 暴露的方法 =====
    const focus = () => inputRef.value?.focus()
    const blur = () => inputRef.value?.blur()
    const select = () => inputRef.value?.select()

    return {
      inputRef,
      focused,
      inputId,
      containerClasses,
      fieldClasses,
      showClearButton,
      handleInput,
      handleChange,
      handleFocus,
      handleBlur,
      handleKeydown,
      clearInput,
      focus,
      blur,
      select
    }
  }
}
</script>

<style scoped>
.base-input-wrapper {
  width: 100%;
  max-width: 100%;
  /* 确保不会超出容器 */
}

.input-label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.9rem;
}

.required-mark {
  color: var(--error-color);
  margin-left: 0.25rem;
}

.input-container {
  position: relative;
  display: flex;
  align-items: center;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  transition: all 0.3s ease;
  width: 100%;
  max-width: 100%;
  box-sizing: border-box;
  /* 确保边框包含在宽度内 */
}

.input-container:hover:not(.input-disabled) {
  border-color: var(--primary-color);
}

.input-focused {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.2);
}

.input-error {
  border-color: var(--error-color);
}

.input-error.input-focused {
  box-shadow: 0 0 0 2px rgba(220, 53, 69, 0.2);
}

.input-success {
  border-color: var(--success-color);
}

.input-success.input-focused {
  box-shadow: 0 0 0 2px rgba(40, 167, 69, 0.2);
}

.input-disabled {
  background: var(--bg-secondary);
  opacity: 0.6;
  cursor: not-allowed;
}

/* 输入框大小 */
.input-small {
  min-height: 32px;
}

.input-small .input-field {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
}

.input-medium {
  min-height: 40px;
}

.input-medium .input-field {
  padding: 0.5rem 0.75rem;
  font-size: 0.9rem;
}

.input-large {
  min-height: 48px;
}

.input-large .input-field {
  padding: 0.75rem 1rem;
  font-size: 1rem;
}

/* 输入框样式 */
.input-field {
  flex: 1;
  border: none;
  outline: none;
  box-shadow: none;
  background: transparent;
  color: var(--text-primary);
  font-family: inherit;
  width: 100%;
  min-width: 0;
  /* 允许收缩 */
  box-sizing: border-box;
}

/* 隐藏浏览器自带的清除按钮 */
.input-field::-webkit-search-cancel-button,
.input-field::-webkit-search-decoration,
.input-field::-ms-clear,
.input-field::-ms-reveal {
  display: none;
  width: 0;
  height: 0;
}

.input-field[type="search"]::-webkit-search-cancel-button,
.input-field[type="search"]::-webkit-search-decoration {
  -webkit-appearance: none;
}

.input-field::placeholder {
  color: var(--text-muted);
}

.input-field:disabled {
  cursor: not-allowed;
}

.input-field.has-prefix {
  padding-left: 0;
}

.input-field.has-suffix {
  padding-right: 0;
}

/* 文本域特殊样式 */
textarea.input-field {
  resize: vertical;
  min-height: auto;
  line-height: 1.5;
}

/* 前后缀样式 */
.input-prefix {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
  flex-shrink: 0;
  padding-left: 0.75rem;
  padding-right: 0.5rem;
}

.input-suffix {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
  flex-shrink: 0;
  padding-right: 0.75rem;
  padding-left: 0.5rem;
  gap: 0.5rem;
  /* 清除按钮和后置图标之间的间距 */
}

.suffix-content {
  display: flex;
  align-items: center;
}

/* 加载指示器样式 */
.loading-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 16px;
  height: 16px;
  color: var(--primary-color);
  animation: spin 1s linear infinite;
}

.spinner-circle {
  stroke-dasharray: 31.416;
  /* 2 * π * 10 */
  stroke-dashoffset: 31.416;
  animation: dash 2s ease-in-out infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 150;
    stroke-dashoffset: 0;
  }

  50% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -35;
  }

  100% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -124;
  }
}

.clear-button {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 50%;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.clear-button:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* 消息样式 */
.input-message {
  margin-top: 0.5rem;
  font-size: 0.8rem;
  line-height: 1.4;
}

.error-message {
  color: var(--error-color);
}

.success-message {
  color: var(--success-color);
}

.help-message {
  color: var(--text-secondary);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .input-large .input-field {
    padding: 0.625rem 0.875rem;
    font-size: 0.95rem;
  }
}
</style>