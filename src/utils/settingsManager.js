/**
 * 阅读设置管理工具
 */

// 默认设置
export const DEFAULT_READER_SETTINGS = {
  fontSize: 16,
  lineHeight: 1.6,
  fontFamily: 'system',
  theme: 'light',
  pageMargin: 20,
  maxWidth: 800,
  showChapterTitle: true,
  enablePageAnimation: true,
  autoSaveProgress: true,
  saveInterval: 30000, // 30秒
  enableKeyboardShortcuts: true,
  enableClickTurn: true,
  showProgressBar: true,
  showPageNumbers: true
}

// 主题配置
export const READER_THEMES = [
  { 
    name: '日间', 
    value: 'light', 
    bg: '#ffffff', 
    color: '#333333',
    description: '适合白天阅读的明亮主题'
  },
  { 
    name: '夜间', 
    value: 'dark', 
    bg: '#1a1a1a', 
    color: '#e0e0e0',
    description: '适合夜晚阅读的暗色主题'
  },
  { 
    name: '护眼', 
    value: 'sepia', 
    bg: '#f4f1e8', 
    color: '#5c4b37',
    description: '温和的护眼色调，减少眼部疲劳'
  },
  { 
    name: '青色', 
    value: 'cyan', 
    bg: '#e0f7fa', 
    color: '#006064',
    description: '清新的青色主题'
  }
]

// 字体配置
export const FONT_FAMILIES = [
  { name: '系统默认', value: 'system' },
  { name: '宋体', value: 'serif' },
  { name: '黑体', value: 'sans-serif' },
  { name: '等宽字体', value: 'monospace' },
  { name: '微软雅黑', value: "'Microsoft YaHei', sans-serif" },
  { name: '苹方', value: "'PingFang SC', sans-serif" }
]

/**
 * 设置管理类
 */
export class SettingsManager {
  constructor() {
    this.storageKey = 'readerSettings'
    this.settings = { ...DEFAULT_READER_SETTINGS }
    this.listeners = new Set()
  }

  /**
   * 加载设置
   */
  load() {
    try {
      const saved = localStorage.getItem(this.storageKey)
      if (saved) {
        const parsedSettings = JSON.parse(saved)
        this.settings = { ...DEFAULT_READER_SETTINGS, ...parsedSettings }
      }
    } catch (error) {
      console.error('加载设置失败:', error)
      this.settings = { ...DEFAULT_READER_SETTINGS }
    }
    return this.settings
  }

  /**
   * 保存设置
   */
  save() {
    try {
      localStorage.setItem(this.storageKey, JSON.stringify(this.settings))
      this.notifyListeners()
    } catch (error) {
      console.error('保存设置失败:', error)
    }
  }

  /**
   * 更新设置
   */
  update(newSettings) {
    this.settings = { ...this.settings, ...newSettings }
    this.save()
  }

  /**
   * 获取设置
   */
  get(key) {
    return key ? this.settings[key] : this.settings
  }

  /**
   * 重置设置
   */
  reset() {
    this.settings = { ...DEFAULT_READER_SETTINGS }
    this.save()
  }

  /**
   * 添加设置变更监听器
   */
  addListener(callback) {
    this.listeners.add(callback)
  }

  /**
   * 移除设置变更监听器
   */
  removeListener(callback) {
    this.listeners.delete(callback)
  }

  /**
   * 通知所有监听器
   */
  notifyListeners() {
    this.listeners.forEach(callback => {
      try {
        callback(this.settings)
      } catch (error) {
        console.error('设置监听器执行失败:', error)
      }
    })
  }

  /**
   * 导出设置
   */
  export() {
    return {
      settings: this.settings,
      exportTime: new Date().toISOString(),
      version: '1.0.0'
    }
  }

  /**
   * 导入设置
   */
  import(data) {
    try {
      if (data.settings && typeof data.settings === 'object') {
        this.settings = { ...DEFAULT_READER_SETTINGS, ...data.settings }
        this.save()
        return true
      }
      return false
    } catch (error) {
      console.error('导入设置失败:', error)
      return false
    }
  }

