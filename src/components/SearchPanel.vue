<template>
  <BaseCard class="search-panel">
    <template #header>
      <h3>全文搜索</h3>
      <BaseButton @click="$emit('close')" variant="ghost" size="small" class="close-btn">
        ×
      </BaseButton>
    </template>
    
    <div class="search-content">
      <!-- 搜索输入 -->
      <div class="search-input-section">
        <div class="search-input-wrapper">
          <BaseInput
            ref="searchInput"
            v-model="searchQuery" 
            @keyup.enter="performSearch"
            @input="onSearchInput"
            placeholder="输入要搜索的内容..."
            type="search"
          />
          <BaseButton 
            @click="performSearch" 
            :disabled="!searchQuery.trim()"
            variant="primary"
            size="small"
            icon="🔍"
          />
        </div>
        
        <!-- 搜索选项 -->
        <div class="search-options">
          <BaseSwitch
            v-model="searchOptions.caseSensitive"
            @change="performSearch"
            label="区分大小写"
            size="small"
          />
          <BaseSwitch
            v-model="searchOptions.wholeWord"
            @change="performSearch"
            label="全词匹配"
            size="small"
          />
        </div>
      </div>
      
      <!-- 搜索结果 -->
      <div class="search-results">
        <div v-if="searching" class="search-status">
          <div class="loading-spinner"></div>
          <span>搜索中...</span>
        </div>
        
        <div v-else-if="searchQuery && searchResults.length === 0" class="no-results">
          <div class="no-results-icon">🔍</div>
          <p>未找到相关内容</p>
          <p class="no-results-hint">尝试使用不同的关键词</p>
        </div>
        
        <div v-else-if="searchResults.length > 0" class="results-list">
          <div class="results-header">
            <span class="results-count">找到 {{ searchResults.length }} 个结果</span>
            <div class="navigation-controls">
              <BaseButton 
                @click="goToPreviousResult" 
                :disabled="currentResultIndex <= 0"
                variant="outline"
                size="small"
                title="上一个"
              >
                ↑
              </BaseButton>
              <span class="current-result">
                {{ currentResultIndex + 1 }} / {{ searchResults.length }}
              </span>
              <BaseButton 
                @click="goToNextResult" 
                :disabled="currentResultIndex >= searchResults.length - 1"
                variant="outline"
                size="small"
                title="下一个"
              >
                ↓
              </BaseButton>
            </div>
          </div>
          
          <div class="results-items">
            <div 
              v-for="(result, index) in searchResults" 
              :key="index"
              @click="goToResult(index)"
              :class="['result-item', { active: index === currentResultIndex }]"
            >
              <div class="result-position">
                {{ formatPosition(result.position) }}
              </div>
              <div class="result-content" v-html="result.highlightedText"></div>
            </div>
          </div>
        </div>
        
        <div v-else class="search-placeholder">
          <div class="placeholder-icon">📚</div>
          <p>输入关键词开始搜索</p>
          <p class="placeholder-hint">支持中英文搜索</p>
        </div>
      </div>
    </div>
  </BaseCard>
</template>

<script>
import { ref, computed, onMounted, nextTick, watch } from 'vue'

