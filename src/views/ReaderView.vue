<template>
  <div class="reader" v-if="book">
    <!-- 阅读器头部 -->
    <div class="reader-header" v-show="showControls">
      <div class="book-info">
        <h2>{{ book.title }}</h2>
        <span class="author">{{ book.author || '未知作者' }}</span>
        <span class="chapter-info" v-if="currentChapter">
          {{ currentChapter.title }}
        </span>
      </div>

      <div class="reader-controls">
        <BaseButton @click="toggleSearch" variant="ghost" size="small" title="搜索 (Ctrl+F)" icon="🔍" />
        <BaseButton @click="toggleBookmarks" variant="ghost" size="small" title="书签 (B)" icon="📌" />
        <BaseButton @click="toggleSettings" variant="ghost" size="small" title="设置 (S)" icon="⚙️" />
        <BaseButton @click="toggleToc" variant="ghost" size="small" title="目录 (T)" icon="📋" />
      </div>
    </div>

    <!-- 主要阅读区域 -->
    <div class="reader-content" @click="handleContentClick">
      <!-- 阅读区域 -->
      <div class="reading-area" :style="readerStyles" ref="readingArea" @scroll="handleScroll">
        <div v-if="loading" class="loading">
          <div class="loading-spinner"></div>
          <p>加载中...</p>
        </div>

        <div v-else class="content-container">
          <!-- 章节标题 -->
          <h1 v-if="currentChapter && showChapterTitle" class="chapter-title">
            {{ currentChapter.title }}
          </h1>

          <!-- 文本内容 -->
          <div class="content-text">
            <p v-for="(line, index) in content" :key="index" ref="textLines">{{ line }}</p>
          </div>


        </div>
      </div>

      <!-- 设置面板 -->
      <transition name="slide-left">
        <div v-if="showSettings" class="settings-panel">
          <div class="panel-header">
            <h3>阅读设置</h3>
            <BaseButton @click="showSettings = false" variant="ghost" size="small">×</BaseButton>
          </div>

          <div class="settings-content">
            <!-- 字体设置 -->
            <div class="setting-group">
              <label>字体大小</label>
              <div class="range-control">
                <button @click="adjustFontSize(-1)" class="adjust-btn">-</button>
                <input v-model.number="settings.fontSize" type="range" min="12" max="32" @input="updateSettings"
                  class="range-input">
                <button @click="adjustFontSize(1)" class="adjust-btn">+</button>
                <span class="value">{{ settings.fontSize }}px</span>
              </div>
            </div>

            <!-- 行间距设置 -->
            <div class="setting-group">
              <label>行间距</label>
              <div class="range-control">
                <button @click="adjustLineHeight(-0.1)" class="adjust-btn">-</button>
                <input v-model.number="settings.lineHeight" type="range" min="1.0" max="3.0" step="0.1"
                  @input="updateSettings" class="range-input">
                <button @click="adjustLineHeight(0.1)" class="adjust-btn">+</button>
                <span class="value">{{ settings.lineHeight.toFixed(1) }}</span>
              </div>
            </div>

            <!-- 字体族设置 -->
            <div class="setting-group">
              <label>字体</label>
              <select v-model="settings.fontFamily" @change="updateSettings" class="select-input">
                <option value="system">系统默认</option>
                <option value="serif">宋体</option>
                <option value="sans-serif">黑体</option>
                <option value="monospace">等宽字体</option>
                <option value="'Microsoft YaHei', sans-serif">微软雅黑</option>
                <option value="'PingFang SC', sans-serif">苹方</option>
              </select>
            </div>

            <!-- 主题设置 -->
            <div class="setting-group">
              <label>阅读主题</label>
              <div class="theme-options">
                <button v-for="theme in themes" :key="theme.value" @click="setTheme(theme.value)"
                  :class="['theme-btn', { active: settings.theme === theme.value }]"
                  :style="{ backgroundColor: getThemeBackground(theme.value), color: getThemeColor(theme.value) }">
                  {{ theme.name }}
                </button>
              </div>
            </div>

            <!-- 页边距设置 -->
            <div class="setting-group">
              <label>页边距</label>
              <div class="range-control">
                <button @click="adjustMargin(-5)" class="adjust-btn">-</button>
                <input v-model.number="settings.pageMargin" type="range" min="10" max="80" step="5"
                  @input="updateSettings" class="range-input">
                <button @click="adjustMargin(5)" class="adjust-btn">+</button>
                <span class="value">{{ settings.pageMargin }}px</span>
              </div>
            </div>

            <!-- 其他设置 -->
            <div class="setting-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="settings.showChapterTitle" @change="updateSettings">
                显示章节标题
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="settings.enablePageAnimation" @change="updateSettings">
                翻页动画
              </label>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="settings.autoSaveProgress" @change="updateSettings">
                自动保存阅读进度
              </label>
              <div class="progress-info-section">
                <div class="current-progress">
                  当前进度: {{ readingProgress.toFixed(3) * 100 }}%
                </div>
                <button @click="manualSaveProgress" class="action-btn save-progress-btn">
                  手动保存进度
                </button>
              </div>
            </div>

            <div class="setting-group">
              <label class="checkbox-label">
                <input type="checkbox" v-model="settings.enableKeyboardShortcuts" @change="updateSettings">
                启用键盘快捷键
              </label>
            </div>



            <!-- 设置管理 -->
            <div class="setting-group">
              <label>设置管理</label>
              <div class="setting-actions">
                <button @click="resetSettings" class="action-btn reset-btn">
                  重置设置
                </button>
                <button @click="exportSettings" class="action-btn export-btn">
                  导出设置
                </button>
                <label class="action-btn import-btn">
                  导入设置
                  <input type="file" accept=".json" @change="importSettings" style="display: none;">
                </label>
              </div>
            </div>
          </div>
        </div>
      </transition>

      <!-- 搜索面板 -->
      <transition name="slide-left">
        <SearchPanel v-if="showSearch" :content="content" :current-position="readingProgress"
          @close="showSearch = false" @go-to-position="goToPosition" @highlight-text="highlightSearchText" />
      </transition>

      <!-- 书签面板 -->
      <transition name="slide-left">
        <BookmarkPanel v-if="showBookmarks" :bookmarks="bookmarks" :current-position="readingProgress"
          @close="showBookmarks = false" @add-bookmark="addBookmark" @delete-bookmark="deleteBookmark"
          @go-to-bookmark="goToBookmark" />
      </transition>

      <!-- 目录面板 -->
      <transition name="slide-left">
        <div v-if="showToc" class="toc-panel">
          <div class="panel-header">
            <h3>目录</h3>
            <BaseButton @click="showToc = false" variant="ghost" size="small">×</BaseButton>
          </div>

          <div class="toc-content">
            <div class="toc-list">
              <div v-for="(chapter, index) in chapters" :key="chapter.id || index" @click="goToChapter(index)"
                :class="['toc-item', { active: currentChapterIndex === index }]">
                <span class="chapter-number">{{ index + 1 }}</span>
                <span class="chapter-title">{{ chapter.title || `第${index + 1}章` }}</span>
              </div>

              <!-- 如果没有章节数据，显示默认章节 -->
              <div v-if="chapters.length === 0" class="toc-item active">
                <span class="chapter-number">1</span>
                <span class="chapter-title">正文</span>
              </div>
            </div>
          </div>
        </div>
      </transition>
    </div>

    <!-- 阅读器底部 -->
    <div class="reader-footer" v-show="showControls">
      <div class="progress-section">
        <div class="progress-info">
          <span class="chapter-info">{{ currentChapter.title }}</span>
          <span class="progress-percent">{{ readingProgress.toFixed(3) * 100 }}%</span>
        </div>

        <div class="progress-bar" @click="handleProgressClick">
          <div class="progress-fill" :style="{ width: readingProgress * 100 + '%' }"></div>
        </div>
      </div>

      <div class="navigation-controls">
        <BaseButton @click="previousChapter" :disabled="!hasPreviousChapter" variant="outline" size="small"
          title="上一章 (←)">
          上一章
        </BaseButton>

        <BaseButton @click="nextChapter" :disabled="!hasNextChapter" variant="outline" size="small" title="下一章 (→)">
          下一章
        </BaseButton>
      </div>
    </div>
  </div>

  <!-- 错误状态 -->
  <div v-else class="error-state">
    <div class="error-content">
      <h2>📖</h2>
      <h3>图书未找到</h3>
      <p>请检查图书是否存在或重新选择</p>
      <BaseButton @click="$router.push('/library')" variant="primary">返回图书库</BaseButton>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import SearchPanel from '../components/SearchPanel.vue'
