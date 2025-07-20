<template>
  <div class="download-settings">
    <div class="settings-header">
      <h2>下载设置</h2>
      <p>配置图书下载的行为和存储选项</p>
    </div>
    
    <div class="settings-sections">
      <!-- 下载路径 -->
      <div class="setting-section">
        <h3>存储路径</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">默认下载路径</span>
            <span class="label-desc">下载的图书文件保存位置</span>
          </label>
          <div class="path-input-group">
            <input 
              type="text" 
              v-model="localSettings.downloadPath" 
              class="setting-input"
              readonly
            >
            <button class="btn-secondary" @click="selectDownloadPath">
              选择路径
            </button>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">按作者分类存储</span>
            <span class="label-desc">在下载路径下按作者名创建子文件夹</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.organizeByAuthor"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">按日期分类存储</span>
            <span class="label-desc">在下载路径下按下载日期创建子文件夹</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.organizeByDate"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">文件名格式</span>
            <span class="label-desc">下载文件的命名格式</span>
          </label>
          <select 
            v-model="localSettings.filenameFormat" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="title">书名</option>
            <option value="title-author">书名-作者</option>
            <option value="author-title">作者-书名</option>
            <option value="title-source">书名-来源</option>
            <option value="custom">自定义格式</option>
          </select>
        </div>
        
        <div class="setting-item" v-if="localSettings.filenameFormat === 'custom'">
          <label class="setting-label">
            <span class="label-text">自定义文件名格式</span>
            <span class="label-desc">使用 {title}, {author}, {source} 等变量</span>
          </label>
          <input 
            type="text" 
            v-model="localSettings.customFilenameFormat" 
            class="setting-input"
            placeholder="{title}-{author}"
            @input="updateSettings"
          >
        </div>
      </div>
      
      <!-- 下载行为 -->
      <div class="setting-section">
        <h3>下载行为</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">并发下载数</span>
            <span class="label-desc">同时下载的章节数量</span>
          </label>
          <div class="concurrent-control">
            <input 
              type="range" 
              v-model.number="localSettings.concurrent" 
              min="1"
              max="10"
              step="1"
              class="concurrent-slider"
              @input="updateSettings"
            >
            <span class="concurrent-value">{{ localSettings.concurrent }}个</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载超时时间</span>
            <span class="label-desc">单个章节的下载超时时间（秒）</span>
          </label>
          <div class="timeout-control">
            <input 
              type="range" 
              v-model.number="localSettings.timeout" 
              min="10"
              max="120"
              step="10"
              class="timeout-slider"
              @input="updateSettings"
            >
            <span class="timeout-value">{{ Math.round(localSettings.timeout / 1000) }}秒</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">重试次数</span>
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
            <span class="label-text">下载间隔</span>
            <span class="label-desc">章节下载之间的延迟时间（毫秒）</span>
          </label>
          <div class="delay-control">
            <input 
              type="range" 
              v-model.number="localSettings.downloadDelay" 
              min="0"
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
            <span class="label-text">自动导入到图书库</span>
            <span class="label-desc">下载完成后自动添加到图书库</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoImport"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载完成后通知</span>
            <span class="label-desc">显示系统通知提醒下载完成</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.showNotification"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 文件格式 -->
      <div class="setting-section">
        <h3>文件格式</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">默认保存格式</span>
            <span class="label-desc">下载图书的默认文件格式</span>
          </label>
          <select 
            v-model="localSettings.defaultFormat" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="txt">TXT 文本文件</option>
            <option value="epub">EPUB 电子书</option>
            <option value="both">同时保存两种格式</option>
          </select>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">TXT 文件编码</span>
            <span class="label-desc">TXT 文件的字符编码格式</span>
          </label>
          <select 
            v-model="localSettings.txtEncoding" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="utf-8">UTF-8</option>
            <option value="gbk">GBK</option>
            <option value="gb2312">GB2312</option>
            <option value="big5">Big5</option>
          </select>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">章节分隔符</span>
            <span class="label-desc">TXT 文件中章节之间的分隔符</span>
          </label>
          <select 
            v-model="localSettings.chapterSeparator" 
            class="setting-select"
            @change="updateSettings"
          >
            <option value="double-line">双换行</option>
            <option value="line-break">分隔线</option>
            <option value="page-break">分页符</option>
            <option value="custom">自定义</option>
          </select>
        </div>
        
        <div class="setting-item" v-if="localSettings.chapterSeparator === 'custom'">
          <label class="setting-label">
            <span class="label-text">自定义分隔符</span>
            <span class="label-desc">输入自定义的章节分隔符</span>
          </label>
          <input 
            type="text" 
            v-model="localSettings.customSeparator" 
            class="setting-input"
            placeholder="例如: ====="
            @input="updateSettings"
          >
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">包含章节标题</span>
            <span class="label-desc">在文件内容中包含章节标题</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.includeChapterTitle"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
      </div>
      
      <!-- 下载队列 -->
      <div class="setting-section">
        <h3>下载队列</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">最大队列长度</span>
            <span class="label-desc">同时排队等待下载的图书数量</span>
          </label>
          <div class="queue-length-control">
            <input 
              type="range" 
              v-model.number="localSettings.maxQueueLength" 
              min="1"
              max="20"
              step="1"
              class="queue-length-slider"
              @input="updateSettings"
            >
            <span class="queue-length-value">{{ localSettings.maxQueueLength }}本</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">自动开始下载</span>
            <span class="label-desc">添加到队列后自动开始下载</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoStartDownload"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">下载完成后自动开始下一个</span>
            <span class="label-desc">当前下载完成后自动开始队列中的下一个</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.autoStartNext"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">保留下载历史</span>
            <span class="label-desc">保留已完成下载的历史记录</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.keepDownloadHistory"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item" v-if="localSettings.keepDownloadHistory">
          <label class="setting-label">
            <span class="label-text">历史记录保留天数</span>
            <span class="label-desc">自动清理多少天前的下载历史</span>
          </label>
          <select 
            v-model.number="localSettings.historyRetentionDays" 
            class="setting-select"
            @change="updateSettings"
          >
            <option :value="7">7天</option>
            <option :value="30">30天</option>
            <option :value="90">90天</option>
            <option :value="365">1年</option>
            <option :value="0">永久保留</option>
          </select>
        </div>
      </div>
    </div>
    
    <!-- 下载统计 -->
    <div class="stats-section">
      <h3>下载统计</h3>
      <div class="stats-grid">
        <div class="stat-item">
          <div class="stat-value">{{ downloadStats.totalDownloads }}</div>
          <div class="stat-label">总下载数</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ downloadStats.successfulDownloads }}</div>
          <div class="stat-label">成功下载</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ downloadStats.failedDownloads }}</div>
          <div class="stat-label">失败下载</div>
        </div>
        <div class="stat-item">
          <div class="stat-value">{{ formatFileSize(downloadStats.totalSize) }}</div>
          <div class="stat-label">总下载大小</div>
        </div>
      </div>
      <div class="stats-actions">
        <button class="btn-secondary" @click="clearDownloadHistory">
          清空下载历史
        </button>
        <button class="btn-secondary" @click="exportDownloadStats">
          导出统计数据
        </button>
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
import { open } from '@tauri-apps/plugin-dialog'

