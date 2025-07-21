<template>
  <BaseCard 
    class="book-card" 
    :class="{ compact }"
    hoverable
    @click="$emit('click', book)"
  >
    <template #header>
      <div class="book-cover">
        <div class="book-cover-placeholder">
          <span class="book-icon">📖</span>
        </div>
        <BaseBadge 
          :variant="getFormatVariant(book.format)" 
          class="book-format-badge"
        >
          {{ book.format?.toUpperCase() }}
        </BaseBadge>
      </div>
      
      <div class="book-info">
        <h3 class="book-title">{{ book.title }}</h3>
        <p class="book-author">{{ book.author || '未知作者' }}</p>
        <div class="book-meta">
          <span class="book-size">{{ formatFileSize(book.file_size) }}</span>
          <span class="book-date">{{ formatDate(book.created_at) }}</span>
        </div>
      </div>
    </template>

    <!-- 阅读进度 -->
    <div class="progress-section" v-if="book.reading_progress > 0">
      <div class="progress-info">
        <span class="progress-label">阅读进度</span>
        <span class="progress-percent">{{ Math.round(book.reading_progress * 100) }}%</span>
      </div>
      <div class="progress-bar">
        <div 
          class="progress-fill" 
          :style="{ width: book.reading_progress * 100 + '%' }"
        ></div>
      </div>
    </div>

    <!-- 最近阅读时间 -->
    <div v-if="book.last_read" class="last-read">
      <BaseBadge variant="info" size="small">
        最近阅读: {{ formatDate(book.last_read) }}
      </BaseBadge>
    </div>

    <template #actions v-if="actions && actions.length > 0">
      <BaseButton 
        v-if="actions.includes('read')"
        @click.stop="$emit('read', book)"
        variant="primary"
        size="small"
        icon="📖"
      >
        {{ book.reading_progress > 0 ? '继续阅读' : '开始阅读' }}
      </BaseButton>
      
      <BaseButton 
        v-if="actions.includes('edit')"
        @click.stop="$emit('edit', book)"
        variant="secondary"
        size="small"
        icon="✏️"
      >
        编辑
      </BaseButton>
      
      <BaseButton 
        v-if="actions.includes('delete')"
        @click.stop="$emit('delete', book)"
        variant="danger"
        size="small"
        icon="🗑️"
      >
        删除
      </BaseButton>
    </template>
  </BaseCard>
</template>

<script>
export default {
  name: 'BookCardRefactored',
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
}

.book-card.compact {
  padding: 1rem;
}

.book-card.compact .book-cover {
  width: 60px;
  height: 80px;
}

.book-card.compact .book-title {
  font-size: 1rem;
}

.book-cover {
  position: relative;
  width: 80px;
  height: 120px;
  flex-shrink: 0;
  margin-right: 1rem;
}

.book-cover-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary-color), var(--primary-hover));
  color: white;
  border-radius: 6px;
  font-size: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.book-format-badge {
  position: absolute;
  top: -6px;
  right: -6px;
}

.book-info {
  flex: 1;
  min-width: 0;
}

.book-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.5rem 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
}

.progress-section {
  margin: 1rem 0;
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

.last-read {
  margin-top: 0.75rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .book-cover {
    width: 60px;
    height: 90px;
    margin-right: 0.75rem;
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
}
</style>