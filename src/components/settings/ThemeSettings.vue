<template>
  <div class="theme-settings">
    <div class="settings-header">
      <h2>主题外观</h2>
      <p>自定义应用的视觉外观和主题</p>
    </div>
    
    <div class="settings-sections">
      <!-- 主题选择 -->
      <div class="setting-section">
        <h3>应用主题</h3>
        <div class="theme-grid">
          <div 
            v-for="theme in appThemes" 
            :key="theme.value"
            :class="['theme-card', { active: localSettings.appTheme === theme.value }]"
            @click="selectAppTheme(theme.value)"
          >
            <div class="theme-preview" :style="theme.previewStyle">
              <div class="preview-header"></div>
              <div class="preview-content">
                <div class="preview-text"></div>
                <div class="preview-text short"></div>
              </div>
            </div>
            <div class="theme-info">
              <h4>{{ theme.name }}</h4>
              <p>{{ theme.description }}</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 阅读器主题 -->
      <div class="setting-section">
        <h3>阅读器主题</h3>
        <div class="theme-grid">
          <div 
            v-for="theme in readerThemes" 
            :key="theme.value"
            :class="['theme-card', { active: localSettings.readerTheme === theme.value }]"
            @click="selectReaderTheme(theme.value)"
          >
            <div class="theme-preview reader-preview" :style="theme.previewStyle">
              <div class="reader-text">
                <div class="text-line"></div>
                <div class="text-line"></div>
                <div class="text-line short"></div>
              </div>
            </div>
            <div class="theme-info">
              <h4>{{ theme.name }}</h4>
              <p>{{ theme.description }}</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 自定义设置 -->
      <div class="setting-section">
        <h3>自定义设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">跟随系统主题</span>
            <span class="label-desc">自动根据系统主题切换明暗模式</span>
          </label>
          <BaseSwitch
            v-model="localSettings.followSystemTheme"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用动画效果</span>
            <span class="label-desc">界面切换和交互动画</span>
          </label>
          <BaseSwitch
            v-model="localSettings.enableAnimations"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">紧凑模式</span>
            <span class="label-desc">减少界面元素间距，显示更多内容</span>
          </label>
          <BaseSwitch
            v-model="localSettings.compactMode"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">界面缩放</span>
            <span class="label-desc">调整整体界面大小</span>
          </label>
          <div class="scale-control">
            <input 
              type="range" 
              v-model.number="localSettings.uiScale" 
              min="0.8"
              max="1.5"
              step="0.1"
              class="scale-slider"
              @input="updateSettings"
            >
            <span class="scale-value">{{ Math.round(localSettings.uiScale * 100) }}%</span>
          </div>
        </div>
      </div>
      
      <!-- 颜色自定义 -->
      <div class="setting-section">
        <h3>颜色自定义</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">主色调</span>
            <span class="label-desc">应用的主要强调色</span>
          </label>
          <div class="color-picker-group">
            <input 
              type="color" 
              v-model="localSettings.accentColor" 
              class="color-picker"
              @change="updateSettings"
            >
            <div class="color-presets">
              <div 
                v-for="color in colorPresets" 
                :key="color"
                :class="['color-preset', { active: localSettings.accentColor === color }]"
                :style="{ backgroundColor: color }"
                @click="selectAccentColor(color)"
              ></div>
            </div>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用自定义颜色</span>
            <span class="label-desc">使用自定义颜色替代主题默认颜色</span>
          </label>
          <BaseSwitch
            v-model="localSettings.useCustomColors"
            @change="updateSettings"
          />
        </div>
      </div>
    </div>
    
    <!-- 预览区域 -->
    <div class="preview-section">
      <h3>实时预览</h3>
      <div class="live-preview" :style="previewStyles">
        <div class="preview-toolbar">
          <div class="toolbar-buttons">
            <div class="toolbar-button"></div>
            <div class="toolbar-button"></div>
            <div class="toolbar-button"></div>
          </div>
        </div>
        <div class="preview-sidebar">
          <div class="sidebar-item active"></div>
          <div class="sidebar-item"></div>
          <div class="sidebar-item"></div>
        </div>
        <div class="preview-main">
          <div class="main-header"></div>
          <div class="main-content">
            <div class="content-card">
              <div class="card-header"></div>
              <div class="card-body">
                <div class="card-text"></div>
                <div class="card-text short"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <div class="settings-actions">
      <BaseButton @click="resetToDefaults" variant="secondary">
        恢复默认主题
      </BaseButton>
      <BaseButton @click="saveSettings" variant="primary" :loading="saving">
        应用主题
      </BaseButton>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'

