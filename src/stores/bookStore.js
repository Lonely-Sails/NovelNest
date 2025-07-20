import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

/**
 * 图书管理 Store
 * 负责管理图书库、导入、搜索等功能
 */
export const useBookStore = defineStore('book', {
  state: () => ({
    // 图书列表
    books: [],
    // 当前选中的图书
    currentBook: null,
    // 加载状态
    loading: false,
    // 搜索查询
    searchQuery: '',
    // 过滤后的图书列表
    filteredBooks: [],
    // 最近阅读的图书
    recentBooks: [],
    // 统计信息
    stats: {
      totalBooks: 0,
      readingBooks: 0,
      completedBooks: 0
    }
  }),

  getters: {
    // 获取正在阅读的图书
    readingBooks: (state) => 
      state.books.filter(book => book.reading_progress > 0 && book.reading_progress < 1),
    
    // 获取已完成的图书
    completedBooks: (state) => 
      state.books.filter(book => book.reading_progress >= 1),
    
    // 获取未开始的图书
    unreadBooks: (state) => 
      state.books.filter(book => book.reading_progress === 0),
    
    // 根据搜索查询过滤图书
    searchResults: (state) => {
      if (!state.searchQuery) return state.books
      
      const query = state.searchQuery.toLowerCase()
      return state.books.filter(book => 
        book.title.toLowerCase().includes(query) ||
        (book.author && book.author.toLowerCase().includes(query))
      )
    }
  },

  actions: {
    // 加载所有图书
    async loadBooks() {
      this.loading = true
      try {
        const books = await invoke('get_books')
        this.books = books
        this.updateStats()
        this.updateRecentBooks()
      } catch (error) {
        console.error('加载图书失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 搜索图书
    async searchBooks(query) {
      this.searchQuery = query
      if (!query) {
        this.filteredBooks = this.books
        return
      }

      try {
        const results = await invoke('search_books', { query })
        this.filteredBooks = results
      } catch (error) {
        console.error('搜索图书失败:', error)
        // 如果后端搜索失败，使用前端过滤
        this.filteredBooks = this.searchResults
      }
    },

    // 导入单个图书
    async importBook(filePath) {
      this.loading = true
      try {
        const book = await invoke('import_book', { filePath })
        this.books.push(book)
        this.updateStats()
        return book
      } catch (error) {
        console.error('导入图书失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 批量导入文件夹
    async importFolder(folderPath) {
      this.loading = true
      try {
        const books = await invoke('import_folder', { folderPath })
        this.books.push(...books)
        this.updateStats()
        return books
      } catch (error) {
        console.error('导入文件夹失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 删除图书
    async deleteBook(bookId) {
      try {
        await invoke('delete_book', { bookId })
        this.books = this.books.filter(book => book.id !== bookId)
        this.updateStats()
        this.updateRecentBooks()
      } catch (error) {
        console.error('删除图书失败:', error)
        throw error
      }
    },

    // 获取图书详情
    async getBookById(bookId) {
      const book = this.books.find(b => b.id === bookId)
      if (book) {
        this.currentBook = book
        return book
      }

      try {
        const book = await invoke('get_book_by_id', { bookId })
        this.currentBook = book
        return book
      } catch (error) {
        console.error('获取图书详情失败:', error)
        throw error
      }
    },

    // 更新阅读进度
    async updateReadingProgress(bookId, progress) {
      try {
        await invoke('save_reading_progress', { bookId, progress })
        
        // 更新本地状态
        const book = this.books.find(b => b.id === bookId)
        if (book) {
          book.reading_progress = progress
          book.last_read = new Date().toISOString()
        }
        
        if (this.currentBook && this.currentBook.id === bookId) {
          this.currentBook.reading_progress = progress
          this.currentBook.last_read = new Date().toISOString()
        }
        
        this.updateStats()
        this.updateRecentBooks()
      } catch (error) {
        console.error('更新阅读进度失败:', error)
        throw error
      }
    },

    // 更新统计信息
    updateStats() {
      this.stats.totalBooks = this.books.length
      this.stats.readingBooks = this.readingBooks.length
      this.stats.completedBooks = this.completedBooks.length
    },

    // 更新最近阅读列表
    updateRecentBooks() {
      this.recentBooks = this.books
        .filter(book => book.last_read)
        .sort((a, b) => new Date(b.last_read) - new Date(a.last_read))
        .slice(0, 6)
    },

    // 清空搜索
    clearSearch() {
      this.searchQuery = ''
      this.filteredBooks = []
    },

    // 设置当前图书
    setCurrentBook(book) {
      this.currentBook = book
    }
  }
})