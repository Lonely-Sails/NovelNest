<template>
  <div class="home">
    <!-- 欢迎区域 -->
    <BaseCard class="hero-section">
      <div class="hero-content">
        <h1 class="title">NovelNest</h1>
        <p class="subtitle">您的专属小说阅读伴侣</p>
        <div class="quick-actions">
          <BaseButton @click="$router.push('/library')" variant="primary" icon="📚" size="large">
            图书库
          </BaseButton>
          <BaseButton @click="importBook" variant="secondary" icon="📖" size="large">
            导入图书
          </BaseButton>
          <BaseButton @click="$router.push('/downloads')" variant="secondary" icon="🌐" size="large">
            在线下载
          </BaseButton>
        </div>
      </div>
    </BaseCard>

    <!-- 统计信息 -->
    <div class="stats-section">
      <BaseCard class="stat-card" compact>
        <div class="stat-content">
          <h3>{{ totalBooks }}</h3>
          <p>总图书数</p>
        </div>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <div class="stat-content">
          <h3>{{ readingBooks }}</h3>
          <p>正在阅读</p>
        </div>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <div class="stat-content">
          <h3>{{ completedBooks }}</h3>
          <p>已完成</p>
        </div>
      </BaseCard>
    </div>

    <!-- 最近阅读 -->
    <div class="recent-section" v-if="recentBooks.length > 0">
      <h2>最近阅读</h2>
      <div class="book-grid">
        <BaseCard v-for="book in recentBooks" :key="book.id" class="book-card" hoverable @click="openBook(book)">
          <div class="book-cover">
            <span class="book-icon">📖</span>
            <BaseBadge :variant="getProgressVariant(book.reading_progress)" class="progress-badge">
              {{ Math.round(book.reading_progress * 100) }}%
            </BaseBadge>
          </div>
          <div class="book-info">
            <h4>{{ book.title }}</h4>
            <p>{{ book.author || '未知作者' }}</p>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: book.reading_progress * 100 + '%' }"></div>
            </div>
          </div>
        </BaseCard>
      </div>
    </div>

    <!-- 空状态 -->
    <BaseCard v-else class="empty-state">
      <div class="empty-content">
        <div class="empty-icon">📚</div>
        <h3>开始您的阅读之旅</h3>
        <p>导入您的第一本图书，开始享受阅读的乐趣</p>
      </div>
      <template #actions>
        <BaseButton @click="$router.push('/library')" variant="primary" icon="📚">
          前往图书库
        </BaseButton>
        <BaseButton @click="importBook" variant="secondary" icon="📖">
          导入图书
        </BaseButton>
      </template>
    </BaseCard>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useToast } from '@/composables/useToast'

export default {
  name: 'HomeView',
  setup() {
    const router = useRouter()
    const { showInfo } = useToast()
    
    const totalBooks = ref(0)
    const readingBooks = ref(0)
    const completedBooks = ref(0)
    const recentBooks = ref([])

    const loadStats = async () => {
      try {
        // 这些命令将在后续任务中实现
        // const books = await invoke('get_books')
        // totalBooks.value = books.length
        // readingBooks.value = books.filter(book => book.reading_progress > 0 && book.reading_progress < 1).length
        // completedBooks.value = books.filter(book => book.reading_progress >= 1).length
        // recentBooks.value = books.filter(book => book.last_read).slice(0, 6)

        // 临时数据用于展示
        totalBooks.value = 12
        readingBooks.value = 3
        completedBooks.value = 8
        recentBooks.value = [
          {
            id: 1,
            title: '红楼梦',
            author: '曹雪芹',
            reading_progress: 0.65
          },
          {
            id: 2,
            title: '西游记',
            author: '吴承恩',
            reading_progress: 0.23
          },
          {
            id: 3,
            title: '三国演义',
            author: '罗贯中',
            reading_progress: 0.89
          },
          {
            id: 4,
            title: '水浒传',
            author: '施耐庵',
            reading_progress: 0.12
          }
        ]
      } catch (error) {
        console.error('加载统计数据失败:', error)
      }
    }

    const importBook = async () => {
      router.push('/library')
      showInfo('请在图书库页面使用导入功能')
    }

    const openBook = (book) => {
      router.push(`/reader/${book.id}`)
    }

    const getProgressVariant = (progress) => {
      if (progress === 0) return 'secondary'
      if (progress < 0.3) return 'warning'
      if (progress < 0.8) return 'info'
      if (progress < 1) return 'primary'
      return 'success'
    }

    onMounted(() => {
      loadStats()
    })

    return {
      totalBooks,
      readingBooks,
      completedBooks,
      recentBooks,
      importBook,
      openBook,
      getProgressVariant
    }
  }
}
</script>

<style scoped>
.home {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

/* 欢迎区域 */
.hero-section {
  margin-bottom: 3rem;
}

.hero-content {
  text-align: center;
  padding: 2rem;
}

.title {
  font-size: 2.5rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1rem;
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.quick-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

/* 统计信息 */
.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.stat-card {
  text-align: center;
}

.stat-content h3 {
  font-size: 2rem;
  font-weight: 600;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
}

.stat-content p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

/* 最近阅读 */
.recent-section h2 {
  color: var(--text-primary);
  margin-bottom: 1.5rem;
  font-size: 1.5rem;
  font-weight: 600;
}

.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.book-card {
  cursor: pointer;
}

.book-cover {
  position: relative;
  text-align: center;
  margin-bottom: 1rem;
}

.book-icon {
  font-size: 3rem;
  color: var(--primary-color);
}

.progress-badge {
  position: absolute;
  top: -8px;
  right: -8px;
}

.book-info h4 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.book-info p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin-bottom: 1rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.progress-bar {
  width: 100%;
  height: 4px;
  background-color: var(--bg-secondary);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary-color), var(--primary-hover));
  transition: width 0.3s ease;
  border-radius: 2px;
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 3rem 2rem;
}

.empty-content {
  margin-bottom: 2rem;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-content h3 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1.2rem;
  font-weight: 500;
}

.empty-content p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .home {
    padding: 1rem;
  }

  .hero-content {
    padding: 1.5rem;
  }

  .title {
    font-size: 2rem;
  }

  .quick-actions {
    flex-direction: column;
    align-items: center;
  }

  .quick-actions>* {
    width: 100%;
    max-width: 300px;
  }

  .stats-section {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .book-grid {
    grid-template-columns: 1fr;
  }

  .recent-section h2 {
    font-size: 1.3rem;
  }
}

@media (max-width: 480px) {
  .hero-content {
    padding: 1rem;
  }

  .title {
    font-size: 1.8rem;
  }

  .subtitle {
    font-size: 0.9rem;
  }

  .stat-content h3 {
    font-size: 1.5rem;
  }
}
</style>