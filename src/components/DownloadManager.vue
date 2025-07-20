<template>
  <div class="download-manager">
    <!-- 章节选择区域 -->
    <div v-if="selectedBook && !showDownloadQueue" class="chapter-selection">
      <div class="selection-header">
        <button @click="goBack" class="back-btn">
          ← 返回搜索
        </button>
        <div class="book-info">
          <h3>{{ selectedBook.title }}</h3>
          <p>{{ selectedBook.author }} · {{ selectedBook.sourceName }}</p>
        </div>
      </div>
      
      <div class="chapter-controls">
        <div class="selection-actions">
          <button @click="selectAllChapters" class="action-btn">全选</button>
          <button @click="selectNoneChapters" class="action-btn">清空</button>
          <button @click="selectRangeChapters" class="action-btn">范围选择</button>
        </div>
        
        <div class="download-info">
          <span class="selected-count">
            已选择 {{ selectedChapters.length }} / {{ chapters.length }} 章节
          </span>
          <button
            @click="startDownload"
            :disabled="selectedChapters.length === 0 || downloading"
            class="download-start-btn"
          >
            {{ downloading ? '下载中...' : '开始下载' }}
          </button>
        </div>
      </div>
      
      <!-- 章节列表 -->
      <div class="chapters-container">
        <div v-if="loadingChapters" class="loading-chapters">
          <Loading />
          <p>正在加载章节列表...</p>
        </div>
        
        <div v-else-if="chapters.length === 0" class="no-chapters">
          <div class="no-chapters-icon">📚</div>
          <h4>无法获取章节列表</h4>
          <p>请检查网络连接或尝试其他书源</p>
          <button @click="retryLoadChapters" class="retry-btn">重试</button>
        </div>
        
        <div v-else class="chapters-list">
          <div class="chapters-header">
            <span class="chapter-index">#</span>
            <span class="chapter-title">章节标题</span>
            <span class="chapter-actions">操作</span>
          </div>
          
          <div class="chapters-body">
            <label
              v-for="(chapter, index) in chapters"
              :key="index"
              class="chapter-item"
              :class="{ 'chapter-selected': selectedChapters.includes(index) }"
            >
              <input
                type="checkbox"
                :value="index"
                v-model="selectedChapters"
                class="chapter-checkbox"
              />
              <span class="chapter-number">{{ index + 1 }}</span>
              <span class="chapter-name">{{ chapter.title }}</span>
              <button
                @click.prevent="previewChapter(chapter)"
                class="preview-btn"
              >
                预览
              </button>
            </label>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 下载队列区域 -->
    <div v-else class="download-queue">
      <div class="queue-header">
        <h3>下载管理</h3>
        <div class="queue-actions">
          <button
            v-if="!showDownloadQueue"
            @click="showDownloadQueue = true"
            class="queue-toggle-btn"
          >
            查看下载队列 ({{ downloadQueue.length }})
          </button>
          <div class="batch-actions">
            <button 
              @click="pauseAllDownloads" 
              :disabled="activeDownloads.length === 0"
              class="batch-btn"
            >
              暂停全部
            </button>
            <button 
              @click="resumeAllDownloads" 
              :disabled="!hasPausedDownloads"
              class="batch-btn"
            >
              恢复全部
            </button>
            <button @click="clearCompletedDownloads" class="clear-btn">
              清除已完成
            </button>
          </div>
        </div>
      </div>
      
      <!-- 下载统计 -->
      <div class="download-stats">
        <div class="stat-item">
          <span class="stat-label">总任务</span>
          <span class="stat-value">{{ downloadQueue.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">进行中</span>
          <span class="stat-value">{{ activeDownloads.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">已完成</span>
          <span class="stat-value">{{ completedDownloads.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">失败</span>
          <span class="stat-value">{{ failedDownloads.length }}</span>
        </div>
      </div>
      
      <!-- 下载列表 -->
      <div class="downloads-list">
        <div
          v-for="download in downloadQueue"
          :key="download.id"
          class="download-item"
          :class="`download-${download.status}`"
        >
          <div class="download-info">
            <div class="download-title">{{ download.bookInfo.title }}</div>
            <div class="download-meta">
              <span class="download-author">{{ download.bookInfo.author }}</span>
              <span class="download-source">{{ download.bookInfo.sourceName }}</span>
              <span class="download-chapters">
                {{ download.downloadedChapters }} / {{ download.totalChapters }} 章节
              </span>
            </div>
          </div>
          
          <div class="download-progress">
            <div class="progress-info">
              <div class="progress-bar">
                <div
                  class="progress-fill"
                  :style="{ width: `${download.progress}%` }"
                ></div>
              </div>
              <span class="progress-text">{{ Math.round(download.progress) }}%</span>
            </div>
            <div class="progress-details">
              <span class="download-speed" v-if="download.status === 'downloading'">
                {{ getDownloadSpeed(download) }}
              </span>
              <span class="time-remaining" v-if="download.status === 'downloading'">
                剩余: {{ getTimeRemaining(download) }}
              </span>
              <span class="download-time" v-if="download.status === 'completed'">
                用时: {{ getDownloadDuration(download) }}
              </span>
            </div>
          </div>
          
          <div class="download-status">
            <span class="status-text">{{ getStatusText(download.status) }}</span>
            <div class="download-actions">
              <button
                v-if="download.status === 'downloading'"
                @click="pauseDownload(download.id)"
                class="action-btn pause-btn"
              >
                暂停
              </button>
              <button
                v-if="download.status === 'paused'"
                @click="resumeDownload(download.id)"
                class="action-btn resume-btn"
              >
                恢复
              </button>
              <button
                v-if="download.status === 'downloading' || download.status === 'paused'"
                @click="cancelDownload(download.id)"
                class="action-btn cancel-btn"
              >
                取消
              </button>
              <button
                v-if="download.status === 'failed'"
                @click="retryDownload(download.id)"
                class="action-btn retry-btn"
              >
                重试
              </button>
              <button
                v-if="download.status === 'completed'"
                @click="openBook(download)"
                class="action-btn open-btn"
              >
                打开
              </button>
            </div>
          </div>
        </div>
        
        <div v-if="downloadQueue.length === 0" class="empty-queue">
          <div class="empty-icon">📥</div>
          <h4>暂无下载任务</h4>
          <p>搜索并选择小说开始下载</p>
        </div>
      </div>
    </div>
    
    <!-- 章节预览模态框 -->
    <Modal
      v-if="showPreview"
      @close="closePreview"
      title="章节预览"
      size="large"
    >
      <div class="chapter-preview">
        <div v-if="loadingPreview" class="preview-loading">
          <Loading />
          <p>正在加载章节内容...</p>
        </div>
        
        <div v-else-if="previewContent" class="preview-content">
          <h4 class="preview-title">{{ previewContent.title }}</h4>
          <div class="preview-text">{{ previewContent.content }}</div>
        </div>
        
        <div v-else class="preview-error">
          <p>无法加载章节内容</p>
          <button @click="retryPreview" class="retry-btn">重试</button>
        </div>
      </div>
    </Modal>
    
    <!-- 范围选择模态框 -->
    <Modal
      v-if="showRangeSelector"
      @close="closeRangeSelector"
      title="范围选择"
    >
      <div class="range-selector">
        <div class="range-inputs">
          <div class="input-group">
            <label>起始章节</label>
            <input
              v-model.number="rangeStart"
              type="number"
              :min="1"
              :max="chapters.length"
              class="range-input"
            />
          </div>
          <div class="input-group">
            <label>结束章节</label>
            <input
              v-model.number="rangeEnd"
              type="number"
              :min="rangeStart"
              :max="chapters.length"
              class="range-input"
            />
          </div>
        </div>
        
        <div class="range-preview">
          将选择第 {{ rangeStart }} 到第 {{ rangeEnd }} 章，共 {{ rangeEnd - rangeStart + 1 }} 章节
        </div>
        
        <div class="range-actions">
          <button @click="closeRangeSelector" class="cancel-btn">取消</button>
          <button @click="applyRangeSelection" class="confirm-btn">确认选择</button>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script>
import { ref, computed, watch, onMounted } from 'vue'
import { usePluginStore } from '../stores/pluginStore'
import { useRouter } from 'vue-router'
import Loading from './Loading.vue'
import Modal from './Modal.vue'

export default {
  name: 'DownloadManager',
  components: {
    Loading,
    Modal
  },
  props: {
    selectedBook: {
      type: Object,
      default: null
    }
  },
  emits: ['back', 'book-downloaded'],
  setup(props, { emit }) {
    const pluginStore = usePluginStore()
    const router = useRouter()
    
    // 响应式数据
    const chapters = ref([])
    const selectedChapters = ref([])
    const loadingChapters = ref(false)
    const downloading = ref(false)
    const showDownloadQueue = ref(false)
    const showPreview = ref(false)
    const showRangeSelector = ref(false)
    const loadingPreview = ref(false)
    const previewContent = ref(null)
    const currentPreviewChapter = ref(null)
    const rangeStart = ref(1)
    const rangeEnd = ref(1)
    
    // 计算属性
    const downloadQueue = computed(() => pluginStore.downloadQueue)
    const activeDownloads = computed(() => pluginStore.activeDownloads)
    const completedDownloads = computed(() => pluginStore.completedDownloads)
    const failedDownloads = computed(() => pluginStore.failedDownloads)
    const hasPausedDownloads = computed(() => 
      downloadQueue.value.some(d => d.status === 'paused')
    )
    
    // 方法
    const loadChapters = async () => {
      if (!props.selectedBook) return
      
      loadingChapters.value = true
      try {
        const chapterList = await pluginStore.getOnlineChapters(
          props.selectedBook.sourceId,
          props.selectedBook.bookUrl
        )
        chapters.value = chapterList
        rangeEnd.value = chapterList.length
      } catch (error) {
        console.error('加载章节失败:', error)
        chapters.value = []
      } finally {
        loadingChapters.value = false
      }
    }
    
    const retryLoadChapters = () => {
      loadChapters()
    }
    
    const selectAllChapters = () => {
      selectedChapters.value = chapters.value.map((_, index) => index)
    }
    
    const selectNoneChapters = () => {
      selectedChapters.value = []
    }
    
    const selectRangeChapters = () => {
      showRangeSelector.value = true
    }
    
    const applyRangeSelection = () => {
      selectedChapters.value = []
      for (let i = rangeStart.value - 1; i < rangeEnd.value; i++) {
        selectedChapters.value.push(i)
      }
      closeRangeSelector()
    }
    
    const closeRangeSelector = () => {
      showRangeSelector.value = false
    }
    
    const startDownload = async () => {
      if (selectedChapters.value.length === 0) return
      
      downloading.value = true
      try {
        const selectedChapterList = selectedChapters.value.map(index => chapters.value[index])
        
        await pluginStore.downloadBook(
          props.selectedBook.sourceId,
          props.selectedBook.bookUrl,
          props.selectedBook,
          selectedChapterList
        )
        
        showDownloadQueue.value = true
        emit('book-downloaded')
      } catch (error) {
        console.error('下载失败:', error)
      } finally {
        downloading.value = false
      }
    }
    
    const previewChapter = async (chapter) => {
      currentPreviewChapter.value = chapter
      showPreview.value = true
      loadingPreview.value = true
      previewContent.value = null
      
      try {
        const content = await pluginStore.getChapterContent(
          props.selectedBook.sourceId,
          chapter.url
        )
        previewContent.value = content
      } catch (error) {
        console.error('预览章节失败:', error)
        previewContent.value = null
      } finally {
        loadingPreview.value = false
      }
    }
    
    const retryPreview = () => {
      if (currentPreviewChapter.value) {
        previewChapter(currentPreviewChapter.value)
      }
    }
    
    const closePreview = () => {
      showPreview.value = false
      previewContent.value = null
      currentPreviewChapter.value = null
    }
    
    const pauseDownload = (downloadId) => {
      pluginStore.pauseDownload(downloadId)
    }
    
    const resumeDownload = (downloadId) => {
      pluginStore.resumeDownload(downloadId)
    }
    
    const cancelDownload = (downloadId) => {
      pluginStore.cancelDownload(downloadId)
    }
    
    const retryDownload = (downloadId) => {
      pluginStore.retryDownload(downloadId)
    }
    
    const clearCompletedDownloads = () => {
      pluginStore.clearDownloadHistory()
    }
    
    const pauseAllDownloads = () => {
      activeDownloads.value.forEach(download => {
        pluginStore.pauseDownload(download.id)
      })
    }
    
    const resumeAllDownloads = () => {
      downloadQueue.value
        .filter(d => d.status === 'paused')
        .forEach(download => {
          pluginStore.resumeDownload(download.id)
        })
    }
    
    const openBook = async (download) => {
      try {
        if (download.bookId) {
          // 跳转到阅读器并打开指定书籍
          router.push(`/reader/${download.bookId}`)
        } else {
          // 如果没有bookId，跳转到图书库
          router.push('/library')
        }
      } catch (error) {
        console.error('打开书籍失败:', error)
        // 可以显示错误提示
      }
    }
    
    const goBack = () => {
      emit('back')
    }
    
    const getStatusText = (status) => {
      const statusMap = {
        pending: '等待中',
        downloading: '下载中',
        completed: '已完成',
        failed: '失败',
        cancelled: '已取消'
      }
      return statusMap[status] || status
    }
    
    const getDownloadSpeed = (download) => {
      if (!download.startTime || download.downloadedChapters === 0) return '计算中...'
      
      const elapsed = (Date.now() - new Date(download.startTime).getTime()) / 1000
      const speed = download.downloadedChapters / elapsed
      
      if (speed < 1) {
        return `${(speed * 60).toFixed(1)} 章/分钟`
      } else {
        return `${speed.toFixed(1)} 章/秒`
      }
    }
    
    const getTimeRemaining = (download) => {
      if (!download.startTime || download.downloadedChapters === 0) return '计算中...'
      
      const elapsed = (Date.now() - new Date(download.startTime).getTime()) / 1000
      const speed = download.downloadedChapters / elapsed
      const remaining = (download.totalChapters - download.downloadedChapters) / speed
      
      if (remaining < 60) {
        return `${Math.round(remaining)}秒`
      } else if (remaining < 3600) {
        return `${Math.round(remaining / 60)}分钟`
      } else {
        return `${Math.round(remaining / 3600)}小时`
      }
    }
    
    const getDownloadDuration = (download) => {
      if (!download.startTime || !download.endTime) return '未知'
      
      const duration = (new Date(download.endTime).getTime() - new Date(download.startTime).getTime()) / 1000
      
      if (duration < 60) {
        return `${Math.round(duration)}秒`
      } else if (duration < 3600) {
        return `${Math.round(duration / 60)}分钟`
      } else {
        return `${Math.round(duration / 3600)}小时`
      }
    }
    
    // 监听选中的书籍变化
    watch(() => props.selectedBook, (newBook) => {
      if (newBook) {
        showDownloadQueue.value = false
        selectedChapters.value = []
        loadChapters()
      }
    }, { immediate: true })
    
    // 监听下载完成
    watch(() => completedDownloads.value.length, (newCount, oldCount) => {
      if (newCount > oldCount) {
        // 有新的下载完成
        const newlyCompleted = completedDownloads.value.slice(oldCount)
        newlyCompleted.forEach(download => {
          console.log(`《${download.bookInfo.title}》下载完成`)
          // 这里可以显示通知
        })
      }
    })
    
    return {
      chapters,
      selectedChapters,
      loadingChapters,
      downloading,
      showDownloadQueue,
      showPreview,
      showRangeSelector,
      loadingPreview,
      previewContent,
      rangeStart,
      rangeEnd,
      downloadQueue,
      activeDownloads,
      completedDownloads,
      failedDownloads,
      hasPausedDownloads,
      loadChapters,
      retryLoadChapters,
      selectAllChapters,
      selectNoneChapters,
      selectRangeChapters,
      applyRangeSelection,
      closeRangeSelector,
      startDownload,
      previewChapter,
      retryPreview,
      closePreview,
      cancelDownload,
      pauseDownload,
      resumeDownload,
      retryDownload,
      clearCompletedDownloads,
      pauseAllDownloads,
      resumeAllDownloads,
      openBook,
      goBack,
      getStatusText,
      getDownloadSpeed,
      getTimeRemaining,
      getDownloadDuration
    }
  }
}
</script>

<style scoped>
.download-manager {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 1rem;
}

/* 章节选择区域 */
.chapter-selection {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 1rem;
}

.selection-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.back-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.back-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.book-info h3 {
  color: var(--text-primary);
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0 0 0.25rem 0;
}

.book-info p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin: 0;
}

.chapter-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.selection-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 0.5rem 0.75rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
}

.action-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.download-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.selected-count {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.download-start-btn {
  background: var(--primary-color);
  color: white;
  border: none;
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  transition: background-color 0.3s ease;
}

.download-start-btn:hover:not(:disabled) {
  background: var(--primary-color-dark);
}

.download-start-btn:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.chapters-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.loading-chapters,
.no-chapters {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  text-align: center;
}

.no-chapters-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.no-chapters h4 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.no-chapters p {
  color: var(--text-secondary);
  margin-bottom: 1rem;
}

.retry-btn {
  background: var(--primary-color);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.retry-btn:hover {
  background: var(--primary-color-dark);
}

.chapters-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.chapters-header {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  font-weight: 500;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.chapters-body {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem 0;
}

.chapter-item {
  display: grid;
  grid-template-columns: auto auto 1fr auto;
  gap: 1rem;
  align-items: center;
  padding: 0.75rem 1rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
  border-bottom: 1px solid var(--border-color-light);
}

.chapter-item:hover {
  background: var(--bg-secondary);
}

.chapter-selected {
  background: var(--primary-color-light);
}

.chapter-checkbox {
  margin: 0;
}

.chapter-number {
  color: var(--text-muted);
  font-size: 0.85rem;
  min-width: 3rem;
  text-align: center;
}

.chapter-name {
  color: var(--text-primary);
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.preview-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.3s ease;
}

.preview-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

/* 下载队列区域 */
.download-queue {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 1rem;
}

.queue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.queue-header h3 {
  color: var(--text-primary);
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0;
}

.queue-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.batch-actions {
  display: flex;
  gap: 0.5rem;
}

.batch-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  padding: 0.4rem 0.8rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
}

.batch-btn:hover:not(:disabled) {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.batch-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.queue-toggle-btn,
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

.queue-toggle-btn:hover,
.clear-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.download-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

.stat-value {
  color: var(--text-primary);
  font-size: 1.2rem;
  font-weight: 600;
}

.downloads-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.download-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  transition: all 0.3s ease;
}

.download-downloading {
  border-color: var(--primary-color);
  background: var(--primary-color-light);
}

.download-completed {
  border-color: var(--success-color);
}

.download-failed {
  border-color: var(--error-color);
}

.download-info {
  flex: 1;
  min-width: 0;
}

.download-title {
  color: var(--text-primary);
  font-weight: 500;
  margin-bottom: 0.25rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.download-meta {
  display: flex;
  gap: 1rem;
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.download-progress {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 200px;
}

.progress-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.progress-details {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary-color);
  transition: width 0.3s ease;
}

.progress-text {
  color: var(--text-secondary);
  font-size: 0.8rem;
  min-width: 3rem;
  text-align: right;
}

.download-speed,
.time-remaining,
.download-time {
  white-space: nowrap;
}

.download-status {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.5rem;
}

.status-text {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.download-actions {
  display: flex;
  gap: 0.5rem;
}

.pause-btn {
  background: var(--warning-color);
  color: white;
}

.resume-btn {
  background: var(--success-color);
  color: white;
}

.cancel-btn {
  background: var(--error-color);
  color: white;
}

.retry-btn {
  background: var(--warning-color);
  color: white;
}

.open-btn {
  background: var(--success-color);
  color: white;
}

.cancel-btn,
.retry-btn,
.open-btn {
  border: none;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: opacity 0.3s ease;
}

.pause-btn:hover,
.resume-btn:hover,
.cancel-btn:hover,
.retry-btn:hover,
.open-btn:hover {
  opacity: 0.8;
}

.empty-queue {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  text-align: center;
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-queue h4 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.empty-queue p {
  color: var(--text-secondary);
}

/* 模态框内容 */
.chapter-preview {
  max-height: 60vh;
  overflow-y: auto;
}

.preview-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2rem;
}

.preview-title {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 500;
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-color);
}

.preview-text {
  color: var(--text-primary);
  line-height: 1.6;
  white-space: pre-wrap;
}

.preview-error {
  text-align: center;
  padding: 2rem;
}

.range-selector {
  padding: 1rem;
}

.range-inputs {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.input-group label {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.range-input {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.range-preview {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 4px;
}

.range-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

.cancel-btn,
.confirm-btn {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.cancel-btn {
  background: none;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.cancel-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.confirm-btn {
  background: var(--primary-color);
  color: white;
  border: none;
}

.confirm-btn:hover {
  background: var(--primary-color-dark);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .chapter-controls {
    flex-direction: column;
    gap: 1rem;
  }
  
  .selection-actions {
    justify-content: center;
  }
  
  .download-info {
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }
  
  .chapters-header {
    grid-template-columns: auto 1fr;
    gap: 0.5rem;
  }
  
  .chapter-item {
    grid-template-columns: auto auto 1fr;
    gap: 0.5rem;
  }
  
  .preview-btn {
    display: none;
  }
  
  .download-item {
    flex-direction: column;
    align-items: stretch;
    gap: 0.75rem;
  }
  
  .download-progress {
    min-width: auto;
  }
  
  .download-status {
    align-items: stretch;
  }
  
  .download-actions {
    justify-content: center;
  }
  
  .range-inputs {
    grid-template-columns: 1fr;
  }
}
</style>