import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

/**
 * 阅读器 Store
 * 负责管理阅读状态、设置、书签等功能
 */
export const useReaderStore = defineStore('reader', {
  state: () => ({
    // 当前阅读的图书内容
    content: '',
    // 当前章节信息
    currentChapter: null,
    // 章节列表
    chapters: [],
    // 当前页码
    currentPage: 0,
    // 总页数
    totalPages: 0,
    // 阅读位置
    readingPosition: 0,
    // 加载状态
    loading: false,
    // 阅读设置
    settings: {
      fontSize: 16,
      lineHeight: 1.6,
      fontFamily: 'system',
      theme: 'light',
      pageMargin: 20,
      backgroundColor: '#ffffff',
      textColor: '#333333'
    },
    // 书签列表
    bookmarks: [],
    // 阅读历史
    readingHistory: [],
    // 界面状态
    ui: {
      showSettings: false,
      showToc: false,
      showBookmarks: false,
      fullscreen: false
    }
  }),

  getters: {
    // 阅读进度百分比
    readingProgress: (state) => {
      if (state.totalPages === 0) return 0
      return state.currentPage / state.totalPages
    },

    // 当前主题样式
    themeStyles: (state) => {
      const themes = {
        light: {
          backgroundColor: '#ffffff',
          color: '#333333'
        },
        dark: {
          backgroundColor: '#1a1a1a',
          color: '#e0e0e0'
        },
        sepia: {
          backgroundColor: '#f4f1e8',
          color: '#5c4b37'
        }
      }

      return {
        fontSize: state.settings.fontSize + 'px',
        lineHeight: state.settings.lineHeight,
        fontFamily: state.settings.fontFamily,
        padding: state.settings.pageMargin + 'px',
        ...themes[state.settings.theme]
      }
    },

    // 是否有上一页
    hasPreviousPage: (state) => state.currentPage > 0,

    // 是否有下一页
    hasNextPage: (state) => state.currentPage < state.totalPages - 1,

    // 当前章节索引
    currentChapterIndex: (state) => {
      if (!state.currentChapter) return -1
      return state.chapters.findIndex(chapter => chapter.id === state.currentChapter.id)
    },

    // 是否有上一章
    hasPreviousChapter: (state) => {
      const index = state.currentChapterIndex
      return index > 0
    },

    // 是否有下一章
    hasNextChapter: (state) => {
      const index = state.currentChapterIndex
      return index >= 0 && index < state.chapters.length - 1
    }
  },

  actions: {
    // 加载图书内容
    async loadBookContent(bookId, chapterIndex = 0) {
      this.loading = true
      try {
        const content = await invoke('get_book_content', { bookId })
        this.content = content
        this.calculatePages()
        
        // 加载章节信息
        await this.loadChapters(bookId)
        
        // 加载阅读进度
        await this.loadReadingProgress(bookId)
        
      } catch (error) {
        console.error('加载图书内容失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 加载章节列表
    async loadChapters(bookId) {
      try {
        const chapters = await invoke('get_book_chapters', { bookId })
        this.chapters = chapters
      } catch (error) {
        console.error('加载章节列表失败:', error)
        // 如果没有章节信息，创建默认章节
        this.chapters = [{ id: 'default', title: '正文', index: 0 }]
      }
    },

    // 加载阅读进度
    async loadReadingProgress(bookId) {
      try {
        const progress = await invoke('get_reading_progress', { bookId })
        this.readingPosition = progress.position || 0
        this.currentPage = Math.floor(progress.position * this.totalPages)
      } catch (error) {
        console.error('加载阅读进度失败:', error)
      }
    },

    // 保存阅读进度
    async saveReadingProgress(bookId) {
      try {
        const progress = this.readingProgress
        await invoke('save_reading_progress', { bookId, progress })
        this.readingPosition = progress
      } catch (error) {
        console.error('保存阅读进度失败:', error)
      }
    },

    // 翻到下一页
    nextPage() {
      if (this.hasNextPage) {
        this.currentPage++
      }
    },

    // 翻到上一页
    previousPage() {
      if (this.hasPreviousPage) {
        this.currentPage--
      }
    },

    // 跳转到指定页
    goToPage(page) {
      if (page >= 0 && page < this.totalPages) {
        this.currentPage = page
      }
    },

    // 跳转到下一章
    async nextChapter(bookId) {
      const currentIndex = this.currentChapterIndex
      if (this.hasNextChapter) {
        await this.loadBookContent(bookId, currentIndex + 1)
      }
    },

    // 跳转到上一章
    async previousChapter(bookId) {
      const currentIndex = this.currentChapterIndex
      if (this.hasPreviousChapter) {
        await this.loadBookContent(bookId, currentIndex - 1)
      }
    },

    // 跳转到指定章节
    async goToChapter(bookId, chapterIndex) {
      if (chapterIndex >= 0 && chapterIndex < this.chapters.length) {
        await this.loadBookContent(bookId, chapterIndex)
      }
    },

    // 计算页数（简化版本）
    calculatePages() {
      // 这是一个简化的页数计算，实际实现会更复杂
      const wordsPerPage = 500
      const wordCount = this.content.length
      this.totalPages = Math.max(1, Math.ceil(wordCount / wordsPerPage))
    },

    // 更新阅读设置
    updateSettings(newSettings) {
      this.settings = { ...this.settings, ...newSettings }
      this.saveSettingsToLocal()
      this.calculatePages() // 重新计算页数
    },

    // 从本地存储加载设置
    loadSettingsFromLocal() {
      try {
        const saved = localStorage.getItem('readerSettings')
        if (saved) {
          this.settings = { ...this.settings, ...JSON.parse(saved) }
        }
      } catch (error) {
        console.error('加载阅读设置失败:', error)
      }
    },

    // 保存设置到本地存储
    saveSettingsToLocal() {
      try {
        localStorage.setItem('readerSettings', JSON.stringify(this.settings))
      } catch (error) {
        console.error('保存阅读设置失败:', error)
      }
    },

    // 加载书签
    async loadBookmarks(bookId) {
      try {
        const bookmarks = await invoke('get_bookmarks', { bookId })
        this.bookmarks = bookmarks
      } catch (error) {
        console.error('加载书签失败:', error)
      }
    },

    // 添加书签
    async addBookmark(bookId, note = '') {
      try {
        const position = this.readingPosition
        await invoke('add_bookmark', { bookId, position, note })
        await this.loadBookmarks(bookId)
      } catch (error) {
        console.error('添加书签失败:', error)
        throw error
      }
    },

    // 删除书签
    async deleteBookmark(bookmarkId) {
      try {
        await invoke('delete_bookmark', { bookmarkId })
        this.bookmarks = this.bookmarks.filter(b => b.id !== bookmarkId)
      } catch (error) {
        console.error('删除书签失败:', error)
        throw error
      }
    },

    // 跳转到书签位置
    goToBookmark(bookmark) {
      this.readingPosition = bookmark.position
      this.currentPage = Math.floor(bookmark.position * this.totalPages)
    },

    // 切换界面元素显示状态
    toggleUI(element) {
      this.ui[element] = !this.ui[element]
      
      // 确保同时只显示一个面板
      if (this.ui[element] && element !== 'fullscreen') {
        Object.keys(this.ui).forEach(key => {
          if (key !== element && key !== 'fullscreen') {
            this.ui[key] = false
          }
        })
      }
    },

    // 重置界面状态
    resetUI() {
      Object.keys(this.ui).forEach(key => {
        this.ui[key] = false
      })
    },

    // 全屏切换
    toggleFullscreen() {
      this.ui.fullscreen = !this.ui.fullscreen
      
      if (this.ui.fullscreen) {
        document.documentElement.requestFullscreen?.()
      } else {
        document.exitFullscreen?.()
      }
    }
  }
})