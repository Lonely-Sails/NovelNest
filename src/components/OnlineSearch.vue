<template>
  <div class="online-search">
    <!-- 搜索区域 -->
    <div class="search-section">
      <div class="search-header">
        <h2>在线搜索</h2>
        <p class="search-description">从多个书源搜索在线小说资源</p>
      </div>
      
      <div class="search-controls">
        <SearchBar
          v-model="searchQuery"
          placeholder="输入小说名称或作者..."
          :loading="pluginStore.searching"
          :show-search-button="true"
          :show-history="true"
          :suggestions="searchSuggestions"
          :show-suggestions="showSearchSuggestions"
          @search="handleSearch"
          @input="handleSearchInput"
          class="search-input"
        />
        
        <div class="source-filter">
          <button
            @click="showSourceSelector = !showSourceSelector"
            class="source-selector-btn"
            :class="{ active: showSourceSelector }"
          >
            <span class="source-icon">📚</span>
            <span>书源选择 ({{ selectedSourcesCount }})</span>
            <span class="dropdown-arrow">{{ showSourceSelector ? '▲' : '▼' }}</span>
          </button>
          
          <div v-if="showSourceSelector" class="source-selector">
            <div class="source-selector-header">
              <span>选择搜索书源</span>
              <div class="source-actions">
                <button @click="selectAllSources" class="action-btn">全选</button>
                <button @click="clearSelectedSources" class="action-btn">清空</button>
              </div>
            </div>
            
            <div class="source-list">
              <label
                v-for="source in enabledSources"
                :key="source.id"
                class="source-item"
              >
                <input
                  type="checkbox"
                  :value="source.id"
                  v-model="selectedSources"
                  class="source-checkbox"
                />
                <div class="source-info">
                  <span class="source-name">{{ source.name }}</span>
                  <span class="source-version">v{{ source.version }}</span>
                </div>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 搜索结果区域 -->
    <div class="results-section">
      <!-- 搜索状态 -->
      <div v-if="pluginStore.searching" class="search-status">
        <Loading />
        <p>正在搜索中...</p>
        <div class="search-progress">
          <p class="search-query">搜索关键词: "{{ searchQuery }}"</p>
          <p class="search-sources">正在搜索 {{ selectedSourcesCount }} 个书源</p>
        </div>
      </div>
      
      <!-- 搜索结果为空 -->
      <div v-else-if="searchQuery && searchResults.length === 0 && !pluginStore.searching" class="empty-results">
        <div class="empty-icon">🔍</div>
        <h3>未找到相关结果</h3>
        <p>尝试使用不同的关键词或检查书源设置</p>
      </div>
      
      <!-- 搜索结果列表 -->
      <div v-else-if="searchResults.length > 0" class="results-container">
        <div class="results-header">
          <h3>搜索结果 ({{ searchResults.length }})</h3>
          <div class="results-actions">
            <div class="results-filters">
              <select v-model="selectedSourceFilter" class="source-filter-select">
                <option value="">所有书源</option>
                <option v-for="source in availableSourcesInResults" :key="source.id" :value="source.id">
                  {{ source.name }} ({{ getResultCountBySource(source.id) }})
                </option>
              </select>
              <select v-model="sortBy" class="sort-select">
                <option value="relevance">相关度</option>
                <option value="source">书源</option>
                <option value="title">书名</option>
                <option value="author">作者</option>
              </select>
            </div>
            <button @click="clearResults" class="clear-btn">清空结果</button>
          </div>
        </div>
        
        <div class="results-grid">
          <div
            v-for="result in filteredAndSortedResults"
            :key="`${result.sourceId}_${result.bookUrl}`"
            class="result-card"
            @click="selectBook(result)"
          >
            <div class="result-cover">
              <img
                v-if="result.coverUrl"
                :src="result.coverUrl"
                :alt="result.title"
                @error="handleImageError"
                class="cover-image"
              />
              <div v-else class="cover-placeholder">
                📖
              </div>
            </div>
            
            <div class="result-info">
              <h4 class="result-title">{{ result.title }}</h4>
              <p class="result-author">{{ result.author }}</p>
              <p class="result-description">{{ truncateText(result.description, 100) }}</p>
              
              <div class="result-meta">
                <span class="result-source">{{ result.sourceName }}</span>
                <button
                  @click.stop="selectBook(result)"
                  class="download-btn"
                >
                  选择下载
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 初始状态 -->
      <div v-else class="initial-state">
        <div class="initial-icon">🌐</div>
        <h3>开始搜索在线小说</h3>
        <p>输入小说名称或作者名，从多个书源搜索资源</p>
        <div class="search-tips">
          <h4>搜索技巧：</h4>
          <ul>
            <li>使用准确的小说名称获得更好的结果</li>
            <li>可以只输入作者名搜索该作者的所有作品</li>
            <li>选择多个书源可以获得更全面的搜索结果</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue'
