import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { pluginManager } from '@/utils/pluginManager'

/**
 * 插件管理 Store
 * 负责管理书源插件、在线搜索、下载等功能
 */
export const usePluginStore = defineStore('plugin', {
  state: () => ({
    // 书源列表
    bookSources: [],
    // 已加载的插件
    loadedPlugins: new Map(),
    // 搜索结果
    searchResults: [],
    // 在线图书章节
    onlineChapters: [],
    // 下载队列
    downloadQueue: [],
    // 下载进度
    downloadProgress: {},
    // 加载状态
    loading: false,
    // 搜索状态
    searching: false,
    // 下载状态
    downloading: false,
    // 当前搜索关键词
    currentSearchQuery: '',
    // 选中的书源
    selectedSources: []
  }),

  getters: {
    // 启用的书源
    enabledSources: (state) => 
      state.bookSources.filter(source => source.enabled),
    
    // 禁用的书源
    disabledSources: (state) => 
      state.bookSources.filter(source => !source.enabled),
    
    // 正在下载的项目
    activeDownloads: (state) => 
      state.downloadQueue.filter(item => item.status === 'downloading'),
    
    // 已完成的下载
    completedDownloads: (state) => 
      state.downloadQueue.filter(item => item.status === 'completed'),
    
    // 下载失败的项目
    failedDownloads: (state) => 
      state.downloadQueue.filter(item => item.status === 'failed'),
    
    // 总下载进度
    totalDownloadProgress: (state) => {
      const total = state.downloadQueue.length
      if (total === 0) return 0
      
      const completed = state.completedDownloads.length
      return (completed / total) * 100
    }
  },

  actions: {
    // 加载所有书源
    async loadBookSources() {
      this.loading = true
      try {
        const sources = await invoke('get_book_sources')
        this.bookSources = sources
      } catch (error) {
        console.error('加载书源失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 加载书源插件
    async loadBookSource(pluginPath) {
      try {
        const source = await invoke('load_book_source', { pluginPath })
        this.bookSources.push(source)
        return source
      } catch (error) {
        console.error('加载书源插件失败:', error)
        throw error
      }
    },

    // 切换书源启用状态
    async toggleBookSource(sourceId, enabled) {
      try {
        await invoke('toggle_book_source', { sourceId, enabled })
        
        const source = this.bookSources.find(s => s.id === sourceId)
        if (source) {
          source.enabled = enabled
        }
      } catch (error) {
        console.error('切换书源状态失败:', error)
        throw error
      }
    },

    // 删除书源
    async removeBookSource(sourceId) {
      try {
        await invoke('remove_book_source', { sourceId })
        this.bookSources = this.bookSources.filter(s => s.id !== sourceId)
        
        // 从插件管理器中卸载插件
        pluginManager.unloadPlugin(sourceId)
        this.loadedPlugins.delete(sourceId)
      } catch (error) {
        console.error('删除书源失败:', error)
        throw error
      }
    },

    // 加载插件代码
    async loadPlugin(sourceId) {
      try {
        // 检查插件管理器中是否已加载
        if (pluginManager.isPluginLoaded(sourceId)) {
          return pluginManager.getPlugin(sourceId)
        }

        // 从后端获取插件代码
        const pluginCode = await invoke('get_plugin_code', { sourceId })
        
        // 使用插件管理器加载插件
        const plugin = await pluginManager.loadPlugin(sourceId, pluginCode)
        
        // 同步到本地缓存（保持向后兼容）
        this.loadedPlugins.set(sourceId, plugin)
        
        return plugin
      } catch (error) {
        console.error(`加载插件 ${sourceId} 失败:`, error)
        throw error
      }
    },

    // 在线搜索图书
    async searchOnlineBooks(query, sourceIds = null) {
      this.searching = true
      this.currentSearchQuery = query
      this.searchResults = []
      
      try {
        // 如果没有指定书源，使用所有启用的书源
        const sources = sourceIds || this.enabledSources.map(s => s.id)
        
        if (sources.length === 0) {
          throw new Error('没有可用的书源，请先启用至少一个书源')
        }
        
        // 并行搜索所有书源
        const searchPromises = sources.map(async (sourceId) => {
          try {
            const plugin = await this.loadPlugin(sourceId)
            const results = await plugin.search(query)
            
            // 为每个结果添加来源信息
            return {
              sourceId,
              sourceName: this.bookSources.find(s => s.id === sourceId)?.name || sourceId,
              results: results.map(result => ({
                ...result,
                sourceId,
                sourceName: this.bookSources.find(s => s.id === sourceId)?.name || sourceId
              })),
              error: null
            }
          } catch (error) {
            console.error(`书源 ${sourceId} 搜索失败:`, error)
            return {
              sourceId,
              sourceName: this.bookSources.find(s => s.id === sourceId)?.name || sourceId,
              results: [],
              error: error.message
            }
          }
        })
        
        const searchResults = await Promise.all(searchPromises)
        
        // 收集所有成功的结果
        this.searchResults = searchResults.flatMap(result => result.results)
        
        // 记录搜索统计信息
        const successfulSources = searchResults.filter(r => r.error === null).length
        const failedSources = searchResults.filter(r => r.error !== null)
        
        if (failedSources.length > 0) {
          console.warn(`${failedSources.length} 个书源搜索失败:`, failedSources)
        }
        
        if (this.searchResults.length === 0 && successfulSources === 0) {
          throw new Error('所有书源搜索失败，请检查网络连接或书源配置')
        }
        
      } catch (error) {
        console.error('在线搜索失败:', error)
        throw error
      } finally {
        this.searching = false
      }
    },

    // 获取在线图书章节
    async getOnlineChapters(sourceId, bookUrl) {
      this.loading = true
      try {
        const plugin = await this.loadPlugin(sourceId)
        const chapters = await plugin.getChapters(bookUrl)
        this.onlineChapters = chapters
        return chapters
      } catch (error) {
        console.error('获取章节列表失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 获取章节内容
    async getChapterContent(sourceId, chapterUrl) {
      try {
        const plugin = await this.loadPlugin(sourceId)
        const content = await plugin.getChapterContent(chapterUrl)
        return content
      } catch (error) {
        console.error('获取章节内容失败:', error)
        throw error
      }
    },

    // 下载在线图书
    async downloadBook(sourceId, bookUrl, bookInfo, selectedChapters = null) {
      const downloadId = `${sourceId}_${Date.now()}`
      
      // 获取要下载的章节索引
      const chapterIndices = selectedChapters 
        ? selectedChapters.map(chapter => chapter.index)
        : this.onlineChapters.map((_, index) => index)
      
      // 添加到下载队列
      const downloadItem = {
        id: downloadId,
        sourceId,
        bookUrl,
        bookInfo,
        chapters: selectedChapters || this.onlineChapters,
        status: 'pending',
        progress: 0,
        downloadedChapters: 0,
        totalChapters: chapterIndices.length,
        startTime: new Date(),
        error: null
      }
      
      this.downloadQueue.push(downloadItem)
      this.downloadProgress[downloadId] = 0
      
      try {
        await this.processDownload(downloadItem, chapterIndices)
      } catch (error) {
        console.error('下载失败:', error)
        downloadItem.status = 'failed'
        downloadItem.error = error.message
        throw error
      }
    },

    // 处理下载任务
    async processDownload(downloadItem, chapterIndices) {
      downloadItem.status = 'downloading'
      this.downloading = true
      
      try {
        // 调用后端下载命令，后端会处理所有下载逻辑
        const bookId = await invoke('download_book', {
          sourceId: downloadItem.sourceId,
          bookUrl: downloadItem.bookUrl,
          chapters: chapterIndices
        })
        
        downloadItem.status = 'completed'
        downloadItem.progress = 100
        downloadItem.downloadedChapters = downloadItem.totalChapters
        downloadItem.endTime = new Date()
        downloadItem.bookId = bookId
        
        // 下载完成后，通知图书库更新
        try {
          // 这里可以触发图书库刷新
          const { useBookStore } = await import('./bookStore')
          const bookStore = useBookStore()
          await bookStore.loadBooks()
        } catch (error) {
          console.warn('更新图书库失败:', error)
        }
        
      } catch (error) {
        downloadItem.status = 'failed'
        downloadItem.error = error.message
        throw error
      } finally {
        this.downloading = this.activeDownloads.length > 0
      }
    },

    // 暂停下载
    pauseDownload(downloadId) {
      const item = this.downloadQueue.find(d => d.id === downloadId)
      if (item && item.status === 'downloading') {
        item.status = 'paused'
      }
    },

    // 恢复下载
    resumeDownload(downloadId) {
      const item = this.downloadQueue.find(d => d.id === downloadId)
      if (item && item.status === 'paused') {
        item.status = 'downloading'
        // 这里可以重新启动下载逻辑
        this.processDownload(item)
      }
    },

    // 取消下载
    cancelDownload(downloadId) {
      const item = this.downloadQueue.find(d => d.id === downloadId)
      if (item && (item.status === 'downloading' || item.status === 'paused')) {
        item.status = 'cancelled'
      }
    },

    // 重试下载
    async retryDownload(downloadId) {
      const item = this.downloadQueue.find(d => d.id === downloadId)
      if (item && (item.status === 'failed' || item.status === 'cancelled')) {
        item.status = 'pending'
        item.error = null
        item.progress = 0
        item.downloadedChapters = 0
        
        // 重新获取章节索引
        const chapterIndices = item.chapters.map((_, index) => index)
        await this.processDownload(item, chapterIndices)
      }
    },

    // 清除下载历史
    clearDownloadHistory() {
      this.downloadQueue = this.downloadQueue.filter(item => 
        item.status === 'downloading' || item.status === 'pending'
      )
    },

    // 清空搜索结果
    clearSearchResults() {
      this.searchResults = []
      this.currentSearchQuery = ''
    },

    // 清空章节列表
    clearOnlineChapters() {
      this.onlineChapters = []
    },

    // 获取插件管理器统计信息
    getPluginManagerStats() {
      return pluginManager.getStats()
    },

    // 获取插件错误信息
    getPluginError(sourceId) {
      return pluginManager.getPluginError(sourceId)
    },

    // 重新加载插件
    async reloadPlugin(sourceId) {
      try {
        // 先卸载插件
        pluginManager.unloadPlugin(sourceId)
        this.loadedPlugins.delete(sourceId)
        
        // 重新加载插件
        return await this.loadPlugin(sourceId)
      } catch (error) {
        console.error(`重新加载插件 ${sourceId} 失败:`, error)
        throw error
      }
    },

    // 测试插件功能
    async testPlugin(sourceId, testKeyword = '测试') {
      try {
        const plugin = await this.loadPlugin(sourceId)
        
        // 测试搜索功能
        const searchResults = await plugin.search(testKeyword)
        
        let testResult = {
          success: true,
          searchCount: searchResults.length,
          details: []
        }
        
        // 如果有搜索结果，测试获取章节功能
        if (searchResults.length > 0) {
          try {
            const firstBook = searchResults[0]
            const chapters = await plugin.getChapters(firstBook.bookUrl)
            testResult.chaptersCount = chapters.length
            testResult.details.push(`获取到 ${chapters.length} 个章节`)
            
            // 如果有章节，测试获取内容功能
            if (chapters.length > 0) {
              try {
                const firstChapter = chapters[0]
                const content = await plugin.getChapterContent(firstChapter.url)
                testResult.hasContent = !!content && !!content.content
                testResult.details.push(`章节内容获取${testResult.hasContent ? '成功' : '失败'}`)
              } catch (error) {
                testResult.details.push(`章节内容获取失败: ${error.message}`)
              }
            }
          } catch (error) {
            testResult.details.push(`章节列表获取失败: ${error.message}`)
          }
        }
        
        return testResult
      } catch (error) {
        return {
          success: false,
          error: error.message,
          details: []
        }
      }
    },

    // 清理插件管理器
    clearPluginManager() {
      pluginManager.clearAllPlugins()
      this.loadedPlugins.clear()
    },

    // 获取插件错误报告
    getPluginErrorReport(sourceId = null) {
      return pluginManager.getErrorReport(sourceId)
    },

    // 清除插件错误历史
    clearPluginErrorHistory(sourceId = null) {
      pluginManager.clearErrorHistory(sourceId)
    },

    // 获取错误处理器
    getErrorHandler() {
      return pluginManager.getErrorHandler()
    }
  }
})