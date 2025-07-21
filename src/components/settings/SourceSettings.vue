<template>
  <div class="source-settings">
    <div class="settings-header">
      <h2>书源管理</h2>
      <p>配置在线书源的搜索和下载设置</p>
    </div>
    
    <div class="settings-sections">
      <!-- 书源功能 -->
      <div class="setting-section">
        <h3>书源功能</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用书源功能</span>
            <span class="label-desc">开启在线搜索和下载功能</span>
          </label>
          <BaseSwitch
            v-model="localSettings.enabled"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">自动更新书源</span>
            <span class="label-desc">定期检查并更新书源插件</span>
          </label>
          <BaseSwitch
            v-model="localSettings.autoUpdate"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item" v-if="localSettings.autoUpdate">
          <label class="setting-label">
            <span class="label-text">更新检查间隔</span>
            <span class="label-desc">自动检查更新的时间间隔</span>
          </label>
          <select 
            v-model="localSettings.updateInterval" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="daily">每天</option>
            <option value="weekly">每周</option>
            <option value="monthly">每月</option>
            <option value="manual">手动</option>
          </select>
        </div>
      </div>
      
      <!-- 搜索设置 -->
      <div class="setting-section">
        <h3>搜索设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">搜索超时时间</span>
            <span class="label-desc">单个书源的搜索超时时间（秒）</span>
          </label>
          <div class="timeout-control">
            <input 
              type="range" 
              v-model.number="localSettings.searchTimeout" 
              min="5"
              max="60"
              step="5"
              class="timeout-slider"
              @input="updateSettings"
            >
            <span class="timeout-value">{{ localSettings.searchTimeout }}秒</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">并发搜索数量</span>
            <span class="label-desc">同时搜索的书源数量</span>
          </label>
          <div class="concurrent-control">
            <input 
              type="range" 
              v-model.number="localSettings.searchConcurrent" 
              min="1"
              max="10"
              step="1"
              class="concurrent-slider"
              @input="updateSettings"
            >
            <span class="concurrent-value">{{ localSettings.searchConcurrent }}个</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">搜索结果去重</span>
            <span class="label-desc">自动去除重复的搜索结果</span>
          </label>
          <BaseSwitch
            v-model="localSettings.deduplicateResults"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">最大搜索结果</span>
            <span class="label-desc">每个书源返回的最大结果数量</span>
          </label>
          <div class="max-results-control">
            <input 
              type="range" 
              v-model.number="localSettings.maxResults" 
              min="10"
              max="100"
              step="10"
              class="results-slider"
              @input="updateSettings"
            >
            <span class="results-value">{{ localSettings.maxResults }}条</span>
          </div>
        </div>
      </div>
      
      <!-- 下载设置 -->
      <div class="setting-section">
        <h3>下载设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载并发数</span>
            <span class="label-desc">同时下载的章节数量</span>
          </label>
          <div class="download-concurrent-control">
            <input 
              type="range" 
              v-model.number="localSettings.downloadConcurrent" 
              min="1"
              max="10"
              step="1"
              class="download-concurrent-slider"
              @input="updateSettings"
            >
            <span class="download-concurrent-value">{{ localSettings.downloadConcurrent }}个</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载间隔</span>
            <span class="label-desc">章节下载之间的延迟时间（毫秒）</span>
          </label>
          <div class="download-delay-control">
            <input 
              type="range" 
              v-model.number="localSettings.downloadDelay" 
              min="100"
              max="5000"
              step="100"
              class="delay-slider"
              @input="updateSettings"
            >
            <span class="delay-value">{{ localSettings.downloadDelay }}ms</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载重试次数</span>
            <span class="label-desc">下载失败时的重试次数</span>
          </label>
          <select 
            v-model.number="localSettings.retryCount" 
            class="setting-select"
            @change="updateSettings"
          >
            <option :value="0">不重试</option>
            <option :value="1">重试1次</option>
            <option :value="2">重试2次</option>
            <option :value="3">重试3次</option>
            <option :value="5">重试5次</option>
          </select>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">自动导入下载</span>
            <span class="label-desc">下载完成后自动导入到图书库</span>
          </label>
          <BaseSwitch
            v-model="localSettings.autoImport"
            @change="updateSettings"
          />
        </div>
      </div>
      
      <!-- 网络设置 -->
      <div class="setting-section">
        <h3>网络设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">用户代理</span>
            <span class="label-desc">HTTP请求的User-Agent字符串</span>
          </label>
          <select 
            v-model="localSettings.userAgent" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="default">默认 (NovelNest)</option>
            <option value="chrome">Chrome 浏览器</option>
            <option value="firefox">Firefox 浏览器</option>
            <option value="safari">Safari 浏览器</option>
            <option value="mobile">移动设备</option>
            <option value="custom">自定义</option>
          </select>
        </div>
        
        <div class="setting-item" v-if="localSettings.userAgent === 'custom'">
          <label class="setting-label">
            <span class="label-text">自定义User-Agent</span>
            <span class="label-desc">输入自定义的User-Agent字符串</span>
          </label>
          <BaseInput
            v-model="localSettings.customUserAgent" 
            placeholder="输入User-Agent..."
            @input="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用代理</span>
            <span class="label-desc">通过代理服务器访问书源</span>
          </label>
          <BaseSwitch
            v-model="localSettings.useProxy"
            @change="updateSettings"
          />
        </div>
        
        <div v-if="localSettings.useProxy" class="proxy-settings">
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">代理类型</span>
              <span class="label-desc">选择代理服务器类型</span>
            </label>
            <select 
              v-model="localSettings.proxyType" 
              class="setting-select"
              @change="updateSettings"
            >
              <option value="http">HTTP</option>
              <option value="https">HTTPS</option>
              <option value="socks5">SOCKS5</option>
            </select>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">代理地址</span>
              <span class="label-desc">代理服务器的IP地址或域名</span>
            </label>
            <BaseInput
              v-model="localSettings.proxyHost" 
              placeholder="127.0.0.1"
              @input="updateSettings"
            />
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">代理端口</span>
              <span class="label-desc">代理服务器的端口号</span>
            </label>
            <BaseInput
              v-model.number="localSettings.proxyPort" 
              type="number"
              placeholder="8080"
              :min="1"
              :max="65535"
              @input="updateSettings"
            />
          </div>
        </div>
      </div>
      
      <!-- 缓存设置 -->
      <div class="setting-section">
        <h3>缓存设置</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用搜索缓存</span>
            <span class="label-desc">缓存搜索结果以提高响应速度</span>
          </label>
          <BaseSwitch
            v-model="localSettings.enableCache"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item" v-if="localSettings.enableCache">
          <label class="setting-label">
            <span class="label-text">缓存过期时间</span>
            <span class="label-desc">搜索结果的缓存有效期</span>
          </label>
          <select 
            v-model="localSettings.cacheExpiry" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="5">5分钟</option>
            <option value="15">15分钟</option>
            <option value="30">30分钟</option>
            <option value="60">1小时</option>
            <option value="360">6小时</option>
            <option value="1440">24小时</option>
          </select>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">最大缓存大小</span>
            <span class="label-desc">缓存数据的最大占用空间（MB）</span>
          </label>
          <div class="cache-size-control">
            <input 
              type="range" 
              v-model.number="localSettings.maxCacheSize" 
              min="10"
              max="500"
              step="10"
              class="cache-size-slider"
              @input="updateSettings"
            >
            <span class="cache-size-value">{{ localSettings.maxCacheSize }}MB</span>
          </div>
        </div>
        
        <div class="setting-item">
          <div class="cache-actions">
            <BaseButton @click="clearCache" variant="secondary">
              清空缓存
            </BaseButton>
            <BaseBadge variant="info" size="small">
              当前缓存: {{ cacheSize }}MB
            </BaseBadge>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <div class="settings-actions">
      <BaseButton @click="resetToDefaults" variant="secondary">
        恢复默认设置
      </BaseButton>
      <BaseButton @click="saveSettings" variant="primary" :loading="saving">
        保存设置
      </BaseButton>
    </div>
  </div>