import { usePluginStore } from '../stores/pluginStore'
import SearchBar from './SearchBar.vue'
import Loading from './Loading.vue'

export default {
  name: 'OnlineSearch',
  components: {
    SearchBar,
    Loading
  },
  emits: ['book-selected'],
  setup(props, { emit }) {
    const pluginStore = usePluginStore()
    
    // 响应式数据
    const searchQuery = ref('')
    const selectedSources = ref([])
    const showSourceSelector = ref(false)
    const selectedSourceFilter = ref('')
    const sortBy = ref('relevance')
    const searchSuggestions = ref([])
    const showSearchSuggestions = ref(false)
    
    // 计算属性
    const enabledSources = computed(() => pluginStore.enabledSources)
    const searchResults = computed(() => pluginStore.searchResults)
    const selectedSourcesCount = computed(() => 
      selectedSources.value.length || enabledSources.value.length
    )
    
    const availableSourcesInResults = computed(() => {
      const sourceIds = [...new Set(searchResults.value.map(r => r.sourceId))]
      return sourceIds.map(id => enabledSources.value.find(s => s.id === id)).filter(Boolean)
    })
    
    const filteredAndSortedResults = computed(() => {
      let results = searchResults.value
      
      // 按书源过滤
      if (selectedSourceFilter.value) {
        results = results.filter(r => r.sourceId === selectedSourceFilter.value)
      }
      
      // 排序
      switch (sortBy.value) {
        case 'source':
          results = [...results].sort((a, b) => a.sourceName.localeCompare(b.sourceName))
          break
        case 'title':
          results = [...results].sort((a, b) => a.title.localeCompare(b.title))
          break
        case 'author':
          results = [...results].sort((a, b) => (a.author || '').localeCompare(b.author || ''))
          break
        default: // relevance
          // 保持原始顺序（通常已按相关度排序）
          break
      }
      
      return results
    })
    
    // 方法
    const handleSearch = async (query) => {
      if (!query.trim()) return
      
      searchQuery.value = query
      
      try {
        const sourcesToSearch = selectedSources.value.length > 0 
          ? selectedSources.value 
          : null
          
        await pluginStore.searchOnlineBooks(query, sourcesToSearch)
        
        // 搜索成功后的反馈
        if (searchResults.value.length === 0) {
          // 可以在这里添加Toast提示
          console.info('搜索完成，但未找到相关结果')
        } else {
          console.info(`搜索完成，找到 ${searchResults.value.length} 个结果`)
        }
      } catch (error) {
        console.error('搜索失败:', error)
        // 可以在这里添加错误Toast提示
        alert(`搜索失败: ${error.message}`)
      }
    }
    
    const selectAllSources = () => {
      selectedSources.value = enabledSources.value.map(s => s.id)
    }
    
    const clearSelectedSources = () => {
      selectedSources.value = []
    }
    
    const clearResults = () => {
      pluginStore.clearSearchResults()
      searchQuery.value = ''
      selectedSourceFilter.value = ''
      sortBy.value = 'relevance'
    }
    
    const getResultCountBySource = (sourceId) => {
      return searchResults.value.filter(r => r.sourceId === sourceId).length
    }
    
    const handleSearchInput = (query) => {
      if (query.length >= 2) {
        generateSearchSuggestions(query)
        showSearchSuggestions.value = true
      } else {
        showSearchSuggestions.value = false
      }
    }
    
    const generateSearchSuggestions = (query) => {
      // 基于搜索历史生成建议
      const history = JSON.parse(localStorage.getItem('searchHistory') || '[]')
      const suggestions = history
        .filter(item => item.toLowerCase().includes(query.toLowerCase()))
        .slice(0, 5)
        .map(item => ({
          title: item,
          icon: '🕒',
          subtitle: '搜索历史'
        }))
      
      // 添加一些常见的搜索建议
      const commonSuggestions = [
        { title: `${query} 小说`, icon: '📖', subtitle: '小说搜索' },
        { title: `${query} 作者`, icon: '✍️', subtitle: '作者搜索' }
      ]
      
      searchSuggestions.value = [...suggestions, ...commonSuggestions].slice(0, 8)
    }
    
    const selectBook = (result) => {
      emit('book-selected', result)
    }
    
    const handleImageError = (event) => {
      event.target.style.display = 'none'
    }
    
    const truncateText = (text, maxLength) => {
      if (!text) return ''
      return text.length > maxLength 
        ? text.substring(0, maxLength) + '...' 
        : text
    }
    
    // 生命周期
    onMounted(async () => {
      try {
        await pluginStore.loadBookSources()
        // 默认选择所有启用的书源
        selectedSources.value = enabledSources.value.map(s => s.id)
      } catch (error) {
        console.error('加载书源失败:', error)
      }
    })
    
    return {
      pluginStore,
      searchQuery,
      selectedSources,
      showSourceSelector,
      enabledSources,
      searchResults,
      selectedSourcesCount,
      selectedSourceFilter,
      sortBy,
      availableSourcesInResults,
      filteredAndSortedResults,
      handleSearch,
      selectAllSources,
      clearSelectedSources,
      clearResults,
      selectBook,
      handleImageError,
      truncateText,
      getResultCountBySource,
      searchSuggestions,
      showSearchSuggestions,
      handleSearchInput
    }
  }
}
</script>

