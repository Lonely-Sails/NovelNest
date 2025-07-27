<template>
  <div class="library">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <h1>图书库</h1>
        <p class="page-description">管理您的电子书收藏</p>
      </div>
      
      <div class="header-actions">
        <BaseButton @click="showImporter" variant="primary" icon="📖">
          导入图书
        </BaseButton>
        <BaseButton @click="refreshLibrary" variant="outline" :loading="loading" icon="🔄">
          刷新
        </BaseButton>
      </div>
    </div>

    <!-- 统计信息 -->
    <div class="stats-section" v-if="stats.totalBooks > 0">
      <BaseCard class="stat-card" compact>
        <h3>{{ stats.totalBooks }}</h3>
        <p>总图书数</p>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <h3>{{ stats.readingBooks }}</h3>
        <p>正在阅读</p>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <h3>{{ stats.completedBooks }}</h3>
        <p>已完成</p>
      </BaseCard>
    </div>

    <!-- 搜索和过滤区域 -->
    <BaseCard class="controls-section">
      <BaseInput v-model="searchQuery" type="search" placeholder="搜索图书标题、作者..." :loading="searchLoading" clearable
        @clear="handleClearSearch" class="search-input" />

      <div class="filter-controls">
        <!-- 格式过滤 -->
        <div class="filter-group">
          <label class="filter-label">格式:</label>
          <select v-model="selectedFormat" @change="applyFilters" class="filter-select">
            <option value="">全部</option>
            <option value="txt">TXT</option>
            <option value="epub">EPUB</option>
            <option value="pdf">PDF</option>
          </select>
        </div>

        <!-- 阅读状态过滤 -->
        <div class="filter-group">
          <label class="filter-label">状态:</label>
          <select v-model="selectedStatus" @change="applyFilters" class="filter-select">
            <option value="">全部</option>
            <option value="unread">未读</option>
            <option value="reading">在读</option>
            <option value="completed">已读</option>
          </select>
        </div>

        <!-- 排序 -->
        <div class="filter-group">
          <label class="filter-label">排序:</label>
          <select v-model="sortBy" @change="applySorting" class="filter-select">
            <option value="title">标题</option>
            <option value="author">作者</option>
            <option value="created_at">添加时间</option>
            <option value="last_read">最近阅读</option>
            <option value="progress">阅读进度</option>
          </select>
        </div>

        <!-- 视图切换 -->
        <div class="view-toggle">
          <BaseButton @click="viewMode = 'grid'" :variant="viewMode === 'grid' ? 'primary' : 'outline'" size="small">
            网格
          </BaseButton>
          <BaseButton @click="viewMode = 'list'" :variant="viewMode === 'list' ? 'primary' : 'outline'" size="small">
            列表
          </BaseButton>
        </div>
      </div>
    </BaseCard>

    <!-- 图书列表区域 -->
    <div class="books-container">
      <!-- 加载状态 -->
      <Loading v-if="loading" message="加载图书中..." />

      <!-- 空状态 -->
      <BaseCard v-else-if="displayBooks.length === 0 && !searchQuery" class="empty-state">
        <div class="empty-icon">📚</div>
        <h3>还没有图书</h3>
        <p>点击上方按钮开始导入您的第一本图书</p>
        <template #actions>
          <BaseButton @click="showImporter" variant="primary" icon="📖">
            导入图书
          </BaseButton>
        </template>
      </BaseCard>

      <!-- 搜索无结果 -->
      <BaseCard v-else-if="displayBooks.length === 0 && searchQuery" class="empty-state">
        <div class="empty-icon">🔍</div>
        <h3>没有找到相关图书</h3>
        <p>尝试使用不同的关键词搜索</p>
        <template #actions>
          <BaseButton @click="handleClearSearch" variant="secondary">
            清除搜索
          </BaseButton>
        </template>
      </BaseCard>

      <!-- 图书网格/列表 -->
      <div v-else :class="['books-grid', `books-${viewMode}`]">
        <BookCard v-for="book in displayBooks" :key="book.id" :book="book" :compact="viewMode === 'list'"
          :actions="['read', 'edit', 'delete']" @click="openBook" @read="openBook" @edit="editBook"
          @delete="confirmDeleteBook" class="book-card-item" />
      </div>
    </div>

    <!-- 图书详情模态框 -->
    <Modal v-if="showBookDetail" @close="closeBookDetail" title="图书详情" size="large">
      <div class="book-detail-content">
        <div class="book-detail-header">
          <div class="book-cover-large">
            <div class="book-cover-placeholder">
              <span class="book-icon">📖</span>
            </div>
            <div class="book-format-badge">{{ selectedBook?.format?.toUpperCase() }}</div>
          </div>
          <div class="book-info-large">
            <h2 class="book-title-large">{{ selectedBook?.title }}</h2>
            <p class="book-author-large">{{ selectedBook?.author || '未知作者' }}</p>
            <div class="book-meta-large">
              <div class="meta-row">
                <span class="meta-label">文件路径:</span>
                <span class="meta-value" :title="selectedBook?.file_path">
                  {{ truncatePath(selectedBook?.file_path) }}
                </span>
              </div>
              <div class="meta-row">
                <span class="meta-label">文件大小:</span>
                <span class="meta-value">{{ formatFileSize(selectedBook?.file_size) }}</span>
              </div>
              <div class="meta-row">
                <span class="meta-label">文件格式:</span>
                <span class="meta-value">{{ selectedBook?.format?.toUpperCase() }}</span>
              </div>
              <div class="meta-row">
                <span class="meta-label">添加时间:</span>
                <span class="meta-value">{{ formatDate(selectedBook?.created_at) }}</span>
              </div>
              <div class="meta-row" v-if="selectedBook?.last_read">
                <span class="meta-label">最近阅读:</span>
                <span class="meta-value">{{ formatDate(selectedBook?.last_read) }}</span>
              </div>
              <div class="meta-row">
                <span class="meta-label">阅读进度:</span>
                <span class="meta-value">{{ Math.round((selectedBook?.reading_progress || 0) * 100) }}%</span>
              </div>
            </div>
            <div class="progress-section">
              <div class="progress-bar-large">
                <div class="progress-fill" :style="{ width: (selectedBook?.reading_progress || 0) * 100 + '%' }"></div>
              </div>
              <div class="progress-actions">
                <button @click="resetProgress" class="btn-link" v-if="selectedBook?.reading_progress > 0">
                  重置进度
                </button>
                <button @click="markAsCompleted" class="btn-link" v-if="selectedBook?.reading_progress < 1">
                  标记为已读
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 阅读历史 -->
        <div class="reading-history" v-if="readingHistory.length > 0">
          <h3>阅读历史</h3>
          <div class="history-list">
            <div v-for="record in readingHistory" :key="record.id" class="history-item">
              <div class="history-date">{{ formatDate(record.read_at) }}</div>
              <div class="history-duration" v-if="record.duration">
                阅读时长: {{ formatDuration(record.duration) }}
              </div>
            </div>
          </div>
        </div>

        <!-- 书签列表 -->
        <div class="bookmarks-section" v-if="bookmarks.length > 0">
          <h3>书签</h3>
          <div class="bookmarks-list">
            <div v-for="bookmark in bookmarks" :key="bookmark.id" class="bookmark-item">
              <div class="bookmark-info">
                <div class="bookmark-note">{{ bookmark.note || '无备注' }}</div>
                <div class="bookmark-meta">
                  <span>位置: {{ bookmark.position }}</span>
                  <span v-if="bookmark.chapter_index">章节: {{ bookmark.chapter_index + 1 }}</span>
                  <span>{{ formatDate(bookmark.created_at) }}</span>
                </div>
              </div>
              <button @click="deleteBookmark(bookmark.id)" class="bookmark-delete" title="删除书签">
                🗑️
              </button>
            </div>
          </div>
        </div>

        <div class="book-detail-actions">
          <BaseButton @click="openBook(selectedBook)" variant="primary" icon="📖">
            {{ selectedBook?.reading_progress > 0 ? '继续阅读' : '开始阅读' }}
          </BaseButton>
          <BaseButton @click="showEditModal" variant="secondary" icon="✏️">
            编辑信息
          </BaseButton>
          <BaseButton @click="confirmDeleteBook(selectedBook)" variant="danger" icon="🗑️">
            删除图书
          </BaseButton>
        </div>
      </div>
    </Modal>

    <!-- 图书编辑模态框 -->
    <Modal v-if="showEditBook" @close="closeEditModal" title="编辑图书信息" size="medium">
      <div class="book-edit-content">
        <form @submit.prevent="saveBookEdit" class="edit-form">
          <div class="form-group">
            <label for="edit-title" class="form-label">书名 *</label>
            <input id="edit-title" v-model="editForm.title" type="text" class="form-input" required
              placeholder="请输入书名" />
          </div>

          <div class="form-group">
            <label for="edit-author" class="form-label">作者</label>
            <input id="edit-author" v-model="editForm.author" type="text" class="form-input" placeholder="请输入作者名" />
          </div>

          <div class="form-group">
            <label for="edit-progress" class="form-label">阅读进度</label>
            <div class="progress-input-group">
              <input id="edit-progress" v-model.number="editForm.progress" type="range" min="0" max="100"
                class="progress-slider" />
              <span class="progress-value">{{ editForm.progress }}%</span>
            </div>
          </div>

          <div class="form-actions">
            <BaseButton type="button" @click="closeEditModal" variant="secondary">
              取消
            </BaseButton>
            <BaseButton type="submit" variant="primary" :loading="saving">
              保存
            </BaseButton>
          </div>
        </form>
      </div>
    </Modal>

    <!-- 删除确认对话框 -->
    <Modal v-if="showDeleteConfirm" @close="cancelDelete" title="确认删除" size="small">
      <div class="delete-confirm-content">
        <p>确定要删除《{{ bookToDelete?.title }}》吗？</p>
        <p class="delete-warning">此操作不可撤销，图书文件将被永久删除。</p>
        <div class="delete-actions">
          <BaseButton @click="cancelDelete" variant="secondary">取消</BaseButton>
          <BaseButton @click="executeDelete" variant="danger" :loading="deleting">
            确认删除
          </BaseButton>
        </div>
      </div>
    </Modal>

    <!-- 图书导入器 -->
    <Modal v-if="showBookImporter" @close="closeImporter" title="导入图书" size="large">
      <UnifiedBookImporter @import-complete="handleImportComplete" />
    </Modal>


  </div>
