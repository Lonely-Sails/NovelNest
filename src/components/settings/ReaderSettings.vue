<template>
  <div class="reader-settings">
    <div class="settings-header">
      <h2>阅读体验</h2>
      <p>自定义阅读器的显示效果和交互方式</p>
    </div>
    
    <div class="settings-sections">
      <!-- 显示设置 -->
      <div class="setting-section">
        <h3>显示设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">字体大小</span>
            <span class="label-desc">调整阅读文本的字体大小</span>
          </label>
          <div class="font-size-control">
            <button class="size-btn" @click="decreaseFontSize">A-</button>
            <input 
              type="range" 
              v-model.number="localSettings.fontSize" 
              min="12"
              max="32"
              step="1"
              class="size-slider"
              @input="updateSettings"
            >
            <button class="size-btn" @click="increaseFontSize">A+</button>
            <span class="size-value">{{ localSettings.fontSize }}px</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">行间距</span>
            <span class="label-desc">调整文本行与行之间的间距</span>
          </label>
          <div class="line-height-control">
            <input 
              type="range" 
              v-model.number="localSettings.lineHeight" 
              min="1.0"
              max="3.0"
              step="0.1"
              class="line-height-slider"
              @input="updateSettings"
            >
            <span class="line-height-value">{{ localSettings.lineHeight.toFixed(1) }}</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">页面边距</span>
            <span class="label-desc">调整阅读区域的左右边距</span>
          </label>
          <div class="margin-control">
            <input 
              type="range" 
              v-model.number="localSettings.pageMargin" 
              min="10"
              max="80"
              step="5"
              class="margin-slider"
              @input="updateSettings"
            >
            <span class="margin-value">{{ localSettings.pageMargin }}px</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">最大宽度</span>
            <span class="label-desc">限制阅读区域的最大宽度</span>
          </label>
          <div class="width-control">
            <input 
              type="range" 
              v-model.number="localSettings.maxWidth" 
              min="600"
              max="1200"
              step="50"
              class="width-slider"
              @input="updateSettings"
            >
            <span class="width-value">{{ localSettings.maxWidth }}px</span>
          </div>
        </div>
      </div>
      
      <!-- 交互设置 -->
      <div class="setting-section">
        <h3>交互设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用键盘快捷键</span>
            <span class="label-desc">使用方向键、空格键等进行翻页</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.enableKeyboardShortcuts"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用点击翻页</span>
            <span class="label-desc">点击页面左右区域进行翻页</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.enableClickTurn"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用滚轮翻页</span>
            <span class="label-desc">使用鼠标滚轮进行翻页</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.enableWheelTurn"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">翻页动画</span>
            <span class="label-desc">翻页时显示过渡动画效果</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.enablePageAnimation"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item" v-if="localSettings.enablePageAnimation">
          <label class="setting-label">
            <span class="label-text">动画速度</span>
            <span class="label-desc">翻页动画的持续时间</span>
          </label>
          <div class="animation-speed-control">
            <select 
              v-model="localSettings.animationSpeed" 
              class="speed-select"
              @change="updateSettings"
            >
              <option value="fast">快速 (200ms)</option>
              <option value="normal">正常 (300ms)</option>
              <option value="slow">缓慢 (500ms)</option>
            </select>
          </div>
        </div>
      </div>
      
      <!-- 界面元素 -->
      <div class="setting-section">
        <h3>界面元素</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">显示章节标题</span>
            <span class="label-desc">在阅读页面顶部显示当前章节标题</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.showChapterTitle"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">显示进度条</span>
            <span class="label-desc">在页面底部显示阅读进度条</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.showProgressBar"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">显示页码</span>
            <span class="label-desc">显示当前页码和总页数</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.showPageNumbers"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">显示阅读时间</span>
            <span class="label-desc">显示本次阅读时长和预计剩余时间</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.showReadingTime"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">工具栏位置</span>
            <span class="label-desc">选择阅读器工具栏的显示位置</span>
          </label>
          <select 
            v-model="localSettings.toolbarPosition" 
            class="toolbar-position-select"
            @change="updateSettings"
          >
            <option value="top">顶部</option>
            <option value="bottom">底部</option>
            <option value="auto-hide">自动隐藏</option>
            <option value="hidden">隐藏</option>
          </select>
        </div>
      </div>
      
      <!-- 自动保存 -->
      <div class="setting-section">
        <h3>自动保存</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">自动保存阅读进度</span>
            <span class="label-desc">定期自动保存当前阅读位置</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoSaveProgress"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item" v-if="localSettings.autoSaveProgress">
          <label class="setting-label">
            <span class="label-text">保存间隔</span>
            <span class="label-desc">自动保存的时间间隔</span>
          </label>
          <div class="save-interval-control">
            <input 
              type="range" 
              v-model.number="localSettings.saveInterval" 
              min="10"
              max="300"
              step="10"
              class="interval-slider"
              @input="updateSettings"
            >
            <span class="interval-value">{{ localSettings.saveInterval }}秒</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">退出时保存进度</span>
            <span class="label-desc">关闭阅读器时自动保存当前进度</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.saveOnExit"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
      </div>
    </div>
    
    <!-- 阅读预览 -->
    <div class="preview-section">
      <h3>阅读效果预览</h3>
      <div class="reading-preview" :style="previewStyles">
        <div v-if="localSettings.showChapterTitle" class="preview-chapter-title">
          第一章 开始的地方
        </div>
        <div class="preview-content">
          <p class="preview-paragraph">
            这是一个示例段落，用于预览当前的阅读设置效果。您可以通过调整上面的设置来改变文字的显示效果，包括字体大小、行间距、页面边距等。
          </p>
          <p class="preview-paragraph">
            通过实时预览，您可以直观地看到设置变化对阅读体验的影响，从而找到最适合自己的阅读配置。
          </p>
        </div>
        <div v-if="localSettings.showProgressBar" class="preview-progress-bar">
          <div class="progress-fill" style="width: 35%"></div>
        </div>
        <div v-if="localSettings.showPageNumbers" class="preview-page-info">
          第 1 页 / 共 10 页
        </div>
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <div class="settings-actions">
      <button class="btn-secondary" @click="resetToDefaults">
        恢复默认设置
      </button>
      <button class="btn-primary" @click="saveSettings" :disabled="saving">
        {{ saving ? '保存中...' : '保存设置' }}
      </button>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'