export default {
  name: 'ThemeSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    
    // 本地设置状态
    const localSettings = reactive({
      appTheme: 'light',
      readerTheme: 'light',
      followSystemTheme: false,
      enableAnimations: true,
      compactMode: false,
      uiScale: 1.0,
      accentColor: '#007bff',
      useCustomColors: false
    })
    
    // 应用主题选项
    const appThemes = [
      {
        name: '明亮',
        value: 'light',
        description: '清爽的白色主题',
        previewStyle: {
          backgroundColor: '#ffffff',
          color: '#2c3e50',
          border: '1px solid #dee2e6'
        }
      },
      {
        name: '暗黑',
        value: 'dark',
        description: '护眼的深色主题',
        previewStyle: {
          backgroundColor: '#1a1a1a',
          color: '#e2e8f0',
          border: '1px solid #4a5568'
        }
      },
      {
        name: '自动',
        value: 'auto',
        description: '跟随系统主题',
        previewStyle: {
          background: 'linear-gradient(45deg, #ffffff 50%, #1a1a1a 50%)',
          color: '#666666',
          border: '1px solid #999999'
        }
      }
    ]
    
    // 阅读器主题选项
    const readerThemes = [
      {
        name: '日间',
        value: 'light',
        description: '适合白天阅读',
        previewStyle: {
          backgroundColor: '#ffffff',
          color: '#333333'
        }
      },
      {
        name: '夜间',
        value: 'dark',
        description: '适合夜晚阅读',
        previewStyle: {
          backgroundColor: '#1a1a1a',
          color: '#e0e0e0'
        }
      },
      {
        name: '护眼',
        value: 'sepia',
        description: '温和的护眼色调',
        previewStyle: {
          backgroundColor: '#f4f1e8',
          color: '#5c4b37'
        }
      },
      {
        name: '青色',
        value: 'cyan',
        description: '清新的青色主题',
        previewStyle: {
          backgroundColor: '#e0f7fa',
          color: '#006064'
        }
      }
    ]
    
    // 颜色预设
    const colorPresets = [
      '#007bff', '#28a745', '#dc3545', '#ffc107',
      '#17a2b8', '#6f42c1', '#e83e8c', '#fd7e14',
      '#20c997', '#6c757d', '#343a40', '#f8f9fa'
    ]
    
    // 实时预览样式
    const previewStyles = computed(() => {
      const theme = appThemes.find(t => t.value === localSettings.appTheme)
      const baseStyle = theme ? theme.previewStyle : {}
      
      return {
        ...baseStyle,
        transform: `scale(${localSettings.uiScale})`,
        '--accent-color': localSettings.useCustomColors ? localSettings.accentColor : '#007bff'
      }
    })
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
    })
    
    // 加载设置
    const loadSettings = () => {
      const appSettings = settingsStore.app
      const uiSettings = settingsStore.ui
      
      Object.keys(localSettings).forEach(key => {
        if (appSettings[key] !== undefined) {
          localSettings[key] = appSettings[key]
        } else if (uiSettings[key] !== undefined) {
          localSettings[key] = uiSettings[key]
        }
      })
    }
    
    // 更新设置
    const updateSettings = () => {
      settingsStore.updateAppSettings({
        theme: localSettings.appTheme,
        followSystemTheme: localSettings.followSystemTheme
      })
      
      settingsStore.updateUISettings({
        enableAnimations: localSettings.enableAnimations,
        compactMode: localSettings.compactMode,
        uiScale: localSettings.uiScale,
        accentColor: localSettings.accentColor,
        useCustomColors: localSettings.useCustomColors
      })
      
      settingsStore.updateReaderSettings({
        theme: localSettings.readerTheme
      })
    }
    
    // 选择应用主题
    const selectAppTheme = (theme) => {
      localSettings.appTheme = theme
      updateSettings()
    }
    
    // 选择阅读器主题
    const selectReaderTheme = (theme) => {
      localSettings.readerTheme = theme
      updateSettings()
    }
    
    // 选择强调色
    const selectAccentColor = (color) => {
      localSettings.accentColor = color
      localSettings.useCustomColors = true
      updateSettings()
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('主题设置已保存', 'success')
      } catch (error) {
        console.error('保存主题设置失败:', error)
        showToast('保存主题设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认主题设置吗？')) {
        Object.assign(localSettings, {
          appTheme: 'light',
          readerTheme: 'light',
          followSystemTheme: false,
          enableAnimations: true,
          compactMode: false,
          uiScale: 1.0,
          accentColor: '#007bff',
          useCustomColors: false
        })
        updateSettings()
        showToast('已恢复默认主题设置', 'success')
      }
    }
    
    return {
      localSettings,
      saving,
      appThemes,
      readerThemes,
      colorPresets,
      previewStyles,
      updateSettings,
      selectAppTheme,
      selectReaderTheme,
      selectAccentColor,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.theme-settings {
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

/* 主题网格 */
.theme-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.theme-card {
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-primary);
}

.theme-card:hover {
  border-color: var(--accent-color);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.theme-card.active {
  border-color: var(--accent-color);
  background: var(--primary-bg);
}

.theme-preview {
  height: 80px;
  border-radius: 4px;
  margin-bottom: 0.75rem;
  overflow: hidden;
  position: relative;
}

.preview-header {
  height: 20px;
  background: rgba(0, 0, 0, 0.1);
  margin-bottom: 8px;
}

.preview-content {
  padding: 0 8px;
}

.preview-text {
  height: 8px;
  background: rgba(0, 0, 0, 0.3);
  margin-bottom: 4px;
  border-radius: 2px;
}

.preview-text.short {
  width: 60%;
}

/* 阅读器预览 */
.reader-preview {
  padding: 12px;
}

.reader-text {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.text-line {
  height: 6px;
  background: currentColor;
  opacity: 0.7;
  margin-bottom: 6px;
  border-radius: 2px;
}

.text-line.short {
  width: 70%;
}

.theme-info h4 {
  color: var(--text-primary);
  font-size: 0.9rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.theme-info p {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.3;
}

/* 设置项样式 */
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

/* 缩放控制 */
.scale-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.scale-slider {
  width: 120px;
}

.scale-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 40px;
}

/* 颜色选择器 */
.color-picker-group {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.color-picker {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  overflow: hidden;
}

.color-presets {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.color-preset {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
}

.color-preset:hover {
  transform: scale(1.1);
}

.color-preset.active {
  border-color: var(--text-primary);
  transform: scale(1.2);
}

/* 实时预览 */
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

.live-preview {
  height: 200px;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-color);
  transform-origin: top left;
}

.preview-toolbar {
  height: 40px;
  background: rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  padding: 0 1rem;
}

.toolbar-buttons {
  display: flex;
  gap: 0.5rem;
}

.toolbar-button {
  width: 60px;
  height: 20px;
  background: var(--accent-color, #007bff);
  border-radius: 4px;
  opacity: 0.7;
}

.preview-sidebar {
  width: 60px;
  background: rgba(0, 0, 0, 0.03);
  padding: 0.5rem;
  position: absolute;
  height: calc(100% - 40px);
  top: 40px;
}

.sidebar-item {
  height: 20px;
  background: rgba(0, 0, 0, 0.1);
  margin-bottom: 0.5rem;
  border-radius: 4px;
}

.sidebar-item.active {
  background: var(--accent-color, #007bff);
  opacity: 0.8;
}

.preview-main {
  flex: 1;
  margin-left: 60px;
  padding: 1rem;
}

.main-header {
  height: 30px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  margin-bottom: 1rem;
}

.content-card {
  background: rgba(0, 0, 0, 0.02);
  border-radius: 6px;
  padding: 1rem;
  border: 1px solid rgba(0, 0, 0, 0.05);
}

.card-header {
  height: 16px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 2px;
  margin-bottom: 0.5rem;
  width: 80%;
}

.card-text {
  height: 10px;
  background: rgba(0, 0, 0, 0.15);
  border-radius: 2px;
  margin-bottom: 0.5rem;
}

.card-text.short {
  width: 60%;
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
  background: var(--primary-hover);
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
  .theme-settings {
    padding: 1rem;
  }
  
  .theme-grid {
    grid-template-columns: 1fr;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .setting-label {
    margin-right: 0;
  }
  
  .color-picker-group {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .settings-actions {
    flex-direction: column;
  }
  
  .live-preview {
    transform: scale(0.8);
    transform-origin: top left;
    width: 125%;
  }
}
</style>