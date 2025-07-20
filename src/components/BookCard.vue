<template>
  <div 
    class="book-card" 
    :class="{ 'book-card-compact': compact, 'book-card-selected': selected }"
    @click="$emit('click', book)"
  >
    <div class="book-cover">
      <div class="book-cover-image">
        <img v-if="book.coverUrl" :src="book.coverUrl" :alt="book.title" />
        <div v-else class="book-cover-placeholder">
          <span class="book-icon">📖</span>
        </div>
      </div>
      <div class="book-format" v-if="book.format">
        {{ book.format.toUpperCase() }}
      </div>
    </div>
    
    <div class="book-info">
      <h3 class="book-title" :title="book.title">{{ book.title }}</h3>
      <p class="book-author" v-if="book.author" :title="book.author">
        {{ book.author }}
      </p>
      <p class="book-description" v-if="book.description && !compact">
        {{ truncateText(book.description, 100) }}
      </p>
      
      <div class="book-meta">
        <span v-if="book.file_size" class="meta-item">
          {{ formatFileSize(book.file_size) }}
        </span>
        <span v-if="book.created_at" class="meta-item">
          {{ formatDate(book.created_at) }}
        </span>
        <span v-if="book.reading_progress !== undefined" class="meta-item">
          {{ Math.round(book.reading_progress * 100) }}%
        </span>
      </div>
      
      <div v-if="book.reading_progress !== undefined" class="progress-bar">
        <div 
          class="progress-fill" 
          :style="{ width: book.reading_progress * 100 + '%' }"
        ></div>
      </div>
    </div>
    
    <div class="book-actions" v-if="showActions">
      <button 
        v-if="actions.includes('read')"
        @click.stop="$emit('read', book)" 
        class="action-btn action-read"
        title="阅读"
      >
        📖
      </button>
      <button 
        v-if="actions.includes('edit')"
        @click.stop="$emit('edit', book)" 
        class="action-btn action-edit"
        title="编辑"
      >
        ✏️
      </button>
      <button 
        v-if="actions.includes('delete')"
        @click.stop="$emit('delete', book)" 
        class="action-btn action-delete"
        title="删除"
      >
        🗑️
      </button>
      <button 
        v-if="actions.includes('download')"
        @click.stop="$emit('download', book)" 
        class="action-btn action-download"
        title="下载"
      >
        ⬇️
      </button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BookCard',
  props: {
    book: {
      type: Object,
      required: true
    },
    compact: {
      type: Boolean,
      default: false
    },
    selected: {
      type: Boolean,
      default: false
    },
    showActions: {
      type: Boolean,
      default: true
    },
    actions: {
      type: Array,
      default: () => ['read', 'edit', 'delete']
    }
  },
  emits: ['click', 'read', 'edit', 'delete', 'download'],
  setup() {
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
        day: 'numeric'
      })
    }

    const truncateText = (text, maxLength) => {
      if (!text || text.length <= maxLength) return text
      return text.substring(0, maxLength) + '...'
    }

    return {
      formatFileSize,
      formatDate,
      truncateText
    }
  }
}
</script>

<style scoped>
.book-card {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  gap: 1rem;
  position: relative;
  border: 2px solid transparent;
}

.book-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.book-card-selected {
  border-color: #007bff;
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
}

.book-card-compact {
  padding: 1rem;
  flex-direction: row;
  align-items: center;
}

.book-card:not(.book-card-compact) {
  flex-direction: column;
  text-align: center;
}

.book-cover {
  position: relative;
  flex-shrink: 0;
}

.book-card-compact .book-cover {
  width: 60px;
  height: 80px;
}

.book-card:not(.book-card-compact) .book-cover {
  width: 120px;
  height: 160px;
  margin: 0 auto 1rem;
}

.book-cover-image {
  width: 100%;
  height: 100%;
  border-radius: 8px;
  overflow: hidden;
  background: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
}

.book-cover-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.book-cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.book-card-compact .book-icon {
  font-size: 1.5rem;
}

.book-card:not(.book-card-compact) .book-icon {
  font-size: 3rem;
}

.book-format {
  position: absolute;
  top: -4px;
  right: -4px;
  background: #007bff;
  color: white;
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
  font-size: 0.6rem;
  font-weight: bold;
}

.book-info {
  flex: 1;
  min-width: 0;
}

.book-card-compact .book-info {
  text-align: left;
}

.book-title {
  margin: 0 0 0.5rem 0;
  color: #2c3e50;
  font-size: 1.1rem;
  font-weight: 600;
  line-height: 1.3;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.book-card:not(.book-card-compact) .book-title {
  white-space: normal;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.book-author {
  margin: 0 0 0.5rem 0;
  color: #6c757d;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.book-description {
  margin: 0 0 1rem 0;
  color: #495057;
  font-size: 0.85rem;
  line-height: 1.4;
}

.book-meta {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
  font-size: 0.75rem;
  color: #6c757d;
}

.book-card-compact .book-meta {
  flex-wrap: wrap;
  gap: 0.5rem;
}

.book-card:not(.book-card-compact) .book-meta {
  justify-content: center;
}

.meta-item {
  background: #f8f9fa;
  padding: 0.2rem 0.5rem;
  border-radius: 12px;
  white-space: nowrap;
}

.progress-bar {
  width: 100%;
  height: 4px;
  background-color: #e9ecef;
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #007bff, #0056b3);
  transition: width 0.3s ease;
}

.book-actions {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  display: flex;
  gap: 0.25rem;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.book-card:hover .book-actions {
  opacity: 1;
}

.action-btn {
  background: rgba(255, 255, 255, 0.9);
  border: none;
  border-radius: 6px;
  padding: 0.5rem;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  backdrop-filter: blur(4px);
}

.action-btn:hover {
  background: white;
  transform: scale(1.1);
}

.action-read:hover {
  background: #e3f2fd;
}

.action-edit:hover {
  background: #fff3e0;
}

.action-delete:hover {
  background: #ffebee;
}

.action-download:hover {
  background: #e8f5e8;
}

/* 暗色主题支持 */
.theme-dark .book-card {
  background: #2d3748;
  color: #e2e8f0;
}

.theme-dark .book-title {
  color: #e2e8f0;
}

.theme-dark .book-author {
  color: #a0aec0;
}

.theme-dark .book-description {
  color: #cbd5e0;
}

.theme-dark .book-meta {
  color: #a0aec0;
}

.theme-dark .meta-item {
  background: #4a5568;
}

.theme-dark .progress-bar {
  background-color: #4a5568;
}

.theme-dark .book-cover-placeholder {
  background: linear-gradient(135deg, #4a5568 0%, #2d3748 100%);
}

.theme-dark .action-btn {
  background: rgba(45, 55, 72, 0.9);
  color: #e2e8f0;
}

.theme-dark .action-btn:hover {
  background: #4a5568;
}
</style>