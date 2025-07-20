<template>
  <div class="downloads-view">
    <div class="page-header">
      <h1>在线下载</h1>
      <p class="page-description">搜索并下载在线小说资源</p>
    </div>
    
    <div class="downloads-content">
      <!-- 在线搜索组件 -->
      <OnlineSearch
        v-if="!selectedBook"
        @book-selected="handleBookSelected"
      />
      
      <!-- 下载管理组件 -->
      <DownloadManager
        v-else
        :selected-book="selectedBook"
        @back="handleBack"
        @book-downloaded="handleBookDownloaded"
      />
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import OnlineSearch from '../components/OnlineSearch.vue'
import DownloadManager from '../components/DownloadManager.vue'
import { useToast } from '../composables/useToast'

export default {
  name: 'DownloadsView',
  components: {
    OnlineSearch,
    DownloadManager
  },
  setup() {
    const { showToast } = useToast()
    const selectedBook = ref(null)
    
    const handleBookSelected = (book) => {
      selectedBook.value = book
    }
    
    const handleBack = () => {
      selectedBook.value = null
    }
    
    const handleBookDownloaded = () => {
      showToast('下载任务已添加到队列', 'success')
    }
    
    return {
      selectedBook,
      handleBookSelected,
      handleBack,
      handleBookDownloaded
    }
  }
}
</script>

<style scoped>
.downloads-view {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  color: var(--text-primary);
  font-size: 1.8rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.page-description {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.downloads-content {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 3rem;
  border: 1px solid var(--border-color);
}

.empty-state {
  text-align: center;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-state h3 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1.2rem;
  font-weight: 500;
}

.empty-state p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
}

.note {
  font-size: 0.9rem;
  color: var(--text-muted);
  font-style: italic;
}
</style>