export default {
  name: 'SearchPanel',
  // 基础组件已全局注册，无需导入
  props: {
    content: {
      type: String,
      default: ''
    },
    currentPosition: {
      type: Number,
      default: 0
    }
  },
  emits: ['close', 'go-to-position', 'highlight-text'],
  setup(props, { emit }) {
    const searchInput = ref(null)
    const searchQuery = ref('')
    const searching = ref(false)
    const searchResults = ref([])
    const currentResultIndex = ref(-1)
    
    const searchOptions = ref({
      caseSensitive: false,
      wholeWord: false
    })
    
    let searchTimeout = null
    
    const performSearch = async () => {
      if (!searchQuery.value.trim()) {
        searchResults.value = []
        currentResultIndex.value = -1
        return
      }
      
      searching.value = true
      
      try {
        // 模拟搜索延迟
        await new Promise(resolve => setTimeout(resolve, 300))
        
        const results = searchInContent(searchQuery.value.trim())
        searchResults.value = results
        currentResultIndex.value = results.length > 0 ? 0 : -1
        
        // 高亮显示搜索结果
        if (results.length > 0) {
          emit('highlight-text', searchQuery.value.trim(), searchOptions.value)
        }
      } catch (error) {
        console.error('搜索失败:', error)
      } finally {
        searching.value = false
      }
    }
    
    const searchInContent = (query) => {
      const results = []
      const content = props.content
      
      if (!content || !query) return results
      
      let searchText = query
      let targetContent = content
      
      // 处理搜索选项
      if (!searchOptions.value.caseSensitive) {
        searchText = searchText.toLowerCase()
        targetContent = targetContent.toLowerCase()
      }
      
      // 构建搜索正则表达式
      let pattern
      if (searchOptions.value.wholeWord) {
        pattern = new RegExp(`\\b${escapeRegExp(searchText)}\\b`, searchOptions.value.caseSensitive ? 'g' : 'gi')
      } else {
        pattern = new RegExp(escapeRegExp(searchText), searchOptions.value.caseSensitive ? 'g' : 'gi')
      }
      
      let match
      while ((match = pattern.exec(content)) !== null) {
        const position = match.index / content.length
        const contextStart = Math.max(0, match.index - 50)
        const contextEnd = Math.min(content.length, match.index + match[0].length + 50)
        
        let contextText = content.substring(contextStart, contextEnd)
        
        // 高亮匹配的文本
        const highlightedText = contextText.replace(
          new RegExp(escapeRegExp(match[0]), searchOptions.value.caseSensitive ? 'g' : 'gi'),
          `<mark class="search-highlight">$&</mark>`
        )
        
        results.push({
          position,
          index: match.index,
          text: match[0],
          context: contextText,
          highlightedText: highlightedText,
          contextStart,
          contextEnd
        })
        
        // 防止无限循环
        if (match.index === pattern.lastIndex) {
          pattern.lastIndex++
        }
      }
      
      return results
    }
    
    const escapeRegExp = (string) => {
      return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    }
    
    const onSearchInput = () => {
      clearTimeout(searchTimeout)
      searchTimeout = setTimeout(() => {
        if (searchQuery.value.trim()) {
          performSearch()
        } else {
          searchResults.value = []
          currentResultIndex.value = -1
          emit('highlight-text', '', {}) // 清除高亮
        }
      }, 500)
    }
    
    const goToResult = (index) => {
      if (index >= 0 && index < searchResults.value.length) {
        currentResultIndex.value = index
        const result = searchResults.value[index]
        emit('go-to-position', result.position)
      }
    }
    
    const goToNextResult = () => {
      if (currentResultIndex.value < searchResults.value.length - 1) {
        goToResult(currentResultIndex.value + 1)
      }
    }
    
    const goToPreviousResult = () => {
      if (currentResultIndex.value > 0) {
        goToResult(currentResultIndex.value - 1)
      }
    }
    
    const formatPosition = (position) => {
      const percentage = Math.round(position * 100)
      return `${percentage}%`
    }
    
    // 监听搜索选项变化
    watch(() => searchOptions.value, () => {
      if (searchQuery.value.trim()) {
        performSearch()
      }
    }, { deep: true })
    
    onMounted(() => {
      nextTick(() => {
        searchInput.value?.focus()
      })
    })
    
    return {
      searchInput,
      searchQuery,
      searching,
      searchResults,
      currentResultIndex,
      searchOptions,
      performSearch,
      onSearchInput,
      goToResult,
      goToNextResult,
      goToPreviousResult,
      formatPosition
    }
  }
}
</script>

<style scoped>
.search-panel {
  width: 320px;
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 50;
  height: 100%;
}

.close-btn {
  font-size: 1.5rem;
}

.search-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 搜索输入区域 */
.search-input-section {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.search-input-wrapper {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.search-options {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* 搜索结果区域 */
.search-results {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.search-status {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  justify-content: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border-color);
  border-top: 2px solid var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.no-results,
.search-placeholder {
  text-align: center;
  padding: 2rem 1rem;
  color: var(--text-secondary);
}

.no-results-icon,
.placeholder-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.no-results p,
.search-placeholder p {
  margin: 0.5rem 0;
}

.no-results-hint,
.placeholder-hint {
  font-size: 0.8rem;
  color: var(--text-muted);
}

/* 结果列表 */
.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--border-color);
}

.results-count {
  font-size: 0.9rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.navigation-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* 导航按钮样式已由BaseButton组件提供 */

.current-result {
  font-size: 0.8rem;
  color: var(--text-secondary);
  min-width: 40px;
  text-align: center;
}

.results-items {
  max-height: none;
}

.result-item {
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin-bottom: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  background: var(--bg-secondary);
}

.result-item:hover {
  border-color: var(--accent-color);
  transform: translateY(-1px);
}

.result-item.active {
  border-color: var(--accent-color);
  background-color: var(--primary-color-light);
}

.result-position {
  font-size: 0.8rem;
  color: var(--accent-color);
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.result-content {
  font-size: 0.9rem;
  line-height: 1.4;
  color: var(--text-primary);
}

:deep(.search-highlight) {
  background-color: var(--warning-color);
  color: #000;
  padding: 0.1em 0.2em;
  border-radius: 2px;
  font-weight: 600;
}

/* 响应式设计 */
@media (max-width: 640px) {
  .search-panel {
    width: 100vw;
  }
  
  .navigation-controls {
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .current-result {
    order: -1;
  }
}
</style>