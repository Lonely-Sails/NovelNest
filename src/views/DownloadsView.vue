<template>
  <div class="downloads-view">
    <div class="page-header">
      <div class="header-content">
        <h1>在线下载</h1>
        <p class="page-description">搜索并下载在线小说资源</p>
      </div>
      
      <div class="header-actions" v-if="selectedBook">
        <BaseButton @click="handleBack" variant="secondary" icon="←">
          返回搜索
        </BaseButton>
      </div>
    </div>
    
    <BaseCard class="downloads-content">
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
    </BaseCard>
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
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  gap: 2rem;
}

.header-content h1 {
  color: var(--text-primary);
  font-size: 1.8rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.page-description {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
  flex-shrink: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: 1rem;
  }
  
  .header-actions {
    width: 100%;
    justify-content: stretch;
  }
  
  .header-actions > * {
    flex: 1;
  }
}

.downloads-content {
  padding: 3rem;
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