<template>
  <div class="data-management">
    <div class="settings-header">
      <h2>数据管理</h2>
      <p>管理应用数据的备份、恢复和清理</p>
    </div>
    
    <div class="settings-sections">
      <!-- 数据备份 -->
      <div class="setting-section">
        <h3>数据备份</h3>
        <p class="section-desc">备份您的图书库、阅读进度、书签和应用设置</p>
        
        <div class="backup-options">
          <div class="backup-item">
            <div class="backup-info">
              <h4>完整备份</h4>
              <p>包含所有数据：图书库、设置、阅读进度、书签等</p>
            </div>
            <button class="btn-primary" @click="createFullBackup" :disabled="backing">
              {{ backing ? '备份中...' : '创建完整备份' }}
            </button>
          </div>
          
          <div class="backup-item">
            <div class="backup-info">
              <h4>图书库备份</h4>
              <p>仅备份图书库索引和元数据信息</p>
            </div>
            <button class="btn-secondary" @click="createLibraryBackup" :disabled="backing">
              {{ backing ? '备份中...' : '备份图书库' }}
            </button>
          </div>
          
          <div class="backup-item">
            <div class="backup-info">
              <h4>设置备份</h4>
              <p>仅备份应用设置和配置信息</p>
            </div>
            <button class="btn-secondary" @click="createSettingsBackup" :disabled="backing">
              {{ backing ? '备份中...' : '备份设置' }}
            </button>
          </div>
          
          <div class="backup-item">
            <div class="backup-info">
              <h4>阅读数据备份</h4>
              <p>备份阅读进度、书签和阅读历史</p>
            </div>
            <button class="btn-secondary" @click="createReadingDataBackup" :disabled="backing">
              {{ backing ? '备份中...' : '备份阅读数据' }}
            </button>
          </div>
        </div>
        
        <div class="backup-settings">
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">自动备份</span>
              <span class="label-desc">定期自动创建数据备份</span>
            </label>
            <label class="setting-switch">
              <input 
                type="checkbox" 
                v-model="localSettings.autoBackup"
                @change="updateSettings"
              >
              <span class="switch-slider"></span>
            </label>
          </div>
          
          <div class="setting-item" v-if="localSettings.autoBackup">
            <label class="setting-label">
              <span class="label-text">备份频率</span>
              <span class="label-desc">自动备份的时间间隔</span>
            </label>
            <BaseSelect
              v-model="localSettings.backupFrequency"
              :options="backupFrequencyOptions"
              size="small"
              @change="updateSettings"
            />
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">备份保留数量</span>
              <span class="label-desc">保留的备份文件数量</span>
            </label>
            <div class="backup-count-control">
              <input 
                type="range" 
                v-model.number="localSettings.maxBackupCount" 
                min="1"
                max="20"
                step="1"
                class="backup-count-slider"
                @input="updateSettings"
              >
              <span class="backup-count-value">{{ localSettings.maxBackupCount }}个</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 数据恢复 -->
      <div class="setting-section">
        <h3>数据恢复</h3>
        <p class="section-desc">从备份文件恢复您的数据</p>
        
        <div class="restore-options">
          <div class="restore-item">
            <div class="restore-info">
              <h4>从备份文件恢复</h4>
              <p>选择备份文件并恢复数据</p>
            </div>
            <button class="btn-primary" @click="selectBackupFile" :disabled="restoring">
              {{ restoring ? '恢复中...' : '选择备份文件' }}
            </button>
          </div>
          
          <div class="restore-item">
            <div class="restore-info">
              <h4>导入图书库</h4>
              <p>从JSON文件导入图书库数据</p>
            </div>
            <button class="btn-secondary" @click="importLibrary" :disabled="restoring">
              {{ restoring ? '导入中...' : '导入图书库' }}
            </button>
          </div>
          
          <div class="restore-item">
            <div class="restore-info">
              <h4>导入设置</h4>
              <p>从配置文件导入应用设置</p>
            </div>
            <button class="btn-secondary" @click="importSettings" :disabled="restoring">
              {{ restoring ? '导入中...' : '导入设置' }}
            </button>
          </div>
        </div>
        
        <div class="restore-warning">
          <div class="warning-icon">⚠️</div>
          <div class="warning-text">
            <strong>注意：</strong>恢复数据将覆盖当前的数据，请确保您已经备份了重要数据。
          </div>
        </div>
      </div>
      
      <!-- 备份历史 -->
      <div class="setting-section">
        <h3>备份历史</h3>
        <p class="section-desc">查看和管理已创建的备份文件</p>
        
        <div class="backup-list">
          <div v-if="backupHistory.length === 0" class="empty-backups">
            <div class="empty-icon">📦</div>
            <p>暂无备份文件</p>
          </div>
          
          <div v-else class="backup-history">
            <div 
              v-for="backup in backupHistory" 
              :key="backup.id"
              class="backup-record"
            >
              <div class="backup-details">
                <div class="backup-name">{{ backup.name }}</div>
                <div class="backup-meta">
                  <span class="backup-type">{{ getBackupTypeLabel(backup.type) }}</span>
                  <span class="backup-date">{{ formatDate(backup.createdAt) }}</span>
                  <span class="backup-size">{{ formatFileSize(backup.size) }}</span>
                </div>
              </div>
              <div class="backup-actions">
                <button class="btn-small btn-secondary" @click="restoreFromBackup(backup)">
                  恢复
                </button>
                <button class="btn-small btn-secondary" @click="exportBackup(backup)">
                  导出
                </button>
                <button class="btn-small btn-danger" @click="deleteBackup(backup)">
                  删除
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div class="backup-actions">
          <button class="btn-secondary" @click="refreshBackupHistory">
            刷新列表
          </button>
          <button class="btn-secondary" @click="cleanupOldBackups">
            清理旧备份
          </button>
        </div>
      </div>
      
      <!-- 数据清理 -->
      <div class="setting-section">
        <h3>数据清理</h3>
        <p class="section-desc">清理不需要的数据以释放存储空间</p>
        
        <div class="cleanup-options">
          <div class="cleanup-item">
            <div class="cleanup-info">
              <h4>清理缓存数据</h4>
              <p>清除搜索缓存、图片缓存等临时数据</p>
              <span class="cleanup-size">约 {{ formatFileSize(cacheSize) }}</span>
            </div>
            <button class="btn-secondary" @click="clearCache" :disabled="cleaning">
              {{ cleaning ? '清理中...' : '清理缓存' }}
            </button>
          </div>
          
          <div class="cleanup-item">
            <div class="cleanup-info">
              <h4>清理日志文件</h4>
              <p>删除旧的应用日志文件</p>
              <span class="cleanup-size">约 {{ formatFileSize(logSize) }}</span>
            </div>
            <button class="btn-secondary" @click="clearLogs" :disabled="cleaning">
              {{ cleaning ? '清理中...' : '清理日志' }}
            </button>
          </div>
          
          <div class="cleanup-item">
            <div class="cleanup-info">
              <h4>清理下载历史</h4>
              <p>删除已完成的下载记录</p>
              <span class="cleanup-size">{{ downloadHistoryCount }} 条记录</span>
            </div>
            <button class="btn-secondary" @click="clearDownloadHistory" :disabled="cleaning">
              {{ cleaning ? '清理中...' : '清理历史' }}
            </button>
          </div>
          
          <div class="cleanup-item">
            <div class="cleanup-info">
              <h4>清理阅读历史</h4>
              <p>删除过期的阅读历史记录</p>
              <span class="cleanup-size">{{ readingHistoryCount }} 条记录</span>
            </div>
            <button class="btn-secondary" @click="clearReadingHistory" :disabled="cleaning">
              {{ cleaning ? '清理中...' : '清理历史' }}
            </button>
          </div>
        </div>
        
        <div class="cleanup-settings">
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">自动清理</span>
              <span class="label-desc">定期自动清理临时数据</span>
            </label>
            <label class="setting-switch">
              <input 
                type="checkbox" 
                v-model="localSettings.autoCleanup"
                @change="updateSettings"
              >
              <span class="switch-slider"></span>
            </label>
          </div>
          
          <div class="setting-item" v-if="localSettings.autoCleanup">
            <label class="setting-label">
              <span class="label-text">清理频率</span>
              <span class="label-desc">自动清理的时间间隔</span>
            </label>
            <select 
              v-model="localSettings.cleanupFrequency" 
              class="setting-select"
              @change="updateSettings"
            >
              <option value="daily">每天</option>
              <option value="weekly">每周</option>
              <option value="monthly">每月</option>
            </select>
          </div>
        </div>
      </div>
      
      <!-- 数据重置 -->
      <div class="setting-section danger-section">
        <h3>数据重置</h3>
        <p class="section-desc">重置应用数据到初始状态</p>
        
        <div class="reset-options">
          <div class="reset-item">
            <div class="reset-info">
              <h4>重置应用设置</h4>
              <p>将所有设置恢复到默认值</p>
            </div>
            <button class="btn-danger" @click="resetSettings">
              重置设置
            </button>
          </div>
          
          <div class="reset-item">
            <div class="reset-info">
              <h4>清空图书库</h4>
              <p>删除所有图书记录（不删除文件）</p>
            </div>
            <button class="btn-danger" @click="clearLibrary">
              清空图书库
            </button>
          </div>
          
          <div class="reset-item">
            <div class="reset-info">
              <h4>完全重置</h4>
              <p>删除所有数据，恢复到初始安装状态</p>
            </div>
            <button class="btn-danger" @click="fullReset">
              完全重置
            </button>
          </div>
        </div>
        
        <div class="reset-warning">
          <div class="warning-icon">🚨</div>
          <div class="warning-text">
            <strong>危险操作：</strong>重置操作不可恢复，请在操作前确保已备份重要数据。
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'
import { open, save } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'DataManagement',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    
    const backing = ref(false)
    const restoring = ref(false)
    const cleaning = ref(false)
    
    // 备份频率选项
    const backupFrequencyOptions = [
      { label: '每天', value: 'daily' },
      { label: '每周', value: 'weekly' },
      { label: '每月', value: 'monthly' }
    ]
    
    // 本地设置状态
    const localSettings = reactive({
      autoBackup: false,
      backupFrequency: 'weekly',
      maxBackupCount: 5,
      autoCleanup: true,
      cleanupFrequency: 'weekly'
    })
    
    // 备份历史
    const backupHistory = ref([])
    
    // 数据大小统计
    const cacheSize = ref(0)
    const logSize = ref(0)
    const downloadHistoryCount = ref(0)
    const readingHistoryCount = ref(0)
    
    // 初始化
    onMounted(() => {
      loadSettings()
      loadBackupHistory()
      loadDataSizes()
    })
    
    // 加载设置
    const loadSettings = () => {
      // 从设置存储中加载数据管理相关设置
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
    
    // 加载备份历史
    const loadBackupHistory = async () => {
      try {
        // 这里应该调用后端API获取备份历史
        // const history = await invoke('get_backup_history')
        // backupHistory.value = history
        
        // 模拟数据
        backupHistory.value = [
          {
            id: '1',
            name: '完整备份_2024-01-15',
            type: 'full',
            createdAt: new Date('2024-01-15T10:30:00'),
            size: 1024 * 1024 * 25 // 25MB
          },
          {
            id: '2',
            name: '图书库备份_2024-01-10',
            type: 'library',
            createdAt: new Date('2024-01-10T14:20:00'),
            size: 1024 * 1024 * 5 // 5MB
          }
        ]
      } catch (error) {
        console.error('加载备份历史失败:', error)
      }
    }
    
    // 加载数据大小
    const loadDataSizes = async () => {
      try {
        // 这里应该调用后端API获取实际数据大小
        // const sizes = await invoke('get_data_sizes')
        
        // 模拟数据
        cacheSize.value = 1024 * 1024 * 15 // 15MB
        logSize.value = 1024 * 1024 * 3 // 3MB
        downloadHistoryCount.value = 25
        readingHistoryCount.value = 150
      } catch (error) {
        console.error('加载数据大小失败:', error)
      }
    }
    
    // 创建完整备份
    const createFullBackup = async () => {
      backing.value = true
      try {
        const backupPath = await save({
          filters: [
            {
              name: '备份文件',
              extensions: ['backup', 'json']
            }
          ],
          defaultPath: `完整备份_${new Date().toISOString().split('T')[0]}.backup`
        })
        
        if (backupPath) {
          // 这里应该调用后端API创建备份
          // await invoke('create_full_backup', { path: backupPath })
          
          showToast('完整备份创建成功', 'success')
          await loadBackupHistory()
        }
      } catch (error) {
        console.error('创建完整备份失败:', error)
        showToast('创建完整备份失败', 'error')
      } finally {
        backing.value = false
      }
    }
    
    // 创建图书库备份
    const createLibraryBackup = async () => {
      backing.value = true
      try {
        const backupPath = await save({
          filters: [
            {
              name: 'JSON文件',
              extensions: ['json']
            }
          ],
          defaultPath: `图书库备份_${new Date().toISOString().split('T')[0]}.json`
        })
        
        if (backupPath) {
          // 这里应该调用后端API创建图书库备份
          // await invoke('create_library_backup', { path: backupPath })
          
          showToast('图书库备份创建成功', 'success')
          await loadBackupHistory()
        }
      } catch (error) {
        console.error('创建图书库备份失败:', error)
        showToast('创建图书库备份失败', 'error')
      } finally {
        backing.value = false
      }
    }
    
    // 创建设置备份
    const createSettingsBackup = async () => {
      backing.value = true
      try {
        const settingsData = settingsStore.exportSettings()
        const backupPath = await save({
          filters: [
            {
              name: 'JSON文件',
              extensions: ['json']
            }
          ],
          defaultPath: `设置备份_${new Date().toISOString().split('T')[0]}.json`
        })
        
        if (backupPath) {
          // 这里应该保存设置数据到文件
          // await invoke('save_file', { path: backupPath, content: settingsData })
          
          showToast('设置备份创建成功', 'success')
          await loadBackupHistory()
        }
      } catch (error) {
        console.error('创建设置备份失败:', error)
        showToast('创建设置备份失败', 'error')
      } finally {
        backing.value = false
      }
    }
    
    // 创建阅读数据备份
    const createReadingDataBackup = async () => {
      backing.value = true
      try {
        const backupPath = await save({
          filters: [
            {
              name: 'JSON文件',
              extensions: ['json']
            }
          ],
          defaultPath: `阅读数据备份_${new Date().toISOString().split('T')[0]}.json`
        })
        
        if (backupPath) {
          // 这里应该调用后端API创建阅读数据备份
          // await invoke('create_reading_data_backup', { path: backupPath })
          
          showToast('阅读数据备份创建成功', 'success')
          await loadBackupHistory()
        }
      } catch (error) {
        console.error('创建阅读数据备份失败:', error)
        showToast('创建阅读数据备份失败', 'error')
      } finally {
        backing.value = false
      }
    }
    
    // 选择备份文件恢复
    const selectBackupFile = async () => {
      try {
        const selected = await open({
          filters: [
            {
              name: '备份文件',
              extensions: ['backup', 'json']
            }
          ],
          title: '选择备份文件'
        })
        
        if (selected) {
          await restoreFromFile(selected)
        }
      } catch (error) {
        console.error('选择备份文件失败:', error)
        showToast('选择备份文件失败', 'error')
      }
    }
    
    // 从文件恢复
    const restoreFromFile = async (filePath) => {
      if (!confirm('确定要从备份文件恢复数据吗？这将覆盖当前的所有数据。')) {
        return
      }
      
      restoring.value = true
      try {
        // 这里应该调用后端API从文件恢复
        // await invoke('restore_from_backup', { path: filePath })
        
        showToast('数据恢复成功，请重启应用以生效', 'success')
      } catch (error) {
        console.error('数据恢复失败:', error)
        showToast('数据恢复失败', 'error')
      } finally {
        restoring.value = false
      }
    }
    
    // 导入图书库
    const importLibrary = async () => {
      try {
        const selected = await open({
          filters: [
            {
              name: 'JSON文件',
              extensions: ['json']
            }
          ],
          title: '选择图书库文件'
        })
        
        if (selected) {
          if (!confirm('确定要导入图书库数据吗？这将与现有数据合并。')) {
            return
          }
          
          restoring.value = true
          // 这里应该调用后端API导入图书库
          // await invoke('import_library', { path: selected })
          
          showToast('图书库导入成功', 'success')
          restoring.value = false
        }
      } catch (error) {
        console.error('导入图书库失败:', error)
        showToast('导入图书库失败', 'error')
        restoring.value = false
      }
    }
    
    // 导入设置
    const importSettings = async () => {
      try {
        const selected = await open({
          filters: [
            {
              name: 'JSON文件',
              extensions: ['json']
            }
          ],
          title: '选择设置文件'
        })
        
        if (selected) {
          if (!confirm('确定要导入设置吗？这将覆盖当前的设置。')) {
            return
          }
          
          restoring.value = true
          // 这里应该读取文件内容并导入设置
          // const content = await invoke('read_file', { path: selected })
          // await settingsStore.importSettings(content)
          
          showToast('设置导入成功', 'success')
          restoring.value = false
        }
      } catch (error) {
        console.error('导入设置失败:', error)
        showToast('导入设置失败', 'error')
        restoring.value = false
      }
    }
    
    // 从备份恢复
    const restoreFromBackup = async (backup) => {
      if (!confirm(`确定要从备份"${backup.name}"恢复数据吗？`)) {
        return
      }
      
      restoring.value = true
      try {
        // 这里应该调用后端API从备份恢复
        // await invoke('restore_from_backup_id', { backupId: backup.id })
        
        showToast('数据恢复成功', 'success')
      } catch (error) {
        console.error('数据恢复失败:', error)
        showToast('数据恢复失败', 'error')
      } finally {
        restoring.value = false
      }
    }
    
    // 导出备份
    const exportBackup = async (backup) => {
      try {
        const exportPath = await save({
          filters: [
            {
              name: '备份文件',
              extensions: ['backup', 'json']
            }
          ],
          defaultPath: backup.name
        })
        
        if (exportPath) {
          // 这里应该调用后端API导出备份
          // await invoke('export_backup', { backupId: backup.id, path: exportPath })
          
          showToast('备份导出成功', 'success')
        }
      } catch (error) {
        console.error('导出备份失败:', error)
        showToast('导出备份失败', 'error')
      }
    }
    
    // 删除备份
    const deleteBackup = async (backup) => {
      if (!confirm(`确定要删除备份"${backup.name}"吗？`)) {
        return
      }
      
      try {
        // 这里应该调用后端API删除备份
        // await invoke('delete_backup', { backupId: backup.id })
        
        await loadBackupHistory()
        showToast('备份已删除', 'success')
      } catch (error) {
        console.error('删除备份失败:', error)
        showToast('删除备份失败', 'error')
      }
    }
    
    // 刷新备份历史
    const refreshBackupHistory = async () => {
      await loadBackupHistory()
      showToast('备份列表已刷新', 'success')
    }
    
    // 清理旧备份
    const cleanupOldBackups = async () => {
      if (!confirm('确定要清理旧的备份文件吗？将保留最新的几个备份。')) {
        return
      }
      
      try {
        // 这里应该调用后端API清理旧备份
        // await invoke('cleanup_old_backups', { maxCount: localSettings.maxBackupCount })
        
        await loadBackupHistory()
        showToast('旧备份已清理', 'success')
      } catch (error) {
        console.error('清理旧备份失败:', error)
        showToast('清理旧备份失败', 'error')
      }
    }
    
    // 清理缓存
    const clearCache = async () => {
      if (!confirm('确定要清理所有缓存数据吗？')) {
        return
      }
      
      cleaning.value = true
      try {
        // 这里应该调用后端API清理缓存
        // await invoke('clear_cache')
        
        cacheSize.value = 0
        showToast('缓存已清理', 'success')
      } catch (error) {
        console.error('清理缓存失败:', error)
        showToast('清理缓存失败', 'error')
      } finally {
        cleaning.value = false
      }
    }
    
    // 清理日志
    const clearLogs = async () => {
      if (!confirm('确定要清理所有日志文件吗？')) {
        return
      }
      
      cleaning.value = true
      try {
        // 这里应该调用后端API清理日志
        // await invoke('clear_logs')
        
        logSize.value = 0
        showToast('日志已清理', 'success')
      } catch (error) {
        console.error('清理日志失败:', error)
        showToast('清理日志失败', 'error')
      } finally {
        cleaning.value = false
      }
    }
    
    // 清理下载历史
    const clearDownloadHistory = async () => {
      if (!confirm('确定要清理所有下载历史吗？')) {
        return
      }
      
      cleaning.value = true
      try {
        // 这里应该调用后端API清理下载历史
        // await invoke('clear_download_history')
        
        downloadHistoryCount.value = 0
        showToast('下载历史已清理', 'success')
      } catch (error) {
        console.error('清理下载历史失败:', error)
        showToast('清理下载历史失败', 'error')
      } finally {
        cleaning.value = false
      }
    }
    
    // 清理阅读历史
    const clearReadingHistory = async () => {
      if (!confirm('确定要清理过期的阅读历史吗？')) {
        return
      }
      
      cleaning.value = true
      try {
        // 这里应该调用后端API清理阅读历史
        // await invoke('clear_reading_history')
        
        readingHistoryCount.value = 0
        showToast('阅读历史已清理', 'success')
      } catch (error) {
        console.error('清理阅读历史失败:', error)
        showToast('清理阅读历史失败', 'error')
      } finally {
        cleaning.value = false
      }
    }
    
    // 重置设置
    const resetSettings = async () => {
      if (!confirm('确定要重置所有设置到默认值吗？')) {
        return
      }
      
      try {
        settingsStore.resetSettings()
        showToast('设置已重置', 'success')
      } catch (error) {
        console.error('重置设置失败:', error)
        showToast('重置设置失败', 'error')
      }
    }
    
    // 清空图书库
    const clearLibrary = async () => {
      if (!confirm('确定要清空图书库吗？这将删除所有图书记录（不删除文件）。')) {
        return
      }
      
      try {
        // 这里应该调用后端API清空图书库
        // await invoke('clear_library')
        
        showToast('图书库已清空', 'success')
      } catch (error) {
        console.error('清空图书库失败:', error)
        showToast('清空图书库失败', 'error')
      }
    }
    
    // 完全重置
    const fullReset = async () => {
      if (!confirm('确定要完全重置应用吗？这将删除所有数据，包括图书库、设置、阅读进度等。此操作不可恢复！')) {
        return
      }
      
      if (!confirm('最后确认：您真的要删除所有数据吗？建议先创建备份。')) {
        return
      }
      
      try {
        // 这里应该调用后端API完全重置
        // await invoke('full_reset')
        
        showToast('应用已重置，请重启应用', 'success')
      } catch (error) {
        console.error('完全重置失败:', error)
        showToast('完全重置失败', 'error')
      }
    }
    
    // 工具函数
    const getBackupTypeLabel = (type) => {
      const labels = {
        full: '完整备份',
        library: '图书库备份',
        settings: '设置备份',
        reading: '阅读数据备份'
      }
      return labels[type] || '未知类型'
    }
    
    const formatDate = (date) => {
      return new Date(date).toLocaleString('zh-CN')
    }
    
    const formatFileSize = (bytes) => {
      if (bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    }
    
    return {
      localSettings,
      backing,
      restoring,
      cleaning,
      backupHistory,
      cacheSize,
      logSize,
      downloadHistoryCount,
      readingHistoryCount,
      backupFrequencyOptions,
      updateSettings,
      createFullBackup,
      createLibraryBackup,
      createSettingsBackup,
      createReadingDataBackup,
      selectBackupFile,
      importLibrary,
      importSettings,
      restoreFromBackup,
      exportBackup,
      deleteBackup,
      refreshBackupHistory,
      cleanupOldBackups,
      clearCache,
      clearLogs,
      clearDownloadHistory,
      clearReadingHistory,
      resetSettings,
      clearLibrary,
      fullReset,
      getBackupTypeLabel,
      formatDate,
      formatFileSize
    }
  }
}
</script>