import BookmarkPanel from '../components/BookmarkPanel.vue'
import BaseButton from '../components/base/BaseButton.vue'
import { useToast } from '../composables/useToast'
import { createDebouncedFunction } from '../utils/debounce.js'

// 路由和消息提示
const route = useRoute()
const { showSuccess, showError, showInfo } = useToast()

// 防抖函数引用（稍后初始化）
let debouncedSaveProgress = null
let saveProgressImmediately = null
let cancelSaveProgress = null

// 基础状态
const book = ref(null)
const content = ref([])
const loading = ref(false)
const readingArea = ref(null)
const textLines = ref([])
const bookmarks = ref([])

// UI 状态
const showSettings = ref(false)
const showToc = ref(false)
const showSearch = ref(false)
const showBookmarks = ref(false)
const showControls = ref(true)


// 阅读状态
const currentChapterIndex = ref(0)
const chapters = ref([])
const searchHighlight = ref('')
const scrollPosition = ref(0) // 滚动位置，用于计算全书进度

// 阅读设置
const settings = ref({
  fontSize: 16,
  lineHeight: 1.5,
  fontFamily: 'system',
  theme: 'light',
  pageMargin: 20,
  showChapterTitle: true,
  enablePageAnimation: true,
  autoSaveProgress: true,
  enableKeyboardShortcuts: true
})

