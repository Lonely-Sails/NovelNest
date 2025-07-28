<template>
  <div class="base-select-wrapper">
    <!-- 标签 -->
    <label v-if="label" :for="selectId" class="select-label">
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <!-- 选择框 -->
    <div 
      :id="selectId" 
      ref="selectRef"
      :class="containerClasses" 
      :tabindex="disabled ? -1 : 0"
      @click="handleContainerClick"
      @keydown="handleKeydown" 
      @focus="handleFocus" 
      @blur="handleBlur"
    >
      <!-- 前置图标 -->
      <span v-if="hasPrefix" class="select-prefix">
        <slot name="prefix">{{ prefixIcon }}</slot>
      </span>

      <!-- 显示区域 -->
      <div :class="displayClasses">
        <span v-if="displayText" class="select-text">{{ displayText }}</span>
        <span v-else class="select-placeholder">{{ placeholder }}</span>
      </div>

      <!-- 后置区域 -->
      <div class="select-suffix">
        <!-- 清除按钮 -->
        <button v-if="showClearButton" @click.stop="clearSelection" class="clear-button" type="button" tabindex="-1">
          ✕
        </button>

        <!-- 后置图标 -->
        <span v-if="hasSuffix" class="suffix-icon">
          <slot name="suffix">{{ suffixIcon }}</slot>
        </span>

        <!-- 下拉箭头 -->
        <span :class="arrowClasses">{{ dropdownIcon }}</span>
      </div>
    </div>

    <!-- 下拉选项（传送到 body） -->
    <teleport to="body">
      <transition name="dropdown">
        <div v-if="isOpen" class="select-dropdown" :style="dropdownStyle">
          <div class="dropdown-content">
            <!-- 搜索框 -->
            <div v-if="filterable" class="search-container">
              <input ref="searchRef" v-model="searchQuery" type="text" class="search-input"
                :placeholder="searchPlaceholder" @keydown.stop />
            </div>

            <!-- 选项列表 -->
            <div class="options-container">
              <div v-for="(option, index) in filteredOptions" :key="getOptionKey(option, index)"
                :class="getOptionClasses(option, index)" @click="selectOption(option)"
                @mouseenter="highlightedIndex = index">
                <slot name="option" :option="option" :index="index">
                  <span class="option-text">{{ getOptionLabel(option) }}</span>
                  <span v-if="isSelected(option)" class="option-check">✓</span>
                </slot>
              </div>

              <!-- 无选项提示 -->
              <div v-if="!filteredOptions.length" class="no-options">
                {{ noDataText }}
              </div>
            </div>
          </div>
        </div>
      </transition>
    </teleport>

    <!-- 消息提示 -->
    <div v-if="hasMessage" class="select-message">
      <span v-if="errorMessage" class="error-message">{{ errorMessage }}</span>
      <span v-else-if="successMessage" class="success-message">{{ successMessage }}</span>
      <span v-else class="help-message">{{ helpText }}</span>
    </div>
  </div>
</template>

<script>
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'

/**
 * BaseSelect - 基础选择器组件
 * 支持单选、搜索、清除、键盘导航等功能
 */
