<template>
  <div class="base-input-wrapper">
    <!-- 标签 -->
    <label v-if="label" :for="inputId" class="input-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>
    
    <!-- 输入框容器 -->
    <div 
      :class="[
        'input-container',
        `input-${size}`,
        {
          'input-error': hasError,
          'input-success': hasSuccess,
          'input-disabled': disabled,
          'input-focused': focused
        }
      ]"
    >
      <!-- 前置图标 -->
      <span v-if="$slots.prefix || prefixIcon" class="input-prefix">
        <slot name="prefix">{{ prefixIcon }}</slot>
      </span>
      
      <!-- 输入框 -->
      <component
        :is="inputComponent"
        :id="inputId"
        ref="inputRef"
        :class="['input-field', { 'has-prefix': $slots.prefix || prefixIcon, 'has-suffix': $slots.suffix || suffixIcon }]"
        :type="inputType"
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
      
      <!-- 后置图标 -->
      <span v-if="$slots.suffix || suffixIcon || showClearButton" class="input-suffix">
        <button
          v-if="showClearButton && modelValue"
          @click="clearInput"
          class="clear-button"
          type="button"
          tabindex="-1"
        >
          ✕
        </button>
        <slot name="suffix">{{ suffixIcon }}</slot>
      </span>
    </div>
    
    <!-- 帮助文本或错误信息 -->
    <div v-if="helpText || errorMessage || successMessage" class="input-message">
      <span v-if="errorMessage" class="error-message">{{ errorMessage }}</span>
      <span v-else-if="successMessage" class="success-message">{{ successMessage }}</span>
      <span v-else class="help-message">{{ helpText }}</span>
    </div>
  </div>
</template>

<script>
import { ref, computed, nextTick } from 'vue'

export default {
  name: 'BaseInput',
  inheritAttrs: false,
  props: {
    // v-model 绑定值
    modelValue: {
      type: [String, Number],
      default: ''
    },
    // 输入框类型
    type: {
      type: String,
      default: 'text'
    },
    // 标签文本
    label: {
      type: String,
      default: ''
    },
    // 占位符
    placeholder: {
      type: String,
      default: ''
    },
    // 输入框大小：small, medium, large
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
    // 是否只读
    readonly: {
      type: Boolean,
      default: false
    },
    // 是否必填
    required: {
      type: Boolean,
      default: false
    },
    // 最大长度
    maxlength: {
      type: [String, Number],
      default: ''
    },
    // 文本域行数
    rows: {
      type: [String, Number],
      default: 3
    },
    // 文本域列数
    cols: {
      type: [String, Number],
      default: ''
    },
    // 前置图标
    prefixIcon: {
      type: String,
      default: ''
    },
    // 后置图标
    suffixIcon: {
      type: String,
      default: ''
    },
    // 是否显示清除按钮
    clearable: {
      type: Boolean,
      default: false
    },
    // 帮助文本
    helpText: {
      type: String,
      default: ''
    },
    // 错误信息
    errorMessage: {
      type: String,
      default: ''
    },
    // 成功信息
    successMessage: {
      type: String,
      default: ''
    }
  },
  emits: ['update:modelValue', 'change', 'focus', 'blur', 'clear', 'keydown'],
  setup(props, { emit }) {
    const inputRef = ref(null)
    const focused = ref(false)
    
    // 生成唯一ID
    const inputId = `input-${Math.random().toString(36).substr(2, 9)}`
    
    // 计算输入框组件类型
    const inputComponent = computed(() => {
      return props.type === 'textarea' ? 'textarea' : 'input'
    })
    
    // 计算输入框类型
    const inputType = computed(() => {
      return props.type === 'textarea' ? undefined : props.type
    })
    
    // 计算状态
    const hasError = computed(() => !!props.errorMessage)
    const hasSuccess = computed(() => !!props.successMessage)
    const showClearButton = computed(() => props.clearable && !props.disabled && !props.readonly)
    
    // 事件处理
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
      nextTick(() => {
        inputRef.value?.focus()
      })
    }
    
    // 暴露方法
    const focus = () => {
      inputRef.value?.focus()
    }
    
    const blur = () => {
      inputRef.value?.blur()
    }
    
    const select = () => {
      inputRef.value?.select()
    }

    return {
      inputRef,
      focused,
      inputId,
      inputComponent,
      inputType,
      hasError,
      hasSuccess,
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
  background: transparent;
  color: var(--text-primary);
  font-family: inherit;
  width: 100%;
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
.input-prefix,
.input-suffix {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
  flex-shrink: 0;
}

.input-prefix {
  padding-left: 0.75rem;
  padding-right: 0.5rem;
}

.input-suffix {
  padding-right: 0.75rem;
  padding-left: 0.5rem;
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