<style scoped>
.online-search {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  height: 100%;
}

.search-section {
  flex-shrink: 0;
}

.search-header {
  margin-bottom: 1.5rem;
}

.search-header h2 {
  color: var(--text-primary);
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.search-description {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.search-controls {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.search-input {
  flex: 1;
}

.source-filter {
  position: relative;
  flex-shrink: 0;
}

.source-selector-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  background: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.9rem;
  color: var(--text-primary);
  white-space: nowrap;
}

.source-selector-btn:hover,
.source-selector-btn.active {
  border-color: var(--primary-color);
  background: var(--primary-color-light);
}

.source-icon {
  font-size: 1rem;
}

.dropdown-arrow {
  font-size: 0.8rem;
  transition: transform 0.3s ease;
}

.source-selector {
  position: absolute;
  top: 100%;
  right: 0;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  min-width: 280px;
  max-height: 300px;
  overflow: hidden;
}

.source-selector-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-primary);
}

.source-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  background: none;
  border: none;
  color: var(--primary-color);
  cursor: pointer;
  font-size: 0.8rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: background-color 0.3s ease;
}

.action-btn:hover {
  background: var(--primary-color-light);
}

.source-list {
  max-height: 200px;
  overflow-y: auto;
  padding: 0.5rem 0;
}

.source-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 1rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.source-item:hover {
  background: var(--bg-secondary);
}

.source-checkbox {
  margin: 0;
}

.source-info {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.source-name {
  color: var(--text-primary);
  font-size: 0.9rem;
}

.source-version {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

.results-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.search-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  color: var(--text-secondary);
}

.search-progress {
  margin-top: 1rem;
  text-align: center;
}

.search-query {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.search-sources {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.empty-results,
.initial-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  text-align: center;
}

.empty-icon,
.initial-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-results h3,
.initial-state h3 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1.2rem;
  font-weight: 500;
}

.empty-results p,
.initial-state p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 1rem;
}

.search-tips {
  background: var(--bg-secondary);
  padding: 1.5rem;
  border-radius: 8px;
  text-align: left;
  max-width: 400px;
}

.search-tips h4 {
  color: var(--text-primary);
  font-size: 1rem;
  margin-bottom: 0.75rem;
}

.search-tips ul {
  color: var(--text-secondary);
  font-size: 0.85rem;
  line-height: 1.5;
  padding-left: 1.2rem;
}

.search-tips li {
  margin-bottom: 0.5rem;
}

.results-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--border-color);
}

.results-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.results-filters {
  display: flex;
  gap: 0.5rem;
}

.source-filter-select,
.sort-select {
  padding: 0.4rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.85rem;
  cursor: pointer;
  transition: border-color 0.3s ease;
}

.source-filter-select:hover,
.sort-select:hover {
  border-color: var(--primary-color);
}

.source-filter-select:focus,
.sort-select:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px var(--primary-color-light);
}

.results-header h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 500;
}

.clear-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
}

.clear-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.result-card {
  display: flex;
  gap: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.result-card:hover {
  border-color: var(--primary-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.result-cover {
  flex-shrink: 0;
  width: 60px;
  height: 80px;
  border-radius: 4px;
  overflow: hidden;
  background: var(--bg-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.cover-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.cover-placeholder {
  font-size: 1.5rem;
  color: var(--text-muted);
}

.result-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 0;
}

.result-title {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 500;
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-author {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-description {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.4;
  margin: 0;
  flex: 1;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.result-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: auto;
}

.result-source {
  color: var(--text-muted);
  font-size: 0.75rem;
  background: var(--bg-secondary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
}

.download-btn {
  background: var(--primary-color);
  color: white;
  border: none;
  padding: 0.4rem 0.8rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: background-color 0.3s ease;
}

.download-btn:hover {
  background: var(--primary-color-dark);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .search-controls {
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .source-selector {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90vw;
    max-width: 400px;
  }
  
  .results-grid {
    grid-template-columns: 1fr;
  }
  
  .result-card {
    flex-direction: column;
    text-align: center;
  }
  
  .result-cover {
    align-self: center;
  }
}
</style>