export default {
  name: 'BaseSelect',
  props: {
    modelValue: { type: [String, Number, Object, Array], default: null },
    options: { type: Array, default: () => [] },
    labelKey: { type: String, default: 'label' },
    valueKey: { type: String, default: 'value' },
    disabledKey: { type: String, default: 'disabled' },
    label: { type: String, default: '' },
    placeholder: { type: String, default: '请选择' },
    size: { type: String, default: 'medium', validator: v => ['small', 'medium', 'large'].includes(v) },
    disabled: { type: Boolean, default: false },
    required: { type: Boolean, default: false },
    clearable: { type: Boolean, default: false },
    filterable: { type: Boolean, default: false },
    searchPlaceholder: { type: String, default: '搜索选项' },
    prefixIcon: { type: String, default: '' },
    suffixIcon: { type: String, default: '' },
    dropdownIcon: { type: String, default: '▼' },
    noDataText: { type: String, default: '暂无数据' },
    helpText: { type: String, default: '' },
    errorMessage: { type: String, default: '' },
    successMessage: { type: String, default: '' }
  },
  emits: ['update:modelValue', 'change', 'focus', 'blur', 'clear'],
  setup(props, { emit }) {
    // ===== 响应式数据 =====
    const selectRef = ref(null)
    const searchRef = ref(null)
    const focused = ref(false)
    const isOpen = ref(false)
    const searchQuery = ref('')
    const highlightedIndex = ref(-1)
    const dropdownStyle = ref({})

    const selectId = `select-${Math.random().toString(36).substring(2, 11)}`

    // ===== 计算属性 =====
    const hasMessage = computed(() => !!(props.helpText || props.errorMessage || props.successMessage))
    const hasPrefix = computed(() => !!(props.prefixIcon || props.$slots?.prefix))
    const hasSuffix = computed(() => !!(props.suffixIcon || props.$slots?.suffix))
    const showClearButton = computed(() => props.clearable && props.modelValue && !props.disabled)

    const containerClasses = computed(() => [
      'select-container',
      `select-${props.size}`,
      {
        'select-error': !!props.errorMessage,
        'select-success': !!props.successMessage,
        'select-disabled': props.disabled,
        'select-focused': focused.value,
        'select-open': isOpen.value
      }
    ])

    const displayClasses = computed(() => [
      'select-display',
      { 'has-prefix': hasPrefix.value, 'has-suffix': true }
    ])

    const arrowClasses = computed(() => [
      'dropdown-arrow',
      { 'arrow-up': isOpen.value }
    ])

    // ===== 选项处理工具函数 =====
    const getOptionLabel = (option) => {
      if (typeof option === 'string' || typeof option === 'number') return option
      return option[props.labelKey] || option
    }

    const getOptionValue = (option) => {
      if (typeof option === 'string' || typeof option === 'number') return option
      return option[props.valueKey] !== undefined ? option[props.valueKey] : option
    }

    const getOptionKey = (option, index) => {
      const value = getOptionValue(option)
      return value !== null && value !== undefined ? value : index
    }

    const isOptionDisabled = (option) => {
      return typeof option === 'object' && option !== null && !!option[props.disabledKey]
    }

    const isSelected = (option) => getOptionValue(option) === props.modelValue

    const getOptionClasses = (option, index) => [
      'select-option',
      {
        'option-selected': isSelected(option),
        'option-disabled': isOptionDisabled(option),
        'option-highlighted': highlightedIndex.value === index
      }
    ]

    const displayText = computed(() => {
      if (!props.modelValue) return ''
      const selectedOption = props.options.find(option => getOptionValue(option) === props.modelValue)
      return selectedOption ? getOptionLabel(selectedOption) : props.modelValue
    })

    const filteredOptions = computed(() => {
      if (!props.filterable || !searchQuery.value) return props.options
      const query = searchQuery.value.toLowerCase()
      return props.options.filter(option => {
        const label = getOptionLabel(option).toString().toLowerCase()
        return label.includes(query)
      })
    })

    // ===== 位置计算 =====
    const updateDropdownPosition = () => {
      if (!selectRef.value) return

      const rect = selectRef.value.getBoundingClientRect()
      const viewportHeight = window.innerHeight
      const maxHeight = 240
      const spaceBelow = viewportHeight - rect.bottom
      const spaceAbove = rect.top

      let top = rect.bottom
      let height = Math.min(maxHeight, spaceBelow - 10)

      if (spaceBelow < 100 && spaceAbove > spaceBelow) {
        top = rect.top - Math.min(maxHeight, spaceAbove - 10)
        height = Math.min(maxHeight, spaceAbove - 10)
      }

      dropdownStyle.value = {
        top: `${top}px`,
        left: `${rect.left}px`,
        width: `${Math.max(rect.width, 120)}px`,
        minWidth: `${Math.max(rect.width, 120)}px`,
        maxHeight: `${height}px`
      }
    }

    // ===== 核心交互方法 =====
    const toggleDropdown = () => {
      if (props.disabled) return
      isOpen.value = !isOpen.value

      if (isOpen.value) {
        updateDropdownPosition()
        nextTick(() => {
          updateDropdownPosition()
          if (props.filterable && searchRef.value) {
            searchRef.value.focus()
          }
        })
      } else {
        searchQuery.value = ''
        highlightedIndex.value = -1
      }
    }

    const handleContainerClick = (event) => {
      // 确保点击容器的任何地方都能打开下拉框
      // 除非点击的是清除按钮（已经有 @click.stop）
      if (props.disabled) return
      toggleDropdown()
    }

    const selectOption = (option) => {
      if (isOptionDisabled(option)) return
      const value = getOptionValue(option)
      emit('update:modelValue', value)
      emit('change', value, option)
      isOpen.value = false
      searchQuery.value = ''
      highlightedIndex.value = -1
      nextTick(() => selectRef.value?.focus())
    }

    const clearSelection = () => {
      emit('update:modelValue', null)
      emit('clear')
      nextTick(() => selectRef.value?.focus())
    }

    // ===== 事件处理 =====
    const handleFocus = (event) => {
      focused.value = true
      emit('focus', event)
    }

    const handleBlur = (event) => {
      setTimeout(() => {
        focused.value = false
        isOpen.value = false
        searchQuery.value = ''
        highlightedIndex.value = -1
        emit('blur', event)
      }, 150)
    }

    const handleKeydown = (event) => {
      if (props.disabled) return

      switch (event.key) {
        case 'Enter':
        case ' ':
          event.preventDefault()
          if (!isOpen.value) {
            toggleDropdown()
          } else if (highlightedIndex.value >= 0) {
            selectOption(filteredOptions.value[highlightedIndex.value])
          }
          break
        case 'Escape':
          if (isOpen.value) {
            event.preventDefault()
            isOpen.value = false
            searchQuery.value = ''
            highlightedIndex.value = -1
          }
          break
        case 'ArrowDown':
          event.preventDefault()
          if (!isOpen.value) toggleDropdown()
          else
            highlightedIndex.value = Math.min(highlightedIndex.value + 1, filteredOptions.value.length - 1)
          break
        case 'ArrowUp':
          event.preventDefault()
          if (isOpen.value) highlightedIndex.value = Math.max(highlightedIndex.value - 1, 0)
          break
      }
    }

    // ===== 外部点击处理 =====
    const handleClickOutside = (event) => {
      if (!selectRef.value || !isOpen.value) return
      if (selectRef.value.contains(event.target)) return

      const dropdowns = document.querySelectorAll('.select-dropdown')
      for (const dropdown of dropdowns) {
        if (dropdown.contains(event.target)) return
      }

      isOpen.value = false
      searchQuery.value = ''
      highlightedIndex.value = -1
    }

    const handleWindowEvents = () => updateDropdownPosition()

    // ===== 生命周期 =====
    onMounted(() => {
      document.addEventListener('click', handleClickOutside)
      window.addEventListener('scroll', handleWindowEvents, true)
      window.addEventListener('resize', handleWindowEvents)
    })

    onUnmounted(() => {
      document.removeEventListener('click', handleClickOutside)
      window.removeEventListener('scroll', handleWindowEvents, true)
      window.removeEventListener('resize', handleWindowEvents)
    })

    // ===== 返回模板所需的数据和方法 =====
    return {
      selectRef, searchRef, focused, isOpen, searchQuery, highlightedIndex, dropdownStyle, selectId,
      hasMessage, hasPrefix, hasSuffix, containerClasses, displayClasses, arrowClasses, showClearButton, displayText, filteredOptions,
      getOptionLabel, getOptionValue, getOptionKey, isOptionDisabled, isSelected, getOptionClasses,
      toggleDropdown, handleContainerClick, selectOption, clearSelection, handleKeydown, handleFocus, handleBlur
    }
  }
}
</script>