</template>

<script>
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useBookStore } from '@/stores/bookStore'
import { useToast } from '@/composables/useToast'
import { invoke } from '@tauri-apps/api/core'
import BookCard from '@/components/BookCard.vue'
import SearchBar from '@/components/SearchBar.vue'
import UnifiedBookImporter from '@/components/UnifiedBookImporter.vue'
// 全局注册的组件无需导入：Modal, Toast, Loading, BaseButton等

export default {
  name: 'LibraryView',
  components: {
    BookCard,
    SearchBar,
    UnifiedBookImporter
  },
  setup() {
    const router = useRouter()
    const bookStore = useBookStore()
    const { toastState, showSuccess, showError, showWarning, showInfo, hideToast } = useToast()

    // 响应式数据
    const searchQuery = ref('')
    const searchLoading = ref(false)
    const viewMode = ref('grid')
    const selectedFormat = ref('')
    const selectedStatus = ref('')
    const sortBy = ref('title')
    const sortOrder = ref('asc')

    // 模态框状态
    const showBookDetail = ref(false)
    const selectedBook = ref(null)
    const showDeleteConfirm = ref(false)
    const bookToDelete = ref(null)
    const deleting = ref(false)
    const showEditBook = ref(false)
    const saving = ref(false)
    const showBookImporter = ref(false)

    // 图书详情相关数据
    const readingHistory = ref([])
    const bookmarks = ref([])

    // 编辑表单
    const editForm = ref({
      title: '',
      author: '',
      progress: 0
    })



    // 计算属性
    const loading = computed(() => bookStore.loading)
    const books = computed(() => bookStore.books)
    const stats = computed(() => bookStore.stats)

    // 过滤和排序后的图书列表
    const displayBooks = computed(() => {
      let filtered = books.value

      // 应用搜索过滤
      if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase()
        filtered = filtered.filter(book =>
          book.title.toLowerCase().includes(query) ||
          (book.author && book.author.toLowerCase().includes(query))
        )
      }

      // 应用格式过滤
      if (selectedFormat.value) {
        filtered = filtered.filter(book =>
          book.format.toLowerCase() === selectedFormat.value.toLowerCase()
        )
      }

      // 应用状态过滤
      if (selectedStatus.value) {
        switch (selectedStatus.value) {
          case 'unread':
            filtered = filtered.filter(book => book.reading_progress === 0)
            break
          case 'reading':
            filtered = filtered.filter(book => book.reading_progress > 0 && book.reading_progress < 1)
            break
          case 'completed':
            filtered = filtered.filter(book => book.reading_progress >= 1)
            break
        }
      }

      // 应用排序
      filtered.sort((a, b) => {
        let aValue, bValue

        switch (sortBy.value) {
          case 'title':
            aValue = a.title.toLowerCase()
            bValue = b.title.toLowerCase()
            break
          case 'author':
            aValue = (a.author || '').toLowerCase()
            bValue = (b.author || '').toLowerCase()
            break
          case 'created_at':
            aValue = new Date(a.created_at)
            bValue = new Date(b.created_at)
            break
          case 'last_read':
            aValue = a.last_read ? new Date(a.last_read) : new Date(0)
            bValue = b.last_read ? new Date(b.last_read) : new Date(0)
            break
          case 'progress':
            aValue = a.reading_progress || 0
            bValue = b.reading_progress || 0
            break
          default:
            return 0
        }

        if (aValue < bValue) return sortOrder.value === 'asc' ? -1 : 1
        if (aValue > bValue) return sortOrder.value === 'asc' ? 1 : -1
        return 0
      })

      return filtered
    })

    // 方法

    const handleSearch = async (query) => {
      searchLoading.value = true
      try {
        await bookStore.searchBooks(query)
      } catch (error) {
        console.error('搜索失败:', error)
        showError('搜索失败，请重试')
      } finally {
        searchLoading.value = false
      }
    }

    const handleClearSearch = () => {
      searchQuery.value = ''
      bookStore.clearSearch()
    }

    const applyFilters = () => {
      // 过滤逻辑已在 displayBooks 计算属性中处理
    }

    const applySorting = () => {
      // 排序逻辑已在 displayBooks 计算属性中处理
    }

    const showImporter = () => {
      showBookImporter.value = true
    }

    const closeImporter = () => {
      showBookImporter.value = false
    }

    const handleImportComplete = async (result) => {
      if (result.success > 0) {
        await bookStore.loadBooks() // 刷新图书列表
        showSuccess(`成功导入 ${result.success} 本图书${result.error > 0 ? `，${result.error} 个失败` : ''}`)
      }
      closeImporter()
    }

    const refreshLibrary = async () => {
      try {
        await bookStore.loadBooks()
        showSuccess('图书库已刷新')
      } catch (error) {
        console.error('刷新图书库失败:', error)
        showError('刷新失败: ' + error.message)
      }
    }

    const openBook = (book) => {
      router.push(`/reader/${book.id}`)
    }

    const editBook = async (book) => {
      selectedBook.value = book
      showBookDetail.value = true

      // 加载图书详情数据
      await loadBookDetails(book.id)
    }

    const closeBookDetail = () => {
      showBookDetail.value = false
      selectedBook.value = null
      readingHistory.value = []
      bookmarks.value = []
    }

    const loadBookDetails = async (bookId) => {
      try {
        // 加载阅读历史（模拟数据，实际应该调用 Tauri 命令）
        readingHistory.value = [
          {
            id: 1,
            read_at: new Date(Date.now() - 86400000).toISOString(), // 1天前
            duration: 3600 // 1小时
          },
          {
            id: 2,
            read_at: new Date(Date.now() - 172800000).toISOString(), // 2天前
            duration: 2400 // 40分钟
          }
        ]

        // 加载书签
        bookmarks.value = await invoke('get_bookmarks', { bookId })
      } catch (error) {
        console.error('加载图书详情失败:', error)
        // 使用空数据作为后备
        readingHistory.value = []
        bookmarks.value = []
      }
    }

    const showEditModal = () => {
      if (!selectedBook.value) return

      editForm.value = {
        title: selectedBook.value.title,
        author: selectedBook.value.author || '',
        progress: Math.round((selectedBook.value.reading_progress || 0) * 100)
      }

      showEditBook.value = true
    }

    const closeEditModal = () => {
      showEditBook.value = false
      editForm.value = {
        title: '',
        author: '',
        progress: 0
      }
    }

    const saveBookEdit = async () => {
      if (!selectedBook.value || !editForm.value.title.trim()) return

      saving.value = true
      try {
        // 更新图书信息（这里需要实现相应的 Tauri 命令）
        const updatedBook = {
          ...selectedBook.value,
          title: editForm.value.title.trim(),
          author: editForm.value.author.trim() || null,
          reading_progress: editForm.value.progress / 100
        }

        // 更新阅读进度
        if (editForm.value.progress !== Math.round((selectedBook.value.reading_progress || 0) * 100)) {
          await bookStore.updateReadingProgress(selectedBook.value.id, editForm.value.progress / 100)
        }

        // 更新本地状态
        selectedBook.value = updatedBook
        const bookIndex = books.value.findIndex(b => b.id === selectedBook.value.id)
        if (bookIndex !== -1) {
          books.value[bookIndex] = updatedBook
        }

        showSuccess('图书信息更新成功')
        closeEditModal()
      } catch (error) {
        console.error('保存图书信息失败:', error)
        showError('保存图书信息失败: ' + error.message)
      } finally {
        saving.value = false
      }
    }

    const resetProgress = async () => {
      if (!selectedBook.value) return

      try {
        await bookStore.updateReadingProgress(selectedBook.value.id, 0)
        selectedBook.value.reading_progress = 0
        showSuccess('阅读进度已重置')
      } catch (error) {
        console.error('重置进度失败:', error)
        showError('重置进度失败: ' + error.message)
      }
    }

    const markAsCompleted = async () => {
      if (!selectedBook.value) return

      try {
        await bookStore.updateReadingProgress(selectedBook.value.id, 1)
        selectedBook.value.reading_progress = 1
        showSuccess('已标记为已读')
      } catch (error) {
        console.error('标记完成失败:', error)
        showError('标记完成失败: ' + error.message)
      }
    }

    const deleteBookmark = async (bookmarkId) => {
      try {
        await invoke('delete_bookmark', { bookmarkId })
        bookmarks.value = bookmarks.value.filter(b => b.id !== bookmarkId)
        showSuccess('书签删除成功')
      } catch (error) {
        console.error('删除书签失败:', error)
        showError('删除书签失败: ' + error.message)
      }
    }

    const confirmDeleteBook = (book) => {
      bookToDelete.value = book
      showDeleteConfirm.value = true
    }

    const cancelDelete = () => {
      showDeleteConfirm.value = false
      bookToDelete.value = null
    }

    const executeDelete = async () => {
      if (!bookToDelete.value) return

      deleting.value = true
      try {
        await bookStore.deleteBook(bookToDelete.value.id)
        showSuccess('图书删除成功')
        cancelDelete()
      } catch (error) {
        console.error('删除图书失败:', error)
        showError('删除图书失败: ' + error.message)
      } finally {
        deleting.value = false
      }
    }

    const formatFileSize = (bytes) => {
      if (!bytes || bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    }

    const formatDate = (dateString) => {
      if (!dateString) return ''
      const date = new Date(dateString)
      return date.toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    }

    const formatDuration = (seconds) => {
      if (!seconds) return '0分钟'

      const hours = Math.floor(seconds / 3600)
      const minutes = Math.floor((seconds % 3600) / 60)

      if (hours > 0) {
        return `${hours}小时${minutes}分钟`
      }
      return `${minutes}分钟`
    }

    const truncatePath = (path) => {
      if (!path) return ''
      if (path.length <= 50) return path

      const parts = path.split('/')
      if (parts.length <= 2) return path

      return `.../${parts.slice(-2).join('/')}`
    }

    // 监听搜索查询变化
    watch(searchQuery, (newQuery) => {
      if (newQuery) handleSearch(newQuery)
      else bookStore.clearSearch()
    }, { debounce: 300 })

    // 组件挂载时加载数据
    onMounted(async () => {
      try {
        await bookStore.loadBooks()
      } catch (error) {
        console.error('加载图书失败:', error)
        showError('加载图书失败，请刷新页面重试')
      }
    })

    return {
      // 数据
      searchQuery,
      searchLoading,
      viewMode,
      selectedFormat,
      selectedStatus,
      sortBy,
      sortOrder,
      showBookDetail,
      selectedBook,
      showDeleteConfirm,
      bookToDelete,
      deleting,
      showEditBook,
      saving,
      showBookImporter,
      readingHistory,
      bookmarks,
      editForm,

      // 计算属性
      loading,
      books,
      stats,
      displayBooks,

      // 方法
      handleSearch,
      handleClearSearch,
      applyFilters,
      applySorting,
      showImporter,
      closeImporter,
      handleImportComplete,
      refreshLibrary,
      openBook,
      editBook,
      closeBookDetail,
      loadBookDetails,
      showEditModal,
      closeEditModal,
      saveBookEdit,
      resetProgress,
      markAsCompleted,
      deleteBookmark,
      confirmDeleteBook,
      cancelDelete,
      executeDelete,
      formatFileSize,
      formatDate,
      formatDuration,
      truncatePath
    }
  }
}
</script>

