<template>
  <BaseCard class="book-card" :class="{ compact }" hoverable clickable @click="$emit('click', book)">
    <!-- 图书封面和基本信息 -->
    <div class="book-header">
      <div class="book-cover">
        <div class="book-cover-placeholder">
          <span class="book-icon">📖</span>
        </div>
        <BaseBadge :variant="getFormatVariant(book.format)" class="book-format-badge" size="small">
          {{ book.format?.toUpperCase() }}
        </BaseBadge>
      </div>

      <div class="book-info">
        <h3 class="book-title" :title="book.title">{{ book.title }}</h3>
        <p class="book-author" :title="book.author || '未知作者'">{{ book.author || '未知作者' }}</p>
        <div class="book-meta">
          <span class="book-size">{{ formatFileSize(book.file_size) }}</span>
          <span class="book-date">{{ formatDate(book.created_at) }}</span>
        </div>
      </div>
    </div>

    <!-- 最近阅读时间 -->
    <div v-if="book.last_read" class="last-read">
      <BaseBadge variant="info" size="small">
        最近阅读: {{ formatDate(book.last_read) }}
      </BaseBadge>
    </div>

    <!-- 操作按钮 -->
    <div class="book-actions" v-if="actions && actions.length > 0">
      <BaseButton v-if="actions.includes('read')" @click.stop="$emit('read', book)" variant="primary" size="small"
        icon="📖" class="action-btn">
        {{ book.reading_progress > 0 ? '继续阅读' : '开始阅读' }}
      </BaseButton>

      <BaseButton v-if="actions.includes('edit')" @click.stop="$emit('edit', book)" variant="ghost" size="small"
        icon="✏️" class="action-btn">
        编辑
      </BaseButton>
    </div>
  </BaseCard>
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
    actions: {
      type: Array,
      default: () => []
    }
  },
  emits: ['click', 'read', 'edit', 'delete'],
  methods: {
    getFormatVariant(format) {
      const formatMap = {
        'txt': 'info',
        'epub': 'success',
        'pdf': 'warning',
        'mobi': 'primary'
      }
      return formatMap[format?.toLowerCase()] || 'secondary'
    },

    formatFileSize(bytes) {
      if (!bytes || bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    },

    formatDate(dateString) {
      if (!dateString) return ''
      const date = new Date(dateString)
      return date.toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      })
    }
  }
}
</script>

<style scoped>
.book-card {
  transition: all 0.2s ease;
  cursor: pointer;
}

.book-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

/* 图书头部布局 */
.book-header {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.book-cover {
  position: relative;
  width: 80px;
  height: 120px;
  flex-shrink: 0;
}

.book-cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary-color), var(--primary-hover));
  color: white;
  border-radius: var(--radius-md);
  font-size: 2rem;
  box-shadow: var(--shadow-sm);
}

.book-format-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  z-index: 1;
}

.book-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
}

.book-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.5rem 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  line-height: 1.3;
}

.book-author {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin: 0 0 0.75rem 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.book-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-muted);
  margin-top: auto;
}

/* 阅读进度 */
.progress-section {
  margin-bottom: 1rem;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.progress-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.progress-percent {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--primary-color);
}

.progress-bar {
  height: 4px;
  background: var(--bg-secondary);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color), var(--primary-hover));
  border-radius: 2px;
  transition: width 0.3s ease;
}

/* 最近阅读 */
.last-read {
  margin-bottom: 1rem;
}

/* 操作按钮 */
.book-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-start;
  align-items: center;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-color);
}

.action-btn {
  flex: 0 0 auto;
}

.action-btn-danger:hover {
  color: var(--error-color) !important;
  background-color: var(--error-bg) !important;
}

/* 紧凑模式 */
.book-card.compact .book-header {
  margin-bottom: 0.75rem;
}

.book-card.compact .book-cover {
  width: 60px;
  height: 80px;
}

.book-card.compact .book-cover-placeholder {
  font-size: 1.5rem;
}

.book-card.compact .book-title {
  font-size: 1rem;
}

.book-card.compact .book-actions {
  padding-top: 0.5rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .book-header {
    gap: 0.75rem;
  }

  .book-cover {
    width: 60px;
    height: 90px;
  }

  .book-cover-placeholder {
    font-size: 1.5rem;
  }

  .book-title {
    font-size: 1rem;
  }

  .book-meta {
    flex-direction: column;
    gap: 0.25rem;
  }

  .book-actions {
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .action-btn {
    flex: 1 1 auto;
    min-width: 0;
  }
}

@media (max-width: 480px) {
  .book-header {
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .book-cover {
    width: 80px;
    height: 120px;
  }

  .book-info {
    width: 100%;
  }

  .book-title,
  .book-author {
    white-space: normal;
    text-overflow: unset;
    overflow: visible;
  }

  .book-meta {
    justify-content: center;
  }
}
</style>