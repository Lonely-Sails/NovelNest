<template>
  <div class="home">
    <div class="hero-section">
      <h1 class="title">NovelNest</h1>
      <p class="subtitle">您的专属小说阅读伴侣</p>
      <div class="quick-actions">
        <button @click="$router.push('/library')" class="btn btn-primary">
          <span class="icon">📚</span>
          图书库
        </button>
        <button @click="importBook" class="btn btn-secondary">
          <span class="icon">📖</span>
          导入图书
        </button>
        <button @click="$router.push('/downloads')" class="btn btn-secondary">
          <span class="icon">🌐</span>
          在线下载
        </button>
      </div>
    </div>

    <div class="stats-section">
      <div class="stat-card">
        <h3>{{ totalBooks }}</h3>
        <p>总图书数</p>
      </div>
      <div class="stat-card">
        <h3>{{ readingBooks }}</h3>
        <p>正在阅读</p>
      </div>
      <div class="stat-card">
        <h3>{{ completedBooks }}</h3>
        <p>已完成</p>
      </div>
    </div>

    <div class="recent-section" v-if="recentBooks.length > 0">
      <h2>最近阅读</h2>
      <div class="book-grid">
        <div 
          v-for="book in recentBooks" 
          :key="book.id"
          class="book-card"
          @click="openBook(book)"
        >
          <div class="book-cover">
            <span class="book-icon">📖</span>
          </div>
          <div class="book-info">
            <h4>{{ book.title }}</h4>
            <p>{{ book.author || '未知作者' }}</p>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: book.reading_progress * 100 + '%' }"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'HomeView',
  setup() {
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
        totalBooks.value = 0
        readingBooks.value = 0
        completedBooks.value = 0
        recentBooks.value = []
      } catch (error) {
        console.error('加载统计数据失败:', error)
      }
    }

    const importBook = async () => {
      try {
        // 这个命令将在后续任务中实现
        // await invoke('import_book')
        alert('图书导入功能将在后续版本中实现')
      } catch (error) {
        console.error('导入图书失败:', error)
      }
    }

    const openBook = (book) => {
      this.$router.push(`/reader/${book.id}`)
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
      openBook
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

.hero-section {
  text-align: center;
  margin-bottom: 3rem;
  padding: 2rem;
  background: var(--bg-primary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
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

.btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--accent-color);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  border-color: var(--accent-color);
}

.icon {
  font-size: 1rem;
}

.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.stat-card {
  background: var(--bg-primary);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  text-align: center;
}

.stat-card h3 {
  font-size: 2rem;
  font-weight: 600;
  color: var(--accent-color);
  margin-bottom: 0.5rem;
}

.stat-card p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

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
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s ease;
}

.book-card:hover {
  border-color: var(--accent-color);
}

.book-cover {
  text-align: center;
  margin-bottom: 1rem;
}

.book-icon {
  font-size: 3rem;
}

.book-info h4 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1rem;
  font-weight: 500;
}

.book-info p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin-bottom: 1rem;
}

.progress-bar {
  width: 100%;
  height: 4px;
  background-color: var(--border-color);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--accent-color);
  transition: width 0.3s ease;
}
</style>