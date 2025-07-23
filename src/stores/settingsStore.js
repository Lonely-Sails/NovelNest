import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

/**
 * 应用设置 Store
 * 负责管理应用配置、主题、语言等设置
 */
export const useSettingsStore = defineStore('settings', {
  state: () => ({
    // 应用设置
    app: {
      theme: 'light',
      language: 'zh-CN',
      autoSave: true,
      dataPath: './data',
      windowSize: {
        width: 1200,
        height: 800
      },
      windowPosition: {
        x: null,
        y: null
      }
    },
    
    // 阅读器设置
    reader: {
      fontSize: 16,
      fontFamily: 'system',
      lineHeight: 1.6,
      pageMargin: 20,
      backgroundColor: '#ffffff',
      textColor: '#333333',
      theme: 'light',
      autoSaveProgress: true,
      saveInterval: 30000, // 30秒
      pageAnimation: true,
      keyboardShortcuts: true
    },
    
    // 书源设置
    bookSources: {
      enabled: true,
      autoUpdate: false,
      enabledSources: [],
      searchTimeout: 30000,
      downloadConcurrent: 3,
      downloadDelay: 1000
    },
    
    // 下载设置
    download: {
      concurrent: 3,
      timeout: 30000,
      retryCount: 3,
      autoImport: true,
      downloadPath: './downloads'
    },
    
    // 界面设置
    ui: {
      sidebarWidth: 250,
      showSidebar: true,
      compactMode: false,
      showStatusBar: true,
      showToolbar: true
    },
    
    // 加载状态
    loading: false,
    
    // 是否已初始化
    initialized: false
  }),

  getters: {
    // 当前主题
    currentTheme: (state) => state.app.theme,
    
    // 是否为暗色主题
    isDarkTheme: (state) => state.app.theme === 'dark',
    
    // 阅读器主题样式
    readerThemeStyles: (state) => {
      const themes = {
        light: {
          backgroundColor: '#ffffff',
          color: '#333333',
          borderColor: '#e9ecef'
        },
        dark: {
          backgroundColor: '#1a1a1a',
          color: '#e0e0e0',
          borderColor: '#343a40'
        },
        sepia: {
          backgroundColor: '#f4f1e8',
          color: '#5c4b37',
          borderColor: '#d4c5a9'
        }
      }
      
      return themes[state.reader.theme] || themes.light
    },
    
    // 启用的书源列表
    enabledBookSources: (state) => state.bookSources.enabledSources,
    
    // 是否启用自动保存
    isAutoSaveEnabled: (state) => state.reader.autoSaveProgress
  },

  actions: {
    // 初始化设置
    async initializeSettings() {
      if (this.initialized) return
      
      this.loading = true
      try {
        // 从本地存储加载设置
        await this.loadFromLocalStorage()
        
        // 从后端加载设置
        await this.loadFromBackend()
        
        // 应用主题
        this.applyTheme()
        
        this.initialized = true
      } catch (error) {
        console.error('初始化设置失败:', error)
      } finally {
        this.loading = false
      }
    },

    // 从本地存储加载设置
    async loadFromLocalStorage() {
      try {
        const keys = ['app', 'reader', 'bookSources', 'download', 'ui']
        
        for (const key of keys) {
          const saved = localStorage.getItem(`settings_${key}`)
          if (saved) {
            this[key] = { ...this[key], ...JSON.parse(saved) }
          }
        }
      } catch (error) {
        console.error('从本地存储加载设置失败:', error)
      }
    },

    // 从后端加载设置
    async loadFromBackend() {
      try {
        const settings = await invoke('get_settings')
        if (settings) {
          // 转换后端AppSettings结构到前端结构
          if (settings.theme) this.app.theme = settings.theme
          if (settings.language) this.app.language = settings.language
          if (settings.auto_save !== undefined) this.app.autoSave = settings.auto_save
          if (settings.data_path) this.app.dataPath = settings.data_path
          
          if (settings.reader) {
            const reader = settings.reader
            if (reader.font_size) this.reader.fontSize = reader.font_size
            if (reader.font_family) this.reader.fontFamily = reader.font_family
            if (reader.line_height) this.reader.lineHeight = reader.line_height
            if (reader.page_margin) this.reader.pageMargin = reader.page_margin
            if (reader.background_color) this.reader.backgroundColor = reader.background_color
            if (reader.text_color) this.reader.textColor = reader.text_color
          }
          
          if (settings.book_sources) {
            const bookSources = settings.book_sources
            if (bookSources.enabled !== undefined) this.bookSources.enabled = bookSources.enabled
            if (bookSources.auto_update !== undefined) this.bookSources.autoUpdate = bookSources.auto_update
            if (bookSources.enabled_sources) this.bookSources.enabledSources = bookSources.enabled_sources
          }
          
          if (settings.download) {
            const download = settings.download
            if (download.concurrent) this.download.concurrent = download.concurrent
            if (download.timeout) this.download.timeout = download.timeout
            if (download.retry_count) this.download.retryCount = download.retry_count
          }
        }
      } catch (error) {
        console.error('从后端加载设置失败:', error)
      }
    },

    // 保存设置到本地存储
    saveToLocalStorage() {
      try {
        const keys = ['app', 'reader', 'bookSources', 'download', 'ui']
        
        keys.forEach(key => {
          localStorage.setItem(`settings_${key}`, JSON.stringify(this[key]))
        })
      } catch (error) {
        console.error('保存设置到本地存储失败:', error)
      }
    },

    // 保存设置到后端
    async saveToBackend() {
      try {
        // 转换为后端期望的AppSettings结构
        const settings = {
          theme: this.app.theme,
          language: this.app.language,
          auto_save: this.app.autoSave,
          data_path: this.app.dataPath,
          reader: {
            font_size: this.reader.fontSize,
            font_family: this.reader.fontFamily,
            line_height: this.reader.lineHeight,
            page_margin: this.reader.pageMargin,
            background_color: this.reader.backgroundColor,
            text_color: this.reader.textColor
          },
          book_sources: {
            enabled: this.bookSources.enabled,
            auto_update: this.bookSources.autoUpdate,
            enabled_sources: this.bookSources.enabledSources
          },
          download: {
            concurrent: this.download.concurrent,
            timeout: this.download.timeout,
            retry_count: this.download.retryCount
          }
        }
        
        await invoke('save_settings', { settings })
      } catch (error) {
        console.error('保存设置到后端失败:', error)
        throw error
      }
    },

    // 更新应用设置
    updateAppSettings(newSettings) {
      this.app = { ...this.app, ...newSettings }
      this.saveToLocalStorage()
      this.saveToBackend()
      
      // 如果主题改变，应用新主题
      if (newSettings.theme) {
        this.applyTheme()
      }
    },

    // 更新阅读器设置
    updateReaderSettings(newSettings) {
      this.reader = { ...this.reader, ...newSettings }
      this.saveToLocalStorage()
      this.saveToBackend()
    },

    // 更新书源设置
    updateBookSourceSettings(newSettings) {
      this.bookSources = { ...this.bookSources, ...newSettings }
      this.saveToLocalStorage()
      this.saveToBackend()
    },

    // 更新下载设置
    updateDownloadSettings(newSettings) {
      this.download = { ...this.download, ...newSettings }
      this.saveToLocalStorage()
      this.saveToBackend()
    },

    // 更新界面设置
    updateUISettings(newSettings) {
      this.ui = { ...this.ui, ...newSettings }
      this.saveToLocalStorage()
      this.saveToBackend()
    },

    // 应用主题
    applyTheme() {
      // 更新body类名
      document.body.className = `theme-${this.app.theme}`
      // 触发CSS变量更新
      document.documentElement.setAttribute('data-theme', this.app.theme)
    },

    // 切换主题
    toggleTheme() {
      const themes = ['light', 'dark']
      const currentIndex = themes.indexOf(this.app.theme)
      const nextIndex = (currentIndex + 1) % themes.length
      
      this.updateAppSettings({ theme: themes[nextIndex] })
    },

    // 重置设置
    resetSettings(category = null) {
      const defaultSettings = {
        app: {
          theme: 'light',
          language: 'zh-CN',
          autoSave: true,
          dataPath: './data',
          windowSize: { width: 1200, height: 800 },
          windowPosition: { x: null, y: null }
        },
        reader: {
          fontSize: 16,
          fontFamily: 'system',
          lineHeight: 1.6,
          pageMargin: 20,
          backgroundColor: '#ffffff',
          textColor: '#333333',
          theme: 'light',
          autoSaveProgress: true,
          saveInterval: 30000,
          pageAnimation: true,
          keyboardShortcuts: true
        },
        bookSources: {
          enabled: true,
          autoUpdate: false,
          enabledSources: [],
          searchTimeout: 30000,
          downloadConcurrent: 3,
          downloadDelay: 1000
        },
        download: {
          concurrent: 3,
          timeout: 30000,
          retryCount: 3,
          autoImport: true,
          downloadPath: './downloads'
        },
        ui: {
          sidebarWidth: 250,
          showSidebar: true,
          compactMode: false,
          showStatusBar: true,
          showToolbar: true
        }
      }
      
      if (category && defaultSettings[category]) {
        this[category] = { ...defaultSettings[category] }
      } else {
        Object.keys(defaultSettings).forEach(key => {
          this[key] = { ...defaultSettings[key] }
        })
      }
      
      this.saveToLocalStorage()
      this.saveToBackend()
      this.applyTheme()
    },

    // 导出设置
    exportSettings() {
      const settings = {
        app: this.app,
        reader: this.reader,
        bookSources: this.bookSources,
        download: this.download,
        ui: this.ui,
        exportTime: new Date().toISOString(),
        version: '1.0.0'
      }
      
      return JSON.stringify(settings, null, 2)
    },

    // 导入设置
    async importSettings(settingsJson) {
      try {
        const settings = JSON.parse(settingsJson)
        
        // 验证设置格式
        const requiredKeys = ['app', 'reader', 'bookSources', 'download', 'ui']
        const hasAllKeys = requiredKeys.every(key => settings[key])
        
        if (!hasAllKeys) {
          throw new Error('设置文件格式不正确')
        }
        
        // 应用设置
        Object.keys(settings).forEach(key => {
          if (this[key] && typeof this[key] === 'object') {
            this[key] = { ...this[key], ...settings[key] }
          }
        })
        
        this.saveToLocalStorage()
        await this.saveToBackend()
        this.applyTheme()
        
        return true
      } catch (error) {
        console.error('导入设置失败:', error)
        throw error
      }
    }
  }
})