</template>

<script>
import { ref, reactive, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'

export default {
  name: 'SourceSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    const cacheSize = ref(0)
    
    // 本地设置状态
    const localSettings = reactive({
      enabled: true,
      autoUpdate: false,
      updateInterval: 'weekly',
      searchTimeout: 30,
      searchConcurrent: 3,
      deduplicateResults: true,
      maxResults: 50,
      downloadConcurrent: 3,
      downloadDelay: 1000,
      retryCount: 3,
      autoImport: true,
      userAgent: 'default',
      customUserAgent: '',
      useProxy: false,
      proxyType: 'http',
      proxyHost: '',
      proxyPort: 8080,
      enableCache: true,
      cacheExpiry: 30,
      maxCacheSize: 100
    })
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
      loadCacheSize()
    })
    
    // 加载设置
    const loadSettings = () => {
      const bookSourceSettings = settingsStore.bookSources
      const downloadSettings = settingsStore.download
      
      Object.keys(localSettings).forEach(key => {
        if (bookSourceSettings[key] !== undefined) {
          localSettings[key] = bookSourceSettings[key]
        } else if (downloadSettings[key] !== undefined) {
          localSettings[key] = downloadSettings[key]
        }
      })
    }
    
    // 加载缓存大小
    const loadCacheSize = async () => {
      try {
        // 这里应该调用后端API获取实际缓存大小
        // const size = await invoke('get_cache_size')
        // cacheSize.value = size
        cacheSize.value = 25 // 模拟数据
      } catch (error) {
        console.error('获取缓存大小失败:', error)
      }
    }
    
    // 更新设置
    const updateSettings = () => {
      // 分别更新书源设置和下载设置
      const bookSourceSettings = {
        enabled: localSettings.enabled,
        autoUpdate: localSettings.autoUpdate,
        updateInterval: localSettings.updateInterval,
        searchTimeout: localSettings.searchTimeout,
        searchConcurrent: localSettings.searchConcurrent,
        deduplicateResults: localSettings.deduplicateResults,
        maxResults: localSettings.maxResults,
        userAgent: localSettings.userAgent,
        customUserAgent: localSettings.customUserAgent,
        useProxy: localSettings.useProxy,
        proxyType: localSettings.proxyType,
        proxyHost: localSettings.proxyHost,
        proxyPort: localSettings.proxyPort,
        enableCache: localSettings.enableCache,
        cacheExpiry: localSettings.cacheExpiry,
        maxCacheSize: localSettings.maxCacheSize
      }
      
      const downloadSettings = {
        concurrent: localSettings.downloadConcurrent,
        timeout: localSettings.searchTimeout * 1000,
        retryCount: localSettings.retryCount,
        autoImport: localSettings.autoImport,
        downloadDelay: localSettings.downloadDelay
      }
      
      settingsStore.updateBookSourceSettings(bookSourceSettings)
      settingsStore.updateDownloadSettings(downloadSettings)
    }
    
    // 清空缓存
    const clearCache = async () => {
      if (confirm('确定要清空所有缓存数据吗？这将删除所有搜索结果缓存。')) {
        try {
          // 这里应该调用后端API清空缓存
          // await invoke('clear_cache')
          cacheSize.value = 0
          showToast('缓存已清空', 'success')
        } catch (error) {
          console.error('清空缓存失败:', error)
          showToast('清空缓存失败', 'error')
        }
      }
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('书源设置已保存', 'success')
      } catch (error) {
        console.error('保存书源设置失败:', error)
        showToast('保存书源设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认书源设置吗？')) {
        Object.assign(localSettings, {
          enabled: true,
          autoUpdate: false,
          updateInterval: 'weekly',
          searchTimeout: 30,
          searchConcurrent: 3,
          deduplicateResults: true,
          maxResults: 50,
          downloadConcurrent: 3,
          downloadDelay: 1000,
          retryCount: 3,
          autoImport: true,
          userAgent: 'default',
          customUserAgent: '',
          useProxy: false,
          proxyType: 'http',
          proxyHost: '',
          proxyPort: 8080,
          enableCache: true,
          cacheExpiry: 30,
          maxCacheSize: 100
        })
        updateSettings()
        showToast('已恢复默认书源设置', 'success')
      }
    }
    
    return {
      localSettings,
      saving,
      cacheSize,
      updateSettings,
      clearCache,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.source-settings {
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

/* 代理设置区域 */
.proxy-settings {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
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
  min-width: 120px;
}

.setting-select:focus,
.setting-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

/* 滑块控制 */
.timeout-control,
.concurrent-control,
.download-concurrent-control,
.download-delay-control,
.max-results-control,
.cache-size-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.timeout-slider,
.concurrent-slider,
.download-concurrent-slider,
.delay-slider,
.results-slider,
.cache-size-slider {
  width: 120px;
}

.timeout-value,
.concurrent-value,
.download-concurrent-value,
.delay-value,
.results-value,
.cache-size-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 50px;
  text-align: center;
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

/* 缓存操作 */
.cache-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
  width: 100%;
}

.cache-info {
  color: var(--text-secondary);
  font-size: 0.9rem;
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
  .source-settings {
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
  
  .timeout-control,
  .concurrent-control,
  .download-concurrent-control,
  .download-delay-control,
  .max-results-control,
  .cache-size-control {
    width: 100%;
    justify-content: space-between;
  }
  
  .cache-actions {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .settings-actions {
    flex-direction: column;
  }
}
</style>