export default {
  name: 'DownloadSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    
    // 本地设置状态
    const localSettings = reactive({
      downloadPath: './downloads',
      organizeByAuthor: false,
      organizeByDate: false,
      filenameFormat: 'title-author',
      customFilenameFormat: '{title}-{author}',
      concurrent: 3,
      timeout: 30000,
      retryCount: 3,
      downloadDelay: 1000,
      autoImport: true,
      showNotification: true,
      defaultFormat: 'txt',
      txtEncoding: 'utf-8',
      chapterSeparator: 'double-line',
      customSeparator: '=====',
      includeChapterTitle: true,
      maxQueueLength: 5,
      autoStartDownload: true,
      autoStartNext: true,
      keepDownloadHistory: true,
      historyRetentionDays: 30
    })
    
    // 下载统计数据
    const downloadStats = reactive({
      totalDownloads: 0,
      successfulDownloads: 0,
      failedDownloads: 0,
      totalSize: 0
    })
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
      loadDownloadStats()
    })
    
    // 加载设置
    const loadSettings = () => {
      const downloadSettings = settingsStore.download
      Object.keys(localSettings).forEach(key => {
        if (downloadSettings[key] !== undefined) {
          localSettings[key] = downloadSettings[key]
        }
      })
    }
    
    // 加载下载统计
    const loadDownloadStats = async () => {
      try {
        // 这里应该调用后端API获取实际统计数据
        // const stats = await invoke('get_download_stats')
        // Object.assign(downloadStats, stats)
        
        // 模拟数据
        Object.assign(downloadStats, {
          totalDownloads: 25,
          successfulDownloads: 23,
          failedDownloads: 2,
          totalSize: 1024 * 1024 * 150 // 150MB
        })
      } catch (error) {
        console.error('获取下载统计失败:', error)
      }
    }
    
    // 更新设置
    const updateSettings = () => {
      settingsStore.updateDownloadSettings(localSettings)
    }
    
    // 选择下载路径
    const selectDownloadPath = async () => {
      try {
        const selected = await open({
          directory: true,
          title: '选择下载路径'
        })
        
        if (selected) {
          localSettings.downloadPath = selected
          updateSettings()
          showToast('下载路径已更新', 'success')
        }
      } catch (error) {
        console.error('选择下载路径失败:', error)
        showToast('选择下载路径失败', 'error')
      }
    }
    
    // 格式化文件大小
    const formatFileSize = (bytes) => {
      if (bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    }
    
    // 清空下载历史
    const clearDownloadHistory = async () => {
      if (confirm('确定要清空所有下载历史记录吗？此操作不可恢复。')) {
        try {
          // 这里应该调用后端API清空下载历史
          // await invoke('clear_download_history')
          Object.assign(downloadStats, {
            totalDownloads: 0,
            successfulDownloads: 0,
            failedDownloads: 0,
            totalSize: 0
          })
          showToast('下载历史已清空', 'success')
        } catch (error) {
          console.error('清空下载历史失败:', error)
          showToast('清空下载历史失败', 'error')
        }
      }
    }
    
    // 导出统计数据
    const exportDownloadStats = async () => {
      try {
        const statsData = {
          ...downloadStats,
          exportTime: new Date().toISOString(),
          settings: localSettings
        }
        
        const dataStr = JSON.stringify(statsData, null, 2)
        const blob = new Blob([dataStr], { type: 'application/json' })
        const url = URL.createObjectURL(blob)
        
        const a = document.createElement('a')
        a.href = url
        a.download = `download-stats-${new Date().toISOString().split('T')[0]}.json`
        document.body.appendChild(a)
        a.click()
        document.body.removeChild(a)
        URL.revokeObjectURL(url)
        
        showToast('统计数据已导出', 'success')
      } catch (error) {
        console.error('导出统计数据失败:', error)
        showToast('导出统计数据失败', 'error')
      }
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('下载设置已保存', 'success')
      } catch (error) {
        console.error('保存下载设置失败:', error)
        showToast('保存下载设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认下载设置吗？')) {
        Object.assign(localSettings, {
          downloadPath: './downloads',
          organizeByAuthor: false,
          organizeByDate: false,
          filenameFormat: 'title-author',
          customFilenameFormat: '{title}-{author}',
          concurrent: 3,
          timeout: 30000,
          retryCount: 3,
          downloadDelay: 1000,
          autoImport: true,
          showNotification: true,
          defaultFormat: 'txt',
          txtEncoding: 'utf-8',
          chapterSeparator: 'double-line',
          customSeparator: '=====',
          includeChapterTitle: true,
          maxQueueLength: 5,
          autoStartDownload: true,
          autoStartNext: true,
          keepDownloadHistory: true,
          historyRetentionDays: 30
        })
        updateSettings()
        showToast('已恢复默认下载设置', 'success')
      }
    }
    
    return {
      localSettings,
      downloadStats,
      saving,
      updateSettings,
      selectDownloadPath,
      formatFileSize,
      clearDownloadHistory,
      exportDownloadStats,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.download-settings {
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

/* 路径输入组 */
.path-input-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.path-input-group .setting-input {
  flex: 1;
  min-width: 200px;
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
.concurrent-control,
.timeout-control,
.delay-control,
.queue-length-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.concurrent-slider,
.timeout-slider,
.delay-slider,
.queue-length-slider {
  width: 120px;
}

.concurrent-value,
.timeout-value,
.delay-value,
.queue-length-value {
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

/* 统计区域 */
.stats-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.stats-section h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.stat-item {
  text-align: center;
  padding: 1rem;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--accent-color);
  margin-bottom: 0.25rem;
}

.stat-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.stats-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
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
  .download-settings {
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
    flex-direction: column;
  }
  
  .concurrent-control,
  .timeout-control,
  .delay-control,
  .queue-length-control {
    width: 100%;
    justify-content: space-between;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .stats-actions {
    flex-direction: column;
  }
  
  .settings-actions {
    flex-direction: column;
  }
}
</style>