  /**
   * 验证设置
   */
  validate(settings) {
    const errors = []
    
    if (settings.fontSize < 12 || settings.fontSize > 32) {
      errors.push('字体大小必须在12-32之间')
    }
    
    if (settings.lineHeight < 1.0 || settings.lineHeight > 3.0) {
      errors.push('行间距必须在1.0-3.0之间')
    }
    
    if (settings.pageMargin < 10 || settings.pageMargin > 80) {
      errors.push('页边距必须在10-80之间')
    }
    
    if (settings.maxWidth < 600 || settings.maxWidth > 1200) {
      errors.push('页面宽度必须在600-1200之间')
    }
    
    const validThemes = READER_THEMES.map(t => t.value)
    if (!validThemes.includes(settings.theme)) {
      errors.push('无效的主题设置')
    }
    
    return errors
  }
}

/**
 * 阅读进度管理类
 */
export class ProgressManager {
  constructor() {
    this.storagePrefix = 'reading_progress_'
    this.autoSaveInterval = null
  }

  /**
   * 保存阅读进度
   */
  save(bookId, progressData) {
    try {
      const data = {
        ...progressData,
        timestamp: Date.now()
      }
      localStorage.setItem(`${this.storagePrefix}${bookId}`, JSON.stringify(data))
    } catch (error) {
      console.error('保存阅读进度失败:', error)
    }
  }

  /**
   * 加载阅读进度
   */
  load(bookId) {
    try {
      const saved = localStorage.getItem(`${this.storagePrefix}${bookId}`)
      return saved ? JSON.parse(saved) : null
    } catch (error) {
      console.error('加载阅读进度失败:', error)
      return null
    }
  }

  /**
   * 删除阅读进度
   */
  delete(bookId) {
    try {
      localStorage.removeItem(`${this.storagePrefix}${bookId}`)
    } catch (error) {
      console.error('删除阅读进度失败:', error)
    }
  }

  /**
   * 启动自动保存
   */
  startAutoSave(bookId, getProgressData, interval = 30000) {
    this.stopAutoSave()
    
    this.autoSaveInterval = setInterval(() => {
      try {
        const progressData = getProgressData()
        if (progressData) {
          this.save(bookId, progressData)
        }
      } catch (error) {
        console.error('自动保存进度失败:', error)
      }
    }, interval)
  }

  /**
   * 停止自动保存
   */
  stopAutoSave() {
    if (this.autoSaveInterval) {
      clearInterval(this.autoSaveInterval)
      this.autoSaveInterval = null
    }
  }

  /**
   * 获取所有阅读进度
   */
  getAllProgress() {
    const progress = {}
    
    try {
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i)
        if (key && key.startsWith(this.storagePrefix)) {
          const bookId = key.replace(this.storagePrefix, '')
          const data = localStorage.getItem(key)
          if (data) {
            progress[bookId] = JSON.parse(data)
          }
        }
      }
    } catch (error) {
      console.error('获取阅读进度失败:', error)
    }
    
    return progress
  }

  /**
   * 清理过期的阅读进度
   */
  cleanup(maxAge = 30 * 24 * 60 * 60 * 1000) { // 默认30天
    const now = Date.now()
    
    try {
      for (let i = localStorage.length - 1; i >= 0; i--) {
        const key = localStorage.key(i)
        if (key && key.startsWith(this.storagePrefix)) {
          const data = localStorage.getItem(key)
          if (data) {
            const progressData = JSON.parse(data)
            if (now - progressData.timestamp > maxAge) {
              localStorage.removeItem(key)
            }
          }
        }
      }
    } catch (error) {
      console.error('清理阅读进度失败:', error)
    }
  }
}

// 创建全局实例
export const settingsManager = new SettingsManager()
export const progressManager = new ProgressManager()

// 工具函数
export const getThemeStyles = (theme) => {
  const themeConfig = READER_THEMES.find(t => t.value === theme)
  if (!themeConfig) return {}
  
  return {
    backgroundColor: themeConfig.bg,
    color: themeConfig.color
  }
}

export const getFontFamilyStyle = (fontFamily) => {
  const fontMap = {
    system: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    serif: '"Times New Roman", Times, serif',
    'sans-serif': 'Arial, Helvetica, sans-serif',
    monospace: '"Courier New", Courier, monospace'
  }
  
  return fontMap[fontFamily] || fontFamily
}

export const validateSettingsValue = (key, value) => {
  const constraints = {
    fontSize: { min: 12, max: 32 },
    lineHeight: { min: 1.0, max: 3.0 },
    pageMargin: { min: 10, max: 80 },
    maxWidth: { min: 600, max: 1200 }
  }
  
  const constraint = constraints[key]
  if (constraint) {
    return Math.max(constraint.min, Math.min(constraint.max, value))
  }
  
  return value
}