<style scoped>
.library {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

/* 页面头部 */
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

/* 统计信息 */
.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.stat-card {
  text-align: center;
}

.stat-card h3 {
  font-size: 2rem;
  font-weight: 600;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
}

.stat-card p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

/* 快速操作已移至页面头部卡片 */

/* 按钮样式已由BaseButton组件提供 */

/* 搜索和过滤区域 */
.controls-section {
  margin-bottom: 2rem;
}

.search-input {
  margin-bottom: 1rem;
}

.filter-controls {
  display: flex;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.filter-label {
  font-size: 0.9rem;
  color: var(--text-primary);
  font-weight: 500;
  white-space: nowrap;
}

.filter-select {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.9rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
  min-width: 100px;
}

.filter-select:focus {
  outline: none;
  border-color: var(--accent-color);
  background: var(--bg-primary);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.filter-select:hover:not(:focus) {
  border-color: var(--text-secondary);
}

.view-toggle {
  display: flex;
  gap: 0.5rem;
  margin-left: auto;
}

/* 图书容器 */
.books-container {
  min-height: 400px;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
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
  margin-bottom: 1.5rem;
}

/* 空状态操作已移至BaseCard的actions插槽 */

/* 图书网格 */
.books-grid {
  display: grid;
  gap: 1.5rem;
}

.books-grid.books-grid {
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
}

.books-grid.books-list {
  grid-template-columns: 1fr;
  gap: 1rem;
}

.book-card-item {
  transition: all 0.2s ease;
}

/* 图书详情模态框 */
.book-detail-content {
  padding: 1.5rem;
  max-height: 80vh;
  overflow-y: auto;
}

.book-detail-header {
  display: flex;
  gap: 2rem;
  margin-bottom: 2rem;
}

.book-cover-large {
  position: relative;
  width: 150px;
  height: 200px;
  flex-shrink: 0;
}

.book-cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--accent-color);
  color: white;
  border-radius: 8px;
  font-size: 3rem;
}

.book-format-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  background: var(--accent-color);
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.7rem;
  font-weight: bold;
}