// 主题配置
const themes = ref([
  { name: '浅色', value: 'light' },
  { name: '深色', value: 'dark' },
  { name: '护眼', value: 'sepia' },
  { name: '青色', value: 'cyan' }
])

// 计算属性
const currentChapter = computed(() => {
  return chapters.value[currentChapterIndex.value] || { title: '正文' }
})

const showChapterTitle = computed(() => {
  return settings.value.showChapterTitle && currentChapter.value
})

// 获取当前章节的滚动进度
const getScrollProgress = () => {
  if (!readingArea.value) return 0

  const element = readingArea.value
  const scrollTop = element.scrollTop
  const scrollHeight = element.scrollHeight
  const clientHeight = element.clientHeight

  if (scrollHeight <= clientHeight) return 1

  return Math.min(scrollTop / (scrollHeight - clientHeight), 1)
}

const readingProgress = computed(() => {
  if (chapters.value.length === 0) return 0

  // 计算全书进度：当前章节在所有章节中的位置 + 当前章节内的滚动进度
  const chapterProgress = currentChapterIndex.value / chapters.value.length
  const scrollProgress = getScrollProgress()
  const chapterWeight = 1 / chapters.value.length

  return chapterProgress + (scrollProgress * chapterWeight)
})

const hasPreviousChapter = computed(() => {
  return currentChapterIndex.value > 0
})

const hasNextChapter = computed(() => {
  return currentChapterIndex.value < chapters.value.length - 1
})

const readerStyles = computed(() => {
  const themeStyles = getThemeStyles(settings.value.theme)

  return {
    fontSize: settings.value.fontSize + 'px',
    lineHeight: settings.value.lineHeight,
    fontFamily: getFontFamilyStyle(settings.value.fontFamily),
    padding: settings.value.pageMargin + 'px',
    margin: '0 auto',
    ...themeStyles
  }
})

