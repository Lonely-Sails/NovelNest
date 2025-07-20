<template>
  <div class="app-settings">
    <div class="settings-header">
      <h2>应用设置</h2>
      <p>配置应用的基本行为和偏好</p>
    </div>
    
    <div class="settings-sections">
      <!-- 语言设置 -->
      <div class="setting-section">
        <h3>语言和地区</h3>
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">界面语言</span>
            <span class="label-desc">选择应用界面显示语言</span>
          </label>
          <select 
            v-model="localSettings.language" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="zh-CN">简体中文</option>
            <option value="zh-TW">繁體中文</option>
            <option value="en-US">English</option>
            <option value="ja-JP">日本語</option>
          </select>
        </div>
      </div>
      
      <!-- 启动设置 -->
      <div class="setting-section">
        <h3>启动和窗口</h3>
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启动时恢复窗口大小</span>
            <span class="label-desc">记住上次关闭时的窗口大小和位置</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.rememberWindowState"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">最小化到系统托盘</span>
            <span class="label-desc">关闭窗口时最小化到系统托盘而不是退出</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.minimizeToTray"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">开机自启动</span>
            <span class="label-desc">系统启动时自动启动应用</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoStart"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 数据存储 -->
      <div class="setting-section">
        <h3>数据存储</h3>
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">数据存储路径</span>
            <span class="label-desc">图书和配置文件的存储位置</span>
          </label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="localSettings.dataPath" 
              class="setting-input"
              readonly
            >
            <button class="btn-secondary" @click="selectDataPath">
              选择路径
            </button>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">自动保存</span>
            <span class="label-desc">自动保存阅读进度和应用设置</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoSave"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item" v-if="localSettings.autoSave">
          <label class="setting-label">
            <span class="label-text">自动保存间隔</span>
            <span class="label-desc">自动保存的时间间隔（秒）</span>
          </label>
          <div class="number-input-group">
            <input 
              type="number" 
              v-model.number="localSettings.saveInterval" 
              class="setting-input"
              min="10"
              max="300"
              step="10"
              @change="updateSettings"
            >
            <span class="input-unit">秒</span>
          </div>
        </div>
      </div>
      
      <!-- 性能设置 -->
      <div class="setting-section">
        <h3>性能优化</h3>
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用硬件加速</span>
            <span class="label-desc">使用GPU加速提升界面渲染性能</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.hardwareAcceleration"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">内存缓存大小</span>
            <span class="label-desc">用于缓存图书内容的内存大小（MB）</span>
          </label>
          <div class="number-input-group">
            <input 
              type="number" 
              v-model.number="localSettings.cacheSize" 
              class="setting-input"
              min="50"
              max="1000"
              step="50"
              @change="updateSettings"
            >
            <span class="input-unit">MB</span>
          </div>
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
import { ref, reactive, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

export default {
  name: 'AppSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    
    // 本地设置状态
    const localSettings = reactive({
      language: 'zh-CN',
      rememberWindowState: true,
      minimizeToTray: false,
      autoStart: false,
      dataPath: './data',
      autoSave: true,
      saveInterval: 30,
      hardwareAcceleration: true,
      cacheSize: 200
    })
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
    })
    
    // 加载设置
    const loadSettings = () => {
      const appSettings = settingsStore.app
      Object.keys(localSettings).forEach(key => {
        if (appSettings[key] !== undefined) {
          localSettings[key] = appSettings[key]
        }
      })
    }
    
    // 更新设置
    const updateSettings = () => {
      settingsStore.updateAppSettings(localSettings)
    }
    
    // 选择数据路径
    const selectDataPath = async () => {
      try {
        const selected = await open({
          directory: true,
          title: '选择数据存储路径'
        })
        
        if (selected) {
          localSettings.dataPath = selected
          updateSettings()
          showToast('数据路径已更新', 'success')
        }
      } catch (error) {
        console.error('选择路径失败:', error)
        showToast('选择路径失败', 'error')
      }
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('设置已保存', 'success')
      } catch (error) {
        console.error('保存设置失败:', error)
        showToast('保存设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认设置吗？这将重置所有应用设置。')) {
        settingsStore.resetSettings('app')
        loadSettings()
        showToast('已恢复默认设置', 'success')
      }
    }
    
    return {
      localSettings,
      saving,
      updateSettings,
      selectDataPath,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.app-settings {
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

/* 输入控件样式 */
.setting-select,
.setting-input {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
  min-width: 150px;
}

.setting-select:focus,
.setting-input:focus {
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

/* 输入组样式 */
.path-input-group,
.number-input-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.path-input-group .setting-input {
  flex: 1;
  min-width: 200px;
}

.number-input-group .setting-input {
  width: 80px;
  text-align: center;
}

.input-unit {
  color: var(--text-secondary);
  font-size: 0.85rem;
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
  .app-settings {
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
  
  .path-input-group {
    width: 100%;
  }
  
  .settings-actions {
    flex-direction: column;
  }
}
</style>