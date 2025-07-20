<template>
  <div class="bookmark-panel">
    <div class="panel-header">
      <h3>书签管理</h3>
      <button @click="$emit('close')" class="close-btn">×</button>
    </div>
    
    <div class="bookmark-content">
      <!-- 添加书签 -->
      <div class="add-bookmark-section">
        <button @click="showAddForm = !showAddForm" class="add-bookmark-btn">
          <span class="icon">📌</span>
          <span>添加书签</span>
        </button>
        
        <div v-if="showAddForm" class="add-form">
          <textarea 
            v-model="newBookmarkNote" 
            placeholder="添加备注（可选）"
            class="note-input"
            rows="3"
          ></textarea>
          <div class="form-actions">
            <button @click="addBookmark" class="confirm-btn">确认</button>
            <button @click="cancelAdd" class="cancel-btn">取消</button>
          </div>
        </div>
      </div>
      
      <!-- 书签列表 -->
      <div class="bookmarks-list">
        <div v-if="bookmarks.length === 0" class="empty-state">
          <div class="empty-icon">📖</div>
          <p>暂无书签</p>
          <p class="empty-hint">在阅读时添加书签，方便快速定位</p>
        </div>
        
        <div 
          v-for="bookmark in sortedBookmarks" 
          :key="bookmark.id"
          class="bookmark-item"
          @click="goToBookmark(bookmark)"
        >
          <div class="bookmark-header">
            <div class="bookmark-info">
              <span class="bookmark-position">{{ formatPosition(bookmark.position) }}</span>
              <span class="bookmark-time">{{ formatTime(bookmark.created_at) }}</span>
            </div>
            <button 
              @click.stop="deleteBookmark(bookmark.id)" 
              class="delete-btn"
              title="删除书签"
            >
              🗑️
            </button>
          </div>
          
          <div v-if="bookmark.note" class="bookmark-note">
            {{ bookmark.note }}
          </div>
          
          <div class="bookmark-preview">
            {{ bookmark.content_preview || '点击跳转到此位置' }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, inject } from 'vue'

export default {
  name: 'BookmarkPanel',
  props: {
    bookmarks: {
      type: Array,
      default: () => []
    },
    currentPosition: {
      type: Number,
      default: 0
    }
  },
  emits: ['close', 'add-bookmark', 'delete-bookmark', 'go-to-bookmark'],
  setup(props, { emit }) {
    const showAddForm = ref(false)
    const newBookmarkNote = ref('')
    
    const sortedBookmarks = computed(() => {
      return [...props.bookmarks].sort((a, b) => b.created_at - a.created_at)
    })
    
    const addBookmark = () => {
      emit('add-bookmark', {
        position: props.currentPosition,
        note: newBookmarkNote.value.trim()
      })
      cancelAdd()
    }
    
    const cancelAdd = () => {
      showAddForm.value = false
      newBookmarkNote.value = ''
    }
    
    const deleteBookmark = (bookmarkId) => {
      if (confirm('确定要删除这个书签吗？')) {
        emit('delete-bookmark', bookmarkId)
      }
    }
    
    const goToBookmark = (bookmark) => {
      emit('go-to-bookmark', bookmark)
    }
    
    const formatPosition = (position) => {
      const percentage = Math.round(position * 100)
      return `${percentage}%`
    }
    
    const formatTime = (timestamp) => {
      const date = new Date(timestamp)
      const now = new Date()
      const diff = now - date
      
      if (diff < 60000) { // 1分钟内
        return '刚刚'
      } else if (diff < 3600000) { // 1小时内
        return `${Math.floor(diff / 60000)}分钟前`
      } else if (diff < 86400000) { // 1天内
        return `${Math.floor(diff / 3600000)}小时前`
      } else {
        return date.toLocaleDateString()
      }
    }
    
    return {
      showAddForm,
      newBookmarkNote,
      sortedBookmarks,
      addBookmark,
      cancelAdd,
      deleteBookmark,
      goToBookmark,
      formatPosition,
      formatTime
    }
  }
}
</script>

<style scoped>
.bookmark-panel {
  width: 320px;
  background: var(--bg-primary);
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 50;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0.25rem;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.close-btn:hover {
  background-color: var(--sidebar-hover);
  color: var(--text-primary);
}

.bookmark-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

/* 添加书签区域 */
.add-bookmark-section {
  margin-bottom: 2rem;
}

.add-bookmark-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
  padding: 0.75rem;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.3s ease;
}

.add-bookmark-btn:hover {
  background-color: var(--accent-hover);
  transform: translateY(-1px);
}

.add-bookmark-btn .icon {
  font-size: 1.1rem;
}

.add-form {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.note-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
  resize: vertical;
  min-height: 60px;
}

.note-input:focus {
  outline: none;
  border-color: var(--accent-color);
}

.form-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 0.75rem;
}

.confirm-btn,
.cancel-btn {
  flex: 1;
  padding: 0.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.confirm-btn {
  background: var(--accent-color);
  color: white;
}

.confirm-btn:hover {
  background-color: var(--accent-hover);
}

.cancel-btn {
  background: var(--bg-primary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.cancel-btn:hover {
  background-color: var(--sidebar-hover);
}

/* 书签列表 */
.bookmarks-list {
  max-height: none;
}

.empty-state {
  text-align: center;
  padding: 2rem 1rem;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.empty-state p {
  margin: 0.5rem 0;
}

.empty-hint {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.bookmark-item {
  padding: 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin-bottom: 0.75rem;
  cursor: pointer;
  transition: all 0.3s ease;
  background: var(--bg-secondary);
}

.bookmark-item:hover {
  border-color: var(--accent-color);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.bookmark-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.5rem;
}

.bookmark-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.bookmark-position {
  font-weight: 600;
  color: var(--accent-color);
  font-size: 0.9rem;
}

.bookmark-time {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.delete-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
  transition: all 0.3s ease;
  opacity: 0.6;
}

.delete-btn:hover {
  background-color: var(--error-color);
  opacity: 1;
}

.bookmark-note {
  background: var(--bg-primary);
  padding: 0.5rem;
  border-radius: 4px;
  font-size: 0.9rem;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  border-left: 3px solid var(--accent-color);
}

.bookmark-preview {
  font-size: 0.8rem;
  color: var(--text-secondary);
  line-height: 1.4;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

/* 响应式设计 */
@media (max-width: 640px) {
  .bookmark-panel {
    width: 100vw;
  }
}
</style>