const loadBook = async () => {
  const bookId = route.params.id
  loading.value = true

  try {
    // 从后端获取图书信息
    const books = await invoke('get_books')
    const bookData = books.find(book => book.id === bookId)
    if (!bookData) {
      showError('图书不存在！');
      console.error('图书不存在:', bookId)
      return loading.value = false
    }

    book.value = bookData

    // 获取章节数据
    const chaptersData = await invoke('get_book_chapters', { bookId })
    chapters.value = chaptersData || []

    // 先加载书签
    loadBookmarks()
    // 加载阅读进度
    await loadReadingProgress()

    // 获取当前章节内容
    if (chapters.value.length > 0)
      await loadChapterContent(currentChapterIndex.value)
    else {
      // 如果没有章节数据，直接加载全部内容
      const contentData = await invoke('get_book_content', { bookId })
      content.value = contentData || []
    }
  } catch (error) {
    console.error('加载图书失败:', error)
    showError('加载图书失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

const loadChapterContent = async (chapterIndex) => {
  if (!book.value || !chapters.value[chapterIndex]) return

  try {
    loading.value = true
    const chapterContent = await invoke('get_chapter_content', {
      bookId: book.value.id,
      chapterIndex: chapterIndex
    })

    content.value = chapterContent?.content ?? []
    currentChapterIndex.value = chapterIndex

    // 重置滚动位置到顶部
    if (readingArea.value) readingArea.value.scrollTop = 0

    // 章节切换后延迟保存进度，避免影响切换速度
    if (settings.value.autoSaveProgress && debouncedSaveProgress)
      debouncedSaveProgress()

  } catch (error) {
    console.error('加载章节内容失败:', error)
    showError('加载章节失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

const loadBookmarks = async () => {
  if (!book.value) {
    console.warn('loadBookmarks: book.value is null')
    return
  }

  try {
    console.log('开始加载书签, bookId:', book.value.id)
    const bookmarksData = await invoke('get_bookmarks', { bookId: book.value.id })
    console.log('书签数据加载成功:', bookmarksData)

    // 确保数据是数组格式
    if (Array.isArray(bookmarksData)) {
      bookmarks.value = bookmarksData
    } else {
      console.warn('书签数据不是数组格式:', bookmarksData)
      bookmarks.value = []
    }
  } catch (error) {
    console.error('加载书签失败:', error)
    bookmarks.value = []
    // 不显示错误提示，避免干扰用户
  }
}

// 监听滚动事件来更新进度
const handleScroll = () => {
  scrollPosition.value = readingArea.value?.scrollTop || 0

  if (settings.value.autoSaveProgress && debouncedSaveProgress) {
    // 使用防抖机制，避免频繁保存
    debouncedSaveProgress()
  }
}

// 空格键向下滚动功能
const scrollDown = () => {
  if (!readingArea.value) return

  const element = readingArea.value
  const scrollAmount = element.clientHeight * 0.8 // 滚动80%的可视区域高度

  element.scrollBy({
    top: scrollAmount,
    behavior: 'smooth'
  })
}

// 设置相关方法
const toggleSettings = () => {
  showSettings.value = !showSettings.value
  showToc.value = false
  showSearch.value = false
  showBookmarks.value = false
}

const toggleToc = () => {
  showToc.value = !showToc.value
  showSettings.value = false
  showSearch.value = false
  showBookmarks.value = false
}

const toggleSearch = () => {
  showSearch.value = !showSearch.value
  showSettings.value = false
  showToc.value = false
  showBookmarks.value = false
}

const toggleBookmarks = () => {
  showBookmarks.value = !showBookmarks.value
  showSettings.value = false
  showToc.value = false
  showSearch.value = false
}

const setTheme = (theme) => {
  settings.value.theme = theme
  updateSettings()
}

const getThemeStyles = (theme) => {
  const themeStyles = {
    light: {
      backgroundColor: '#ffffff',
      color: '#333333'
    },
    dark: {
      backgroundColor: '#222222',
      color: '#eeeeee'
    },
    sepia: {
      backgroundColor: '#f4f1e8',
      color: '#5c4b37'
    },
    cyan: {
      backgroundColor: '#e0f7fa',
      color: '#006064'
    }
  }

  return themeStyles[theme] || themeStyles.light
}

const getThemeBackground = (theme) => {
  return getThemeStyles(theme).backgroundColor
}

const getThemeColor = (theme) => {
  return getThemeStyles(theme).color
}

const getFontFamilyStyle = (fontFamily) => {
  const fontFamilyMap = {
    system: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    serif: '"Times New Roman", Georgia, serif',
    'sans-serif': 'Arial, Helvetica, sans-serif',
    monospace: '"Courier New", monospace'
  }

  return fontFamilyMap[fontFamily] || fontFamily || fontFamilyMap.system
}

const adjustFontSize = (delta) => {
  const newSize = Math.min(Math.max(12, settings.value.fontSize + delta), 32)
  settings.value.fontSize = newSize
  updateSettings()
}

const adjustLineHeight = (delta) => {
  const newHeight = Math.min(Math.max(1.0, settings.value.lineHeight + delta), 3.0)
  settings.value.lineHeight = Math.round(newHeight * 10) / 10
  updateSettings()
}

const adjustMargin = (delta) => {
  const newMargin = Math.min(Math.max(10, settings.value.pageMargin + delta), 80)
  settings.value.pageMargin = newMargin
  updateSettings()
}

const updateSettings = () => {
  // 保存设置到本地存储
  localStorage.setItem('reader_settings', JSON.stringify(settings.value))
}

const loadSettings = () => {
  try {
    const savedSettings = localStorage.getItem('reader_settings')
    if (savedSettings)
      settings.value = { ...settings.value, ...JSON.parse(savedSettings) }
  } catch (error) {
    console.error('加载阅读设置失败:', error)
  }
}

const resetSettings = () => {
  settings.value = {
    fontSize: 16,
    lineHeight: 1.5,
    fontFamily: 'system',
    theme: 'light',
    pageMargin: 20,
    showChapterTitle: true,
    enablePageAnimation: true,
    autoSaveProgress: true,
    enableKeyboardShortcuts: true
  }
  updateSettings()
  showInfo('已重置阅读设置')
}

const exportSettings = () => {
  try {
    const settingsJson = JSON.stringify(settings.value, null, 2)
    const blob = new Blob([settingsJson], { type: 'application/json' })
    const url = URL.createObjectURL(blob)

    const a = document.createElement('a')
    a.href = url
    a.download = 'reader_settings.json'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    showSuccess('设置已导出')
  } catch (error) {
    console.error('导出设置失败:', error)
    showError('导出设置失败: ' + error.message)
  }
}

const importSettings = (event) => {
  try {
    const file = event.target.files[0]
    if (!file) return

    const reader = new FileReader()
    reader.onload = (e) => {
      try {
        const data = JSON.parse(e.target.result)
        if (data) {
          settings.value = { ...settings.value, ...data }
          updateSettings()
          showSuccess('设置导入成功')
        }
      } catch (error) {
        console.error('解析设置文件失败:', error)
        showError('导入失败: 无效的设置文件')
      }
    }
    reader.readAsText(file)
  } catch (error) {
    console.error('导入设置失败:', error)
    showError('导入设置失败: ' + error.message)
  }
}

// 章节导航方法

const previousChapter = () => {
  if (hasPreviousChapter.value) {
    loadChapterContent(currentChapterIndex.value - 1)
  }
}

const nextChapter = () => {
  if (hasNextChapter.value) loadChapterContent(currentChapterIndex.value + 1)
}

const goToChapter = (index) => {
  if (index >= 0 && index < chapters.value.length) {
    loadChapterContent(index)
    showToc.value = false
  }
}

const goToPosition = async (position) => {
  if (position >= 0 && position <= 1 && chapters.value.length > 0) {
    console.log('跳转到进度位置:', Math.round(position * 100) + '%')

    // 计算目标章节
    const totalChapters = chapters.value.length
    const targetChapterIndex = Math.floor(position * totalChapters)

    console.log('目标章节:', targetChapterIndex)

    // 如果需要切换章节
    if (targetChapterIndex !== currentChapterIndex.value) {
      console.log('需要切换章节，从', currentChapterIndex.value, '到', targetChapterIndex)
      await loadChapterContent(Math.min(targetChapterIndex, totalChapters - 1))

      // 等待章节内容渲染
      await nextTick()
      await new Promise(resolve => setTimeout(resolve, 300))
    }
  }
}



const goToBookmark = async (bookmark) => {
  try {
    if (!bookmark) {
      console.warn('goToBookmark: 无效的书签对象')
      return
    }

    console.log('跳转到书签:', bookmark)

    // 如果书签有章节信息，直接跳转到对应章节
    if (bookmark.chapter_index !== null && bookmark.chapter_index !== undefined) {
      if (bookmark.chapter_index !== currentChapterIndex.value && chapters.value[bookmark.chapter_index]) {
        await loadChapterContent(bookmark.chapter_index)
        await nextTick()
        await new Promise(resolve => setTimeout(resolve, 300))
      }
    }

    showBookmarks.value = false
  } catch (error) {
    console.error('跳转书签失败:', error)
    showError('跳转书签失败')
  }
}

const handleProgressClick = (event) => {
  const rect = event.currentTarget.getBoundingClientRect()
  if (!rect) return

  const clickX = event.clientX - rect.left
  const progress = clickX / rect.width

  // 计算目标章节
  const targetChapterIndex = Math.floor(progress * chapters.value.length)
  if (targetChapterIndex >= 0 && targetChapterIndex < chapters.value.length) {
    goToChapter(targetChapterIndex)
  }
}

const handleContentClick = (event) => {
  // 移除点击翻页功能，保持点击显示/隐藏控制栏的功能
  setHideControlsTimer()
}

const addBookmark = async (note = '') => {
  if (!book.value) return

  try {
    // 使用当前显示窗口最下面一行的索引作为位置
    const currentLineIndex = getCurrentVisibleLineIndex()
    const position = currentLineIndex

    const result = await invoke('add_bookmark', {
      bookId: book.value.id,
      position: position,
      chapterIndex: currentChapterIndex.value,
      note: note || null
    })

    if (result) {
      await loadBookmarks()
      showSuccess('书签添加成功')
      return result
    }
  } catch (error) {
    console.error('添加书签失败:', error)
    showError('添加书签失败: ' + error.message)
  }
  return null
}

const deleteBookmark = async (bookmarkId) => {
  try {
    await invoke('delete_bookmark', { bookmarkId })
    bookmarks.value = bookmarks.value.filter(b => b.id !== bookmarkId)
    showSuccess('书签删除成功')
  } catch (error) {
    console.error('删除书签失败:', error)
    showError('删除书签失败: ' + error.message)
  }
}

const highlightSearchText = (text) => {
  searchHighlight.value = text
  // 实现文本高亮逻辑
}

// 上次保存的进度状态，用于避免重复保存
let lastSavedChapter = -1
let lastSavedLineIndex = -1

// 计算当前显示窗口最下面一行的索引（基于实际的p标签）
const getCurrentVisibleLineIndex = () => {
  if (!readingArea.value) return 0

  try {
    const readingAreaElement = readingArea.value
    const scrollTop = readingAreaElement.scrollTop
    const clientHeight = readingAreaElement.clientHeight
    
    // 获取所有的p标签元素
    const paragraphs = readingAreaElement.querySelectorAll('.content-text p')
    if (paragraphs.length === 0) return 0

    // 计算可视区域的顶部和底部位置（相对于readingArea）
    const viewportTop = scrollTop
    const viewportBottom = scrollTop + clientHeight

    let lastVisibleIndex = 0

    // 遍历所有p标签，找到最后一个在可视区域内的
    for (let i = 0; i < paragraphs.length; i++) {
      const paragraph = paragraphs[i]
      const rect = paragraph.getBoundingClientRect()
      const readingAreaRect = readingAreaElement.getBoundingClientRect()
      
      // 计算p标签相对于readingArea的位置
      const paragraphTop = rect.top - readingAreaRect.top + scrollTop
      const paragraphBottom = paragraphTop + rect.height

      // 检查p标签是否在可视区域内
      if (paragraphTop < viewportBottom && paragraphBottom > viewportTop) {
        lastVisibleIndex = i
      }
      
      // 如果p标签完全在可视区域下方，停止检查
      if (paragraphTop >= viewportBottom) {
        break
      }
    }

    return lastVisibleIndex
  } catch (error) {
    console.error('计算当前可见行索引失败:', error)
    return 0
  }
}

// 根据行索引滚动到指定位置（基于实际的p标签）
const scrollToLineIndex = (lineIndex) => {
  if (!readingArea.value || lineIndex < 0) return

  try {
    const readingAreaElement = readingArea.value
    const paragraphs = readingAreaElement.querySelectorAll('.content-text p')
    
    if (paragraphs.length === 0 || lineIndex >= paragraphs.length) return

    // 获取目标p标签
    const targetParagraph = paragraphs[lineIndex]
    if (!targetParagraph) return

    // 计算目标p标签的位置
    const rect = targetParagraph.getBoundingClientRect()
    const readingAreaRect = readingAreaElement.getBoundingClientRect()
    const currentScrollTop = readingAreaElement.scrollTop
    
    // 计算目标滚动位置（让目标p标签显示在可视区域顶部）
    const targetScrollTop = rect.top - readingAreaRect.top + currentScrollTop
    
    readingAreaElement.scrollTop = targetScrollTop
    console.log(`滚动到行索引 ${lineIndex}，滚动位置: ${targetScrollTop}`)
  } catch (error) {
    console.error('滚动到指定行索引失败:', error)
  }
}

const saveReadingProgress = async () => {
  if (!book.value) return

  try {
    const currentLineIndex = getCurrentVisibleLineIndex()

    // 检查是否需要保存（避免重复保存相同的进度）
    if (currentChapterIndex.value === lastSavedChapter && currentLineIndex === lastSavedLineIndex) {
      return
    }

    const progressData = {
      bookId: book.value.id,
      chapter: currentChapterIndex.value,
      scrollPosition: scrollPosition.value,
      lineIndex: currentLineIndex,
      timestamp: Date.now()
    }

    // 保存到本地存储（作为备份）
    localStorage.setItem(`reading_progress_${book.value.id}`, JSON.stringify(progressData))

    // 调用后端API保存进度信息
    await invoke('save_reading_progress', {
      bookId: book.value.id,
      currentChapter: currentChapterIndex.value,
      currentLineIndex: currentLineIndex
    })

    // 更新上次保存的状态
    lastSavedChapter = currentChapterIndex.value
    lastSavedLineIndex = currentLineIndex

    console.log(`阅读进度已保存: 章节: ${currentChapterIndex.value}, 行: ${currentLineIndex}`)
  } catch (error) {
    console.error('保存阅读进度失败:', error)
    // 即使后端保存失败，本地存储仍然可用
  }
}

// 初始化防抖保存函数（在 saveReadingProgress 定义之后）
const debouncedFunctions = createDebouncedFunction(saveReadingProgress, 2000)
debouncedSaveProgress = debouncedFunctions.debounced
saveProgressImmediately = debouncedFunctions.immediate
cancelSaveProgress = debouncedFunctions.cancel

// 手动保存进度
const manualSaveProgress = async () => {
  try {
    if (saveProgressImmediately) {
      await saveProgressImmediately()
    } else {
      await saveReadingProgress()
    }
    showSuccess(`进度已保存: ${Math.round(readingProgress.value * 100)}%`)
  } catch (error) {
    showError('保存进度失败: ' + error.message)
  }
}

const loadReadingProgress = async () => {
  if (!book.value) return

  try {
    // 等待DOM完全渲染
    await nextTick()

    // 确保readingArea元素存在
    if (!readingArea.value) {
      console.warn('readingArea元素未找到，延迟重试')
      setTimeout(() => loadReadingProgress(), 500)
      return
    }

    // 先尝试从后端获取进度
    try {
      const [savedChapter, savedLineIndex] = await invoke('get_reading_progress', { bookId: book.value.id })
      if (savedChapter !== undefined && savedChapter !== null) {
        console.log(`从后端加载进度: 章节: ${savedChapter}, 行: ${savedLineIndex}`)

        // 如果需要切换到不同章节
        if (savedChapter !== currentChapterIndex.value && chapters.value[savedChapter]) {
          console.log('切换到保存的章节:', savedChapter)
          await loadChapterContent(savedChapter)

          // 等待章节内容渲染完成
          await nextTick()
        }

        // 根据保存的行索引计算滚动位置
        if (readingArea.value && savedLineIndex > 0) {
          console.dir(textLines.value[savedLineIndex])
          console.log('根据行索引恢复滚动位置: 行索引', savedLineIndex)
        }

        return
      }
    } catch (e) {
      console.log('从后端获取进度失败，尝试从本地存储获取:', e)
    }

    // 如果后端获取失败，尝试从本地存储获取
    const saved = localStorage.getItem(`reading_progress_${book.value.id}`)
    if (saved) {
      const progressData = JSON.parse(saved)
      console.log('从本地存储加载进度:', progressData)

      const targetChapter = progressData.chapter || 0
      const targetScrollPosition = progressData.scrollPosition || 0
      const targetLineIndex = progressData.lineIndex || 0

      // 如果需要切换到不同章节
      if (targetChapter !== currentChapterIndex.value && chapters.value[targetChapter]) {
        console.log('切换到章节:', targetChapter)
        await loadChapterContent(targetChapter)

        // 等待章节内容渲染完成
        await nextTick()
        await new Promise(resolve => setTimeout(resolve, 300))

        // 优先使用行索引恢复位置
        if (readingArea.value) {
          if (targetLineIndex > 0) {
            const lineHeight = 30
            const estimatedScrollPosition = targetLineIndex * lineHeight
            readingArea.value.scrollTop = estimatedScrollPosition
            console.log('根据行索引恢复滚动位置:', estimatedScrollPosition, '行索引:', targetLineIndex)
          } else if (targetScrollPosition > 0) {
            readingArea.value.scrollTop = targetScrollPosition
            console.log('根据滚动位置恢复:', targetScrollPosition)
          }
        }
      } else if (readingArea.value) {
        // 在当前章节内恢复滚动位置
        await new Promise(resolve => setTimeout(resolve, 300))

        if (targetLineIndex > 0) {
          const lineHeight = 30
          const estimatedScrollPosition = targetLineIndex * lineHeight
          readingArea.value.scrollTop = estimatedScrollPosition
          console.log('在当前章节根据行索引恢复滚动位置:', estimatedScrollPosition, '行索引:', targetLineIndex)
        } else if (targetScrollPosition > 0) {
          readingArea.value.scrollTop = targetScrollPosition
          console.log('在当前章节根据滚动位置恢复:', targetScrollPosition)
        }
      }
    } else {
      console.log('没有找到保存的阅读进度')
    }
  } catch (error) {
    console.error('加载阅读进度失败:', error)
  }
}

// 自动隐藏控制栏（全屏模式下）
let hideControlsTimer = null
const setHideControlsTimer = () => {
  clearTimeout(hideControlsTimer)
  showControls.value = true
  hideControlsTimer = setTimeout(() => {
    showControls.value = false
  }, 3000)
}

const handleClick = () => {
  setHideControlsTimer()
}

// 键盘快捷键处理
const handleKeydown = (event) => {
  // 如果正在输入，不处理快捷键
  if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') {
    return
  }

  if (!settings.value.enableKeyboardShortcuts) return

  switch (event.key) {
    case 'ArrowLeft':
      previousChapter()
      event.preventDefault()
      break

    case 'ArrowRight':
      nextChapter()
      event.preventDefault()
      break

    case ' ': // 空格键
      scrollDown()
      event.preventDefault()
      break

    case 'Escape':
      showSettings.value = false
      showToc.value = false
      showSearch.value = false
      showBookmarks.value = false
      event.preventDefault()
      break

    case 's':
    case 'S':
      if (!event.ctrlKey) {
        toggleSettings()
        event.preventDefault()
      }
      break

    case 't':
    case 'T':
      if (!event.ctrlKey) {
        toggleToc()
        event.preventDefault()
      }
      break

    case 'b':
    case 'B':
      if (!event.ctrlKey) {
        toggleBookmarks()
        event.preventDefault()
      }
      break
  }
}



// 页面离开前保存进度
const handleBeforeUnload = () => {
  if (settings.value.autoSaveProgress && book.value) {
    // 取消防抖，立即保存
    if (cancelSaveProgress) {
      cancelSaveProgress()
    }

    // 使用同步方式保存到本地存储
    const currentLineIndex = getCurrentVisibleLineIndex()
    const progressData = {
      bookId: book.value.id,
      chapter: currentChapterIndex.value,
      scrollPosition: scrollPosition.value,
      lineIndex: currentLineIndex,
      timestamp: Date.now()
    }
    localStorage.setItem(`reading_progress_${book.value.id}`, JSON.stringify(progressData))

    // 立即保存到后端（同步调用）
    try {
      // 注意：这里使用同步的方式，但实际上invoke是异步的
      // 在页面卸载时，异步操作可能不会完成
      if (saveProgressImmediately) {
        saveProgressImmediately()
      }
    } catch (error) {
      console.error('页面卸载时保存进度失败:', error)
    }
  }
}

// 生命周期
onMounted(async () => {
  loadSettings()
  await loadBook()

  // 添加事件监听
  document.addEventListener('click', handleClick)
  document.addEventListener('keydown', handleKeydown)
  window.addEventListener('beforeunload', handleBeforeUnload)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClick)
  document.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('beforeunload', handleBeforeUnload)
  clearTimeout(hideControlsTimer)

  // 取消防抖定时器
  if (cancelSaveProgress) {
    cancelSaveProgress()
  }

  // 组件卸载时最后保存一次进度
  if (settings.value.autoSaveProgress && book.value) {
    handleBeforeUnload() // 先同步保存到本地
    if (saveProgressImmediately) {
      saveProgressImmediately() // 立即保存到后端
    }
  }
})
</script>

<style scoped>
.reader {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  position: relative;
  overflow: hidden;
}

/* 阅读器头部 */
.reader-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.5rem;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  transition: opacity 0.3s ease;
}

.book-info {
  flex: 1;
  margin: 0 1.5rem;
  min-width: 0;
}

.book-info h2 {
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.author {
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-right: 1rem;
}

.chapter-info {
  font-size: 0.85rem;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.reader-controls {
  display: flex;
  gap: 0.5rem;
}

/* 主要阅读区域 */
.reader-content {
  flex: 1;
  position: relative;
  overflow: hidden;
  background-color: var(--bg-primary);
}

.reading-area {
  height: 100%;
  overflow-y: auto;
  position: relative;
  padding: 2rem;
  transition: all 0.3s ease;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top-color: var(--accent-color);
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.content-container {
  max-width: 800px;
  margin: 0 auto;
  position: relative;
}

.chapter-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 2rem;
  text-align: center;
}

.content-text {
  line-height: 1.8;
  text-align: justify;
}

.content-text p {
  margin-bottom: 1rem;
  text-indent: 2em;
}

/* 设置面板 */
.settings-panel,
.toc-panel {
  position: absolute;
  top: 0;
  right: 0;
  width: 320px;
  height: 100%;
  background-color: var(--bg-secondary);
  box-shadow: -2px 0 10px rgba(0, 0, 0, 0.1);
  z-index: 10;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.settings-content {
  padding: 1rem;
  overflow-y: auto;
}

.setting-group {
  margin-bottom: 1.5rem;
}

.setting-group label {
  display: block;
  font-weight: 500;
  margin-bottom: 0.5rem;
}

.range-control {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.adjust-btn {
  width: 24px;
  height: 24px;
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}

.range-input {
  flex: 1;
}

.value {
  min-width: 50px;
  text-align: right;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.select-input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.theme-options {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.5rem;
}

.theme-btn {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  text-align: center;
}

.theme-btn.active {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.setting-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.85rem;
  text-align: center;
}

.reset-btn {
  color: var(--error-color);
}

.export-btn,
.import-btn {
  color: var(--accent-color);
}

.progress-info-section {
  margin-top: 0.5rem;
  padding: 0.5rem;
  background-color: var(--bg-primary);
  border-radius: 4px;
  border: 1px solid var(--border-color);
}

.current-progress {
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.save-progress-btn {
  width: 100%;
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-progress-btn:hover {
  background-color: var(--accent-color-hover);
}

/* 目录面板 */
.toc-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.toc-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.toc-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.toc-item:hover {
  background-color: var(--bg-hover);
}

.toc-item.active {
  background-color: var(--accent-color-light);
  color: var(--accent-color);
}

.chapter-number {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border-radius: 50%;
  font-size: 0.8rem;
  font-weight: 500;
}

.toc-item.active .chapter-number {
  background-color: var(--accent-color);
  color: white;
}

/* 阅读器底部 */
.reader-footer {
  padding: 0.75rem 1.5rem;
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  transition: opacity 0.3s ease;
}

.progress-section {
  margin-bottom: 0.75rem;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.progress-bar {
  height: 4px;
  background-color: var(--border-color);
  border-radius: 2px;
  overflow: hidden;
  cursor: pointer;
}

.progress-fill {
  height: 100%;
  background-color: var(--accent-color);
  transition: width 0.3s ease;
}

.navigation-controls {
  display: flex;
  justify-content: center;
  gap: 0.75rem;
}

/* 错误状态 */
.error-state {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
}

.error-content {
  text-align: center;
  padding: 2rem;
}

.error-content h2 {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.error-content h3 {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
}

.error-content p {
  color: var(--text-secondary);
  margin-bottom: 1.5rem;
}

/* 动画 */
.slide-left-enter-active,
.slide-left-leave-active {
  transition: transform 0.3s ease;
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(100%);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .reader-header {
    padding: 0.5rem 1rem;
  }

  .book-info {
    margin: 0 0.5rem;
  }

  .book-info h2 {
    font-size: 1rem;
  }

  .reader-controls {
    gap: 0.25rem;
  }

  .settings-panel,
  .toc-panel {
    width: 100%;
  }

  .navigation-controls {
    flex-wrap: wrap;
  }
}
</style>