export default {
  name: 'ReaderSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    
    // 本地设置状态
    const localSettings = reactive({
      fontSize: 16,
      lineHeight: 1.6,
      pageMargin: 20,
      maxWidth: 800,
      enableKeyboardShortcuts: true,
      enableClickTurn: true,
      enableWheelTurn: true,
      enablePageAnimation: true,
      animationSpeed: 'normal',
      showChapterTitle: true,
      showProgressBar: true,
      showPageNumbers: true,
      showReadingTime: false,
      toolbarPosition: 'auto-hide',
      autoSaveProgress: true,
      saveInterval: 30,
      saveOnExit: true
    })
    
    // 预览样式
    const previewStyles = computed(() => ({
      fontSize: `${localSettings.fontSize}px`,
      lineHeight: localSettings.lineHeight,
      padding: `20px ${localSettings.pageMargin}px`,
      maxWidth: `${localSettings.maxWidth}px`,
      margin: '0 auto'
    }))
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
    })
    
    // 加载设置
    const loadSettings = () => {
      const readerSettings = settingsStore.reader
      Object.keys(localSettings).forEach(key => {
        if (readerSettings[key] !== undefined) {
          localSettings[key] = readerSettings[key]
        }
      })
    }
    
    // 更新设置
    const updateSettings = () => {
      settingsStore.updateReaderSettings(localSettings)
    }
    
    // 增加字体大小
    const increaseFontSize = () => {
      if (localSettings.fontSize < 32) {
        localSettings.fontSize += 1
        updateSettings()
      }
    }
    
    // 减少字体大小
    const decreaseFontSize = () => {
      if (localSettings.fontSize > 12) {
        localSettings.fontSize -= 1
        updateSettings()
      }
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('阅读设置已保存', 'success')
      } catch (error) {
        console.error('保存阅读设置失败:', error)
        showToast('保存阅读设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认阅读设置吗？')) {
        Object.assign(localSettings, {
          fontSize: 16,
          lineHeight: 1.6,
          pageMargin: 20,
          maxWidth: 800,
          enableKeyboardShortcuts: true,
          enableClickTurn: true,
          enableWheelTurn: true,
          enablePageAnimation: true,
          animationSpeed: 'normal',
          showChapterTitle: true,
          showProgressBar: true,
          showPageNumbers: true,
          showReadingTime: false,
          toolbarPosition: 'auto-hide',
          autoSaveProgress: true,
          saveInterval: 30,
          saveOnExit: true
        })
        updateSettings()
        showToast('已恢复默认阅读设置', 'success')
      }
    }
    
    return {
      localSettings,
      saving,
      previewStyles,
      updateSettings,
      increaseFontSize,
      decreaseFontSize,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.reader-settings {
  padding: 2rem;
  height: 100%;
  overflow-y: auto;
}

.settings-header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.settings-header h2 {
  color: var(--text-primary);
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.settings-header p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.settings-sections {
  margin-bottom: 2rem;
}

.setting-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.setting-section h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  flex: 1;
  margin-right: 1rem;
}

.label-text {
  display: block;
  color: var(--text-primary);
  font-weight: 500;
  margin-bottom: 0.25rem;
}

.label-desc {
  display: block;
  color: var(--text-secondary);
  font-size: 0.85rem;
  line-height: 1.4;
}

/* 字体大小控制 */
.font-size-control {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.size-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  color: var(--text-primary);
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  font-weight: bold;
  transition: all 0.2s ease;
}

.size-btn:hover {
  background: var(--bg-secondary);
  border-color: var(--accent-color);
}

.size-slider {
  width: 120px;
}

.size-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 40px;
  text-align: center;
}