<style scoped>
.base-select-wrapper {
  position: relative;
  display: inline-block; /* 改为 inline-block，让宽度由内容决定 */
}

.select-label {
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

.select-container {
  position: relative;
  display: flex;
  align-items: center;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  transition: all 0.3s ease;
  cursor: pointer;
  width: 100%;
  min-width: 0;
  outline: none;
}

.select-container:hover:not(.select-disabled) {
  border-color: var(--primary-color);
}

.select-focused {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.select-open {
  border-color: var(--primary-color);
}

.select-error {
  border-color: var(--error-color);
}

.select-error.select-focused {
  box-shadow: 0 0 0 3px rgba(220, 53, 69, 0.1);
}

.select-success {
  border-color: var(--success-color);
}

.select-success.select-focused {
  box-shadow: 0 0 0 3px rgba(40, 167, 69, 0.1);
}

.select-disabled {
  background: var(--bg-secondary);
  opacity: 0.6;
  cursor: not-allowed;
}

/* 选择框大小 */
.select-small {
  min-height: 32px;
  width: 140px; /* 固定宽度，避免被拉伸 */
  max-width: 140px;
  flex-shrink: 0; /* 防止收缩 */
}

.select-small .select-display {
  padding: 0.375rem 0.75rem;
  font-size: 0.85rem;
}

.select-medium {
  min-height: 40px;
  width: 160px; /* 固定宽度，避免被拉伸 */
  max-width: 160px;
  flex-shrink: 0; /* 防止收缩 */
}

.select-medium .select-display {
  padding: 0.5rem 0.75rem;
  font-size: 0.9rem;
}

.select-large {
  min-height: 48px;
  width: 200px; /* 固定宽度，避免被拉伸 */
  max-width: 200px;
  flex-shrink: 0; /* 防止收缩 */
}

.select-large .select-display {
  padding: 0.75rem 1rem;
  font-size: 1rem;
}

/* 显示区域 */
.select-display {
  flex: 1;
  outline: none;
  color: var(--text-primary);
  font-family: inherit;
  width: 100%;
  display: flex;
  align-items: center;
}

.select-display.has-prefix {
  padding-left: 0;
}

.select-display.has-suffix {
  padding-right: 0;
}

.select-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.select-placeholder {
  flex: 1;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 前后缀样式 */
.select-prefix {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
  flex-shrink: 0;
  padding-left: 0.75rem;
  padding-right: 0.5rem;
  pointer-events: none; /* 允许点击事件穿透 */
}

.select-suffix {
  display: flex;
  align-items: center;
  color: var(--text-secondary);
  font-size: 0.9rem;
  flex-shrink: 0;
  padding-right: 0.75rem;
  padding-left: 0.5rem;
  gap: 0.5rem;
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
  pointer-events: auto; /* 确保清除按钮可以接收点击事件 */
}

.clear-button:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.suffix-icon {
  display: flex;
  align-items: center;
  pointer-events: none; /* 允许点击事件穿透 */
}

.dropdown-arrow {
  transition: transform 0.3s ease;
  font-size: 0.8rem;
  color: var(--text-secondary);
  pointer-events: none; /* 允许点击事件穿透 */
}

.arrow-up {
  transform: rotate(180deg);
}

/* 下拉框 */
.select-dropdown {
  position: fixed;
  z-index: 9999;
  background: var(--bg-primary);
  border: 1px solid var(--primary-color);
  border-radius: 6px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  overflow: hidden;
  max-height: 240px;
}

.dropdown-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  max-height: inherit;
  width: 100%;
}

/* 搜索框 */
.search-container {
  padding: 0.75rem;
  border-bottom: 1px solid var(--border-color);
}

.search-input {
  width: 100%;
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  outline: none;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.85rem;
  transition: border-color 0.2s ease;
}

.search-input:focus {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
}

.search-input::placeholder {
  color: var(--text-muted);
}

/* 选项容器 */
.options-container {
  flex: 1;
  overflow-y: auto;
  max-height: inherit;
  width: 100%;
}

/* 选项样式 */
.select-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.625rem 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;
  border-bottom: 1px solid transparent;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
}

.select-option:hover:not(.option-disabled) {
  background: var(--bg-secondary);
}

.option-highlighted {
  background: var(--bg-secondary);
}

.option-selected {
  background: rgba(0, 123, 255, 0.08);
  color: var(--primary-color);
  font-weight: 500;
}

.option-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.option-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.option-check {
  color: var(--primary-color);
  font-weight: bold;
  margin-left: 0.5rem;
}

/* 无选项提示 */
.no-options {
  padding: 1rem;
  text-align: center;
  color: var(--text-muted);
  font-size: 0.9rem;
}

/* 消息样式 */
.select-message {
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

/* 下拉动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  transform-origin: top;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: scaleY(0.9) translateY(-4px);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .select-dropdown {
    max-height: 150px;
  }

  .select-large .select-display {
    padding: 0.625rem 0.875rem;
    font-size: 0.95rem;
  }
}
</style>