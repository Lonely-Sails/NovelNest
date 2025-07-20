<template>
  <div class="search-bar" :class="{ 'search-bar-focused': focused }">
    <div class="search-input-container">
      <div class="search-icon">
        🔍
      </div>
      <input
        ref="inputRef"
        v-model="searchQuery"
        @input="handleInput"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeydown"
        type="text"
        :placeholder="placeholder"
        class="search-input"
        :disabled="disabled"
      />
      <button
        v-if="searchQuery && clearable"
        @click="clearSearch"
        class="search-clear"
        type="button"
      >
        ✕
      </button>
      <button
        v-if="showSearchButton"
        @click="handleSearch"
        class="search-button"
        type="button"
        :disabled="disabled || loading"
      >
        <span v-if="loading" class="search-loading">⟳</span>
        <span v-else>搜索</span>
      </button>
    </div>
    
    <!-- 搜索建议下拉框 -->
    <div v-if="showSuggestions && suggestions.length > 0" class="search-suggestions">
      <div
        v-for="(suggestion, index) in suggestions"
        :key="index"
        @click="selectSuggestion(suggestion)"
        @mouseenter="highlightedIndex = index"
        class="suggestion-item"
        :class="{ 'suggestion-highlighted': index === highlightedIndex }"
      >
        <div class="suggestion-icon">{{ suggestion.icon || '📖' }}</div>
        <div class="suggestion-content">
          <div class="suggestion-title">{{ suggestion.title || suggestion }}</div>
          <div v-if="suggestion.subtitle" class="suggestion-subtitle">
            {{ suggestion.subtitle }}
          </div>
        </div>
      </div>
    </div>
    
    <!-- 搜索历史 -->
    <div v-if="showHistory && searchHistory.length > 0 && !searchQuery" class="search-history">
      <div class="history-header">
        <span>搜索历史</span>
        <button @click="clearHistory" class="history-clear">清除</button>
      </div>
      <div class="history-items">
        <div
          v-for="(item, index) in searchHistory"
          :key="index"
          @click="selectHistory(item)"
          class="history-item"
        >
          <span class="history-icon">🕒</span>
          <span class="history-text">{{ item }}</span>
          <button
            @click.stop="removeHistoryItem(index)"
            class="history-remove"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, watch, nextTick } from 'vue'

export default {
  name: 'SearchBar',
  props: {
    modelValue: {
      type: String,
      default: ''
    },
    placeholder: {
      type: String,
      default: '搜索...'
    },
    disabled: {
      type: Boolean,
      default: false
    },
    loading: {
      type: Boolean,
      default: false
    },
    clearable: {
      type: Boolean,
      default: true
    },
    showSearchButton: {
      type: Boolean,
      default: false
    },
    suggestions: {
      type: Array,
      default: () => []
    },
    showSuggestions: {
      type: Boolean,
      default: false
    },
    showHistory: {
      type: Boolean,
      default: true
    },
    maxHistory: {
      type: Number,
      default: 10
    },
    debounceDelay: {
      type: Number,
      default: 300
    }
  },
  emits: ['update:modelValue', 'search', 'input', 'focus', 'blur', 'clear'],
  setup(props, { emit }) {
    const inputRef = ref(null)
    const focused = ref(false)
    const searchQuery = ref(props.modelValue)
    const highlightedIndex = ref(-1)
    const searchHistory = ref([])
    let debounceTimer = null

    // 从本地存储加载搜索历史
    const loadSearchHistory = () => {
      try {
        const saved = localStorage.getItem('searchHistory')
        if (saved) {
          searchHistory.value = JSON.parse(saved)
        }
      } catch (error) {
        console.error('加载搜索历史失败:', error)
      }
    }

    // 保存搜索历史到本地存储
    const saveSearchHistory = () => {
      try {
        localStorage.setItem('searchHistory', JSON.stringify(searchHistory.value))
      } catch (error) {
        console.error('保存搜索历史失败:', error)
      }
    }

    // 添加到搜索历史
    const addToHistory = (query) => {
      if (!query.trim()) return
      
      // 移除重复项
      const index = searchHistory.value.indexOf(query)
      if (index > -1) {
        searchHistory.value.splice(index, 1)
      }
      
      // 添加到开头
      searchHistory.value.unshift(query)
      
      // 限制历史记录数量
      if (searchHistory.value.length > props.maxHistory) {
        searchHistory.value = searchHistory.value.slice(0, props.maxHistory)
      }
      
      saveSearchHistory()
    }

    // 处理输入
    const handleInput = (event) => {
      const value = event.target.value
      searchQuery.value = value
      emit('update:modelValue', value)
      emit('input', value)

      // 防抖处理
      if (debounceTimer) {
        clearTimeout(debounceTimer)
      }
      
      debounceTimer = setTimeout(() => {
        if (value.trim()) {
          emit('search', value)
        }
      }, props.debounceDelay)
    }

    // 处理搜索
    const handleSearch = () => {
      const query = searchQuery.value.trim()
      if (query) {
        addToHistory(query)
        emit('search', query)
      }
    }

    // 处理焦点
    const handleFocus = () => {
      focused.value = true
      emit('focus')
    }

    const handleBlur = () => {
      // 延迟失焦，允许点击建议项
      setTimeout(() => {
        focused.value = false
        emit('blur')
      }, 200)
    }

    // 处理键盘事件
    const handleKeydown = (event) => {
      if (!props.showSuggestions || props.suggestions.length === 0) {
        if (event.key === 'Enter') {
          handleSearch()
        }
        return
      }

      switch (event.key) {
        case 'ArrowDown':
          event.preventDefault()
          highlightedIndex.value = Math.min(
            highlightedIndex.value + 1,
            props.suggestions.length - 1
          )
          break
        case 'ArrowUp':
          event.preventDefault()
          highlightedIndex.value = Math.max(highlightedIndex.value - 1, -1)
          break
        case 'Enter':
          event.preventDefault()
          if (highlightedIndex.value >= 0) {
            selectSuggestion(props.suggestions[highlightedIndex.value])
          } else {
            handleSearch()
          }
          break
        case 'Escape':
          inputRef.value?.blur()
          break
      }
    }

    // 选择建议
    const selectSuggestion = (suggestion) => {
      const value = suggestion.title || suggestion
      searchQuery.value = value
      emit('update:modelValue', value)
      addToHistory(value)
      emit('search', value)
      inputRef.value?.blur()
    }

    // 选择历史记录
    const selectHistory = (item) => {
      searchQuery.value = item
      emit('update:modelValue', item)
      emit('search', item)
    }

    // 清除搜索
    const clearSearch = () => {
      searchQuery.value = ''
      emit('update:modelValue', '')
      emit('clear')
      inputRef.value?.focus()
    }

    // 清除历史记录
    const clearHistory = () => {
      searchHistory.value = []
      saveSearchHistory()
    }

    // 移除单个历史记录
    const removeHistoryItem = (index) => {
      searchHistory.value.splice(index, 1)
      saveSearchHistory()
    }

    // 监听 modelValue 变化
    watch(() => props.modelValue, (newValue) => {
      searchQuery.value = newValue
    })

    // 监听建议变化，重置高亮索引
    watch(() => props.suggestions, () => {
      highlightedIndex.value = -1
    })

    // 组件挂载时加载搜索历史
    loadSearchHistory()

    return {
      inputRef,
      focused,
      searchQuery,
      highlightedIndex,
      searchHistory,
      handleInput,
      handleSearch,
      handleFocus,
      handleBlur,
      handleKeydown,
      selectSuggestion,
      selectHistory,
      clearSearch,
      clearHistory,
      removeHistoryItem
    }
  }
}
</script>