.book-info-large {
  flex: 1;
}

.book-title-large {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.book-author-large {
  font-size: 1.1rem;
  color: var(--text-secondary);
  margin-bottom: 1.5rem;
}

.book-meta-large {
  margin-bottom: 1rem;
}

.meta-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--border-color);
}

.meta-row:last-child {
  border-bottom: none;
}

.meta-label {
  font-weight: 500;
  color: var(--text-primary);
  flex-shrink: 0;
  margin-right: 1rem;
}

.meta-value {
  color: var(--text-secondary);
  text-align: right;
  word-break: break-all;
}

.progress-section {
  margin-top: 1rem;
}

.progress-bar-large {
  width: 100%;
  height: 8px;
  background-color: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background: var(--accent-color);
  transition: width 0.3s ease;
}

.progress-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
}

.btn-link {
  background: none;
  border: none;
  color: var(--accent-color);
  cursor: pointer;
  font-size: 0.85rem;
  text-decoration: underline;
  padding: 0;
}

.btn-link:hover {
  color: var(--accent-hover);
}

/* 阅读历史 */
.reading-history {
  margin-bottom: 2rem;
}

.reading-history h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid var(--accent-color);
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.history-item {
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border-left: 3px solid var(--accent-color);
}

.history-date {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.history-duration {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

/* 书签列表 */
.bookmarks-section {
  margin-bottom: 2rem;
}

.bookmarks-section h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid #28a745;
}

.bookmarks-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.bookmark-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border-left: 3px solid #28a745;
}

