<template>
  <div class="bookmark-panel">
    <div class="panel-header">
      <h3>书签管理</h3>
      <button @click="$emit('close')" class="close-btn">×</button>
    </div>
    
    <div class="bookmark-content">
      <!-- 简化的添加书签 -->
      <div class="add-bookmark-section">
        <button @click="handleAddBookmark" class="add-bookmark-btn">
          📌 添加书签
        </button>
      </div>
      
      <!-- 简化的书签列表 -->
      <div class="bookmarks-list">
        <div v-if="!bookmarks || bookmarks.length === 0" class="empty-state">
          <div class="empty-icon">📖</div>
          <p>暂无书签</p>
        </div>
        
        <div v-else>
          <div 
            v-for="bookmark in bookmarks" 
            :key="bookmark.id || Math.random()"
            class="bookmark-item"
          >
            <div class="bookmark-info">
              <span>位置: {{ bookmark.position || 0 }}</span>
              <span>时间: {{ bookmark.created_at || '未知' }}</span>
            </div>
            <div v-if="bookmark.note" class="bookmark-note">
              {{ bookmark.note }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
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
    console.log('BookmarkPanel 初始化, bookmarks:', props.bookmarks)
    
    const handleAddBookmark = () => {
      try {
        console.log('添加书签, currentPosition:', props.currentPosition)
        emit('add-bookmark', {
          position: props.currentPosition,
          note: null
        })
      } catch (error) {
        console.error('BookmarkPanel: 添加书签失败', error)
      }
    }
    
    return {
      handleAddBookmark
    }
  }
}
</script>

<style scoped>
.bookmark-panel {
  width: 320px;
  background: var(--bg-primary, #ffffff);
  border-left: 1px solid var(--border-color, #e0e0e0);
  display: flex;
  flex-direction: column;
  height: 100%;
  z-index: 50;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color, #e0e0e0);
}

.panel-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary, #333);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0.25rem;
  color: var(--text-secondary, #666);
}

.bookmark-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.add-bookmark-section {
  margin-bottom: 1.5rem;
}

.add-bookmark-btn {
  width: 100%;
  padding: 0.75rem;
  background: var(--primary-color, #007bff);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.add-form {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--bg-secondary, #f8f9fa);
  border-radius: 4px;
}

.bookmark-textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 4px;
  resize: vertical;
  font-family: inherit;
}

.form-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 0.75rem;
}

.confirm-btn, .cancel-btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
}

.confirm-btn {
  background: var(--primary-color, #007bff);
  color: white;
}

.cancel-btn {
  background: var(--bg-tertiary, #e9ecef);
  color: var(--text-primary, #333);
}

.empty-state {
  text-align: center;
  padding: 2rem 1rem;
  color: var(--text-secondary, #666);
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
  color: var(--text-muted, #999);
}

.bookmark-item {
  margin-bottom: 0.75rem;
  padding: 0.75rem;
  background: var(--bg-secondary, #f8f9fa);
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid var(--border-color, #e0e0e0);
}

.bookmark-item:hover {
  background: var(--bg-tertiary, #e9ecef);
}

.bookmark-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.bookmark-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.bookmark-position {
  background: var(--primary-color, #007bff);
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 3px;
  font-size: 0.7rem;
  font-weight: bold;
}

.bookmark-time {
  font-size: 0.8rem;
  color: var(--text-muted, #999);
}

.delete-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.25rem;
  font-size: 1rem;
  color: var(--text-secondary, #666);
}

.delete-btn:hover {
  color: var(--danger-color, #dc3545);
}

.bookmark-note {
  background: var(--bg-primary, #ffffff);
  padding: 0.5rem;
  border-radius: 4px;
  font-size: 0.9rem;
  color: var(--text-primary, #333);
  margin-bottom: 0.5rem;
  border-left: 3px solid var(--primary-color, #007bff);
}

.bookmark-preview {
  font-size: 0.8rem;
  color: var(--text-secondary, #666);
  line-height: 1.4;
}

@media (max-width: 640px) {
  .bookmark-panel {
    width: 100vw;
  }
}
</style>