/* 其他滑块控制 */
.line-height-control,
.margin-control,
.width-control,
.save-interval-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.line-height-slider,
.margin-slider,
.width-slider,
.interval-slider {
  width: 120px;
}

.line-height-value,
.margin-value,
.width-value,
.interval-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 50px;
  text-align: center;
}

/* 选择框样式 */
.speed-select,
.toolbar-position-select {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
  min-width: 120px;
}

.speed-select:focus,
.toolbar-position-select:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

/* 开关样式 */
.setting-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.setting-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .switch-slider {
  background-color: var(--accent-color);
}

input:checked + .switch-slider:before {
  transform: translateX(26px);
}

/* 阅读预览 */
.preview-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.preview-section h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.reading-preview {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  position: relative;
  min-height: 200px;
}

.preview-chapter-title {
  text-align: center;
  font-weight: 600;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
}

.preview-content {
  padding: 1rem;
}

.preview-paragraph {
  margin-bottom: 1em;
  color: var(--text-primary);
  text-align: justify;
}

.preview-progress-bar {
  position: absolute;
  bottom: 30px;
  left: 20px;
  right: 20px;
  height: 4px;
  background: var(--border-color);
  border-radius: 2px;
}

.progress-fill {
  height: 100%;
  background: var(--accent-color);
  border-radius: 2px;
  transition: width 0.3s ease;
}

.preview-page-info {
  position: absolute;
  bottom: 8px;
  right: 20px;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

/* 按钮样式 */
.btn-primary,
.btn-secondary {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--accent-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background: var(--border-color);
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .reader-settings {
    padding: 1rem;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .setting-label {
    margin-right: 0;
  }
  
  .font-size-control,
  .line-height-control,
  .margin-control,
  .width-control,
  .save-interval-control {
    width: 100%;
    justify-content: space-between;
  }
  
  .settings-actions {
    flex-direction: column;
  }
}
</style>