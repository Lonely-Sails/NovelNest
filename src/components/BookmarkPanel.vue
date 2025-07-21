<template>
  <BaseCard class="bookmark-panel">
    <template #header>
      <h3>书签管理</h3>
      <BaseButton @click="$emit('close')" variant="ghost" size="small" class="close-btn">
        ×
      </BaseButton>
    </template>
    
    <div class="bookmark-content">
      <!-- 添加书签 -->
      <div class="add-bookmark-section">
        <BaseButton @click="showAddForm = !showAddForm" variant="primary" icon="📌" class="add-bookmark-btn">
          添加书签
        </BaseButton>
        
        <BaseCard v-if="showAddForm" class="add-form" compact>
          <BaseInput
            v-model="newBookmarkNote" 
            placeholder="添加备注（可选）"
            type="textarea"
            :rows="3"
          />
          <template #actions>
            <BaseButton @click="addBookmark" variant="primary" size="small">确认</BaseButton>
            <BaseButton @click="cancelAdd" variant="secondary" size="small">取消</BaseButton>
          </template>
        </BaseCard>
      </div>
      
      <!-- 书签列表 -->
      <div class="bookmarks-list">
        <BaseCard v-if="bookmarks.length === 0" class="empty-state">
          <div class="empty-icon">📖</div>
          <p>暂无书签</p>
          <p class="empty-hint">在阅读时添加书签，方便快速定位</p>
        </BaseCard>
        
        <BaseCard 
          v-for="bookmark in sortedBookmarks" 
          :key="bookmark.id"
          class="bookmark-item"
          hoverable
          @click="goToBookmark(bookmark)"
        >
          <template #header>
            <div class="bookmark-info">
              <BaseBadge variant="primary" class="bookmark-position">
                {{ formatPosition(bookmark.position) }}
              </BaseBadge>
              <span class="bookmark-time">{{ formatTime(bookmark.created_at) }}</span>
            </div>
            <BaseButton 
              @click.stop="deleteBookmark(bookmark.id)" 
              variant="ghost"
              size="small"
              title="删除书签"
            >
              🗑️
            </BaseButton>
          </template>
          
          <div v-if="bookmark.note" class="bookmark-note">
            {{ bookmark.note }}
          </div>
          
          <div class="bookmark-preview">
            {{ bookmark.content_preview || '点击跳转到此位置' }}
          </div>
        </BaseCard>
      </div>
    </div>
  </BaseCard>
</template>

<script>
import { ref, computed, inject } from 'vue'

export default {
  name: 'BookmarkPanel',
  // 基础组件已全局注册，无需导入
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
  border-left: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  z-index: 50;
  height: 100%;
}

.close-btn {
  font-size: 1.5rem;
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
  width: 100%;
}

.add-form {
  margin-top: 1rem;
}

/* 表单样式已由BaseInput和BaseButton组件提供 */

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
  margin-bottom: 0.75rem;
  cursor: pointer;
}

.bookmark-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.bookmark-time {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.bookmark-note {
  background: var(--bg-secondary);
  padding: 0.5rem;
  border-radius: 4px;
  font-size: 0.9rem;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  border-left: 3px solid var(--primary-color);
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