.bookmark-info {
  flex: 1;
}

.bookmark-note {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.bookmark-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.bookmark-delete {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  padding: 0.25rem;
  border-radius: 4px;
  transition: background-color 0.2s ease;
}

.bookmark-delete:hover {
  background-color: #dc3545;
}

.book-detail-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

/* 图书编辑模态框 */
.book-edit-content {
  padding: 1.5rem;
}

.edit-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.9rem;
  margin-bottom: 0.25rem;
}

.form-input {
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.9rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
  transition: all 0.2s ease;
  font-family: inherit;
}

.form-input:focus {
  outline: none;
  border-color: var(--accent-color);
  background: var(--bg-primary);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.form-input::placeholder {
  color: var(--text-muted);
  font-size: 0.85rem;
}

.form-input:hover:not(:focus) {
  border-color: var(--text-secondary);
}

.progress-input-group {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.progress-slider {
  flex: 1;
  height: 6px;
  background: var(--border-color);
  border-radius: 3px;
  outline: none;
  cursor: pointer;
  transition: all 0.2s ease;
}

.progress-slider:hover {
  background: var(--text-secondary);
}

.progress-slider::-webkit-slider-thumb {
  appearance: none;
  width: 20px;
  height: 20px;
  background: var(--accent-color);
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.progress-slider::-webkit-slider-thumb:hover {
  background: var(--accent-hover);
  transform: scale(1.1);
}

.progress-slider::-moz-range-thumb {
  width: 20px;
  height: 20px;
  background: var(--accent-color);
  border-radius: 50%;
  cursor: pointer;
  border: none;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.progress-slider::-moz-range-thumb:hover {
  background: var(--accent-hover);
  transform: scale(1.1);
}

.progress-value {
  font-weight: 500;
  color: var(--accent-color);
  min-width: 40px;
  text-align: right;
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

/* 删除确认对话框 */
.delete-confirm-content {
  text-align: center;
  padding: 1rem;
}

.delete-confirm-content p {
  margin-bottom: 1rem;
  color: var(--text-primary);
}

.delete-warning {
  color: #dc3545;
  font-size: 0.9rem;
  margin-bottom: 1.5rem !important;
}

.delete-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .library {
    padding: 1rem;
  }

  .stats-section {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .quick-actions {
    flex-direction: column;
    align-items: center;
  }

  .filter-controls {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }

  .view-toggle {
    margin-left: 0;
    align-self: center;
  }

  .books-grid.books-grid {
    grid-template-columns: 1fr;
  }

  .book-detail-header {
    flex-direction: column;
    text-align: center;
  }

  .book-cover-large {
    align-self: center;
  }

  .book-detail-actions {
    flex-direction: column;
  }

  .delete-actions {
    flex-direction: column;
  }
}
</style>