<style scoped>
.data-management {
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
  margin-bottom: 0.5rem;
}

.section-desc {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

.danger-section {
  border-color: #dc3545;
  background: rgba(220, 53, 69, 0.05);
}

/* 备份选项 */
.backup-options,
.restore-options,
.cleanup-options,
.reset-options {
  margin-bottom: 1.5rem;
}

.backup-item,
.restore-item,
.cleanup-item,
.reset-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  margin-bottom: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.backup-info,
.restore-info,
.cleanup-info,
.reset-info {
  flex: 1;
  margin-right: 1rem;
}

.backup-info h4,
.restore-info h4,
.cleanup-info h4,
.reset-info h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.backup-info p,
.restore-info p,
.cleanup-info p,
.reset-info p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin-bottom: 0;
}

.cleanup-size {
  display: block;
  color: var(--accent-color);
  font-size: 0.8rem;
  font-weight: 500;
  margin-top: 0.25rem;
}

/* 备份设置 */
.backup-settings,
.cleanup-settings {
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.setting-item {
  display: flex;
  align-items: center;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-color);
  gap: 1rem;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  flex: 1;
  min-width: 0;
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

/* 备份历史 */
.backup-list {
  margin-bottom: 1rem;
}

.empty-backups {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.backup-history {
  max-height: 300px;
  overflow-y: auto;
}

.backup-record {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  margin-bottom: 0.5rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.backup-details {
  flex: 1;
}

.backup-name {
  color: var(--text-primary);
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.backup-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.backup-type {
  background: var(--accent-color);
  color: white;
  padding: 0.125rem 0.5rem;
  border-radius: 12px;
  font-size: 0.75rem;
}

.backup-actions {
  display: flex;
  gap: 0.5rem;
}

/* 警告信息 */
.restore-warning,
.reset-warning {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 1rem;
  background: rgba(255, 193, 7, 0.1);
  border: 1px solid #ffc107;
  border-radius: 6px;
  margin-top: 1rem;
}

.warning-icon {
  font-size: 1.2rem;
  flex-shrink: 0;
}

.warning-text {
  color: var(--text-primary);
  font-size: 0.9rem;
  line-height: 1.4;
}

.reset-warning {
  background: rgba(220, 53, 69, 0.1);
  border-color: #dc3545;
}

/* 输入控件样式 */
.setting-select {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
  min-width: 120px;
}

.backup-count-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.backup-count-slider {
  width: 120px;
}

.backup-count-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 40px;
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

/* 按钮样式 */
.btn-primary,
.btn-secondary,
.btn-danger,
.btn-small {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-small {
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
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

.btn-danger {
  background: #dc3545;
  color: white;
}

.btn-danger:hover {
  background: #c82333;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .data-management {
    padding: 1rem;
  }
  
  .backup-item,
  .restore-item,
  .cleanup-item,
  .reset-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .backup-info,
  .restore-info,
  .cleanup-info,
  .reset-info {
    margin-right: 0;
  }
  
  .backup-record {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .backup-actions {
    width: 100%;
    justify-content: flex-end;
  }
  
  .backup-meta {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .setting-label {
    margin-right: 0;
  }
}
</style>