<style scoped>
.search-bar {
  position: relative;
  width: 100%;
  max-width: 500px;
}

.search-input-container {
  display: flex;
  align-items: center;
  background: white;
  border: 2px solid #e9ecef;
  border-radius: 8px;
  transition: all 0.3s ease;
  overflow: hidden;
}

.search-bar-focused .search-input-container {
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
}

.search-icon {
  padding: 0 0.75rem;
  color: #6c757d;
  font-size: 1rem;
}

.search-input {
  flex: 1;
  border: none;
  outline: none;
  padding: 0.75rem 0;
  font-size: 1rem;
  background: transparent;
}

.search-input::placeholder {
  color: #adb5bd;
}

.search-input:disabled {
  color: #6c757d;
  cursor: not-allowed;
}

.search-clear,
.search-button {
  background: none;
  border: none;
  padding: 0.5rem 0.75rem;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.9rem;
}

.search-clear {
  color: #6c757d;
}

.search-clear:hover {
  color: #495057;
  background-color: #f8f9fa;
}

.search-button {
  background-color: #007bff;
  color: white;
  border-radius: 0 6px 6px 0;
  margin: -2px -2px -2px 0;
  padding: 0.75rem 1rem;
}

.search-button:hover:not(:disabled) {
  background-color: #0056b3;
}

.search-button:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
}

.search-loading {
  display: inline-block;
  animation: spin 1s linear infinite;
}

.search-suggestions,
.search-history {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: white;
  border: 1px solid #e9ecef;
  border-top: none;
  border-radius: 0 0 8px 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  max-height: 300px;
  overflow-y: auto;
}

.suggestion-item,
.history-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
  border-bottom: 1px solid #f8f9fa;
}

.suggestion-item:hover,
.suggestion-highlighted,
.history-item:hover {
  background-color: #f8f9fa;
}

.suggestion-icon,
.history-icon {
  font-size: 1rem;
  flex-shrink: 0;
}

.suggestion-content {
  flex: 1;
  min-width: 0;
}

.suggestion-title {
  font-weight: 500;
  color: #2c3e50;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.suggestion-subtitle {
  font-size: 0.85rem;
  color: #6c757d;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0.75rem;
  background-color: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
  font-size: 0.85rem;
  color: #6c757d;
}

.history-clear {
  background: none;
  border: none;
  color: #007bff;
  cursor: pointer;
  font-size: 0.8rem;
}

.history-clear:hover {
  text-decoration: underline;
}

.history-items {
  max-height: 200px;
  overflow-y: auto;
}

.history-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.history-remove {
  background: none;
  border: none;
  color: #6c757d;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
  font-size: 0.8rem;
  opacity: 0;
  transition: all 0.3s ease;
}

.history-item:hover .history-remove {
  opacity: 1;
}

.history-remove:hover {
  background-color: #e9ecef;
  color: #495057;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* 暗色主题支持 */
.theme-dark .search-input-container {
  background: #2d3748;
  border-color: #4a5568;
}

.theme-dark .search-bar-focused .search-input-container {
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.25);
}

.theme-dark .search-input {
  color: #e2e8f0;
}

.theme-dark .search-input::placeholder {
  color: #a0aec0;
}

.theme-dark .search-icon {
  color: #a0aec0;
}

.theme-dark .search-suggestions,
.theme-dark .search-history {
  background: #2d3748;
  border-color: #4a5568;
}

.theme-dark .suggestion-item:hover,
.theme-dark .suggestion-highlighted,
.theme-dark .history-item:hover {
  background-color: #4a5568;
}

.theme-dark .suggestion-title {
  color: #e2e8f0;
}

.theme-dark .suggestion-subtitle {
  color: #a0aec0;
}

.theme-dark .history-header {
  background-color: #4a5568;
  border-color: #718096;
  color: #a0aec0;
}
</style>