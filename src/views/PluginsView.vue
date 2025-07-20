<template>
  <div class="plugins-view">
    <div class="page-header">
      <h1>插件管理</h1>
      <p class="page-description">管理书源插件，扩展应用功能</p>
      
      <div class="header-actions">
        <button class="btn btn-primary" @click="showInstallDialog = true">
          <span class="btn-icon">📦</span>
          安装插件
        </button>
        <button class="btn btn-outline" @click="refreshPlugins" :disabled="loading">
          <span class="btn-icon">🔄</span>
          刷新
        </button>
      </div>
    </div>
    
    <!-- 插件统计 -->
    <div class="plugins-stats" v-if="bookSources.length > 0">
      <div class="stat-card">
        <div class="stat-number">{{ bookSources.length }}</div>
        <div class="stat-label">总插件数</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ enabledSources.length }}</div>
        <div class="stat-label">已启用</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">{{ disabledSources.length }}</div>
        <div class="stat-label">已禁用</div>
      </div>
    </div>
    
    <!-- 插件列表 -->
    <div class="plugins-content">
      <Loading v-if="loading" message="加载插件中..." />
      
      <div v-else-if="bookSources.length === 0" class="empty-state">
        <div class="empty-icon">🔌</div>
        <h3>暂无插件</h3>
        <p>点击"安装插件"按钮添加书源插件</p>
        <button class="btn btn-primary" @click="showInstallDialog = true">
          安装第一个插件
        </button>
      </div>
      
      <div v-else class="plugins-list">
        <div class="plugins-filter">
          <div class="filter-tabs">
            <button 
              class="filter-tab"
              :class="{ active: currentFilter === 'all' }"
              @click="currentFilter = 'all'"
            >
              全部 ({{ bookSources.length }})
            </button>
            <button 
              class="filter-tab"
              :class="{ active: currentFilter === 'enabled' }"
              @click="currentFilter = 'enabled'"
            >
              已启用 ({{ enabledSources.length }})
            </button>
            <button 
              class="filter-tab"
              :class="{ active: currentFilter === 'disabled' }"
              @click="currentFilter = 'disabled'"
            >
              已禁用 ({{ disabledSources.length }})
            </button>
          </div>
        </div>
        
        <div class="plugins-grid">
          <PluginCard
            v-for="plugin in filteredSources"
            :key="plugin.id"
            :plugin="plugin"
            @toggle="handleTogglePlugin"
            @remove="handleRemovePlugin"
            @test="handleTestPlugin"
          />
        </div>
        
        <!-- 插件监控面板 -->
        <div class="plugin-monitor-section" v-if="bookSources.length > 0">
          <PluginMonitor />
        </div>
      </div>
    </div>
    
    <!-- 安装插件对话框 -->
    <Modal v-if="showInstallDialog" @close="showInstallDialog = false">
      <template #header>
        <h3>安装插件</h3>
      </template>
      
      <template #body>
        <div class="install-dialog">
          <div class="install-methods">
            <div class="method-card" @click="selectInstallMethod('file')">
              <div class="method-icon">📁</div>
              <h4>从文件安装</h4>
              <p>选择本地的 .js 插件文件</p>
            </div>
            
            <div class="method-card disabled" title="功能开发中">
              <div class="method-icon">🌐</div>
              <h4>在线安装</h4>
              <p>从插件商店安装（开发中）</p>
            </div>
          </div>
          
          <div v-if="installMethod === 'file'" class="file-install">
            <div class="file-drop-zone" 
                 :class="{ 'drag-over': dragOver }"
                 @drop="handleFileDrop"
                 @dragover.prevent="dragOver = true"
                 @dragleave="dragOver = false"
                 @click="selectFile">
              <div class="drop-content">
                <div class="drop-icon">📄</div>
                <p>拖拽插件文件到此处，或点击选择文件</p>
                <small>支持 .js 格式的插件文件</small>
              </div>
            </div>
            
            <input 
              ref="fileInput"
              type="file"
              accept=".js"
              style="display: none"
              @change="handleFileSelect"
            />
          </div>
        </div>
      </template>
      
      <template #footer>
        <button class="btn btn-secondary" @click="showInstallDialog = false">
          取消
        </button>
      </template>
    </Modal>
    
    <!-- Toast 提示 -->
    <Toast 
      v-if="toast.show"
      :type="toast.type"
      :message="toast.message"
      @close="toast.show = false"
    />
  </div>
</template>

<script>
import { usePluginStore } from '@/stores/pluginStore'
import PluginCard from '@/components/PluginCard.vue'
import PluginMonitor from '@/components/PluginMonitor.vue'
import Loading from '@/components/Loading.vue'
import Modal from '@/components/Modal.vue'
import Toast from '@/components/Toast.vue'
import { open } from '@tauri-apps/plugin-dialog'

export default {
  name: 'PluginsView',
  components: {
    PluginCard,
    PluginMonitor,
    Loading,
    Modal,
    Toast
  },
  data() {
    return {
      loading: false,
      currentFilter: 'all',
      showInstallDialog: false,
      installMethod: null,
      dragOver: false,
      toast: {
        show: false,
        type: 'info',
        message: ''
      }
    }
  },
  computed: {
    pluginStore() {
      return usePluginStore()
    },
    bookSources() {
      return this.pluginStore.bookSources
    },
    enabledSources() {
      return this.pluginStore.enabledSources
    },
    disabledSources() {
      return this.pluginStore.disabledSources
    },
    filteredSources() {
      switch (this.currentFilter) {
        case 'enabled':
          return this.enabledSources
        case 'disabled':
          return this.disabledSources
        default:
          return this.bookSources
      }
    }
  },
  async mounted() {
    await this.loadPlugins()
  },
  methods: {
    async loadPlugins() {
      this.loading = true
      try {
        await this.pluginStore.loadBookSources()
      } catch (error) {
        this.showToast('error', '加载插件失败: ' + error.message)
      } finally {
        this.loading = false
      }
    },
    
    async refreshPlugins() {
      await this.loadPlugins()
      this.showToast('success', '插件列表已刷新')
    },
    
    async handleTogglePlugin(pluginId, enabled) {
      try {
        await this.pluginStore.toggleBookSource(pluginId, enabled)
        const action = enabled ? '启用' : '禁用'
        this.showToast('success', `插件${action}成功`)
      } catch (error) {
        this.showToast('error', '操作失败: ' + error.message)
        throw error
      }
    },
    
    async handleRemovePlugin(pluginId) {
      try {
        await this.pluginStore.removeBookSource(pluginId)
        this.showToast('success', '插件删除成功')
      } catch (error) {
        this.showToast('error', '删除失败: ' + error.message)
        throw error
      }
    },
    
    async handleTestPlugin(pluginId) {
      try {
        // 使用插件商店的测试方法
        const result = await this.pluginStore.testPlugin(pluginId)
        
        if (result.success) {
          return {
            success: true,
            count: result.searchCount,
            details: result.details
          }
        } else {
          return {
            success: false,
            error: result.error
          }
        }
      } catch (error) {
        return {
          success: false,
          error: error.message
        }
      }
    },
    
    selectInstallMethod(method) {
      this.installMethod = method
    },
    
    selectFile() {
      this.$refs.fileInput.click()
    },
    
    async handleFileSelect(event) {
      const file = event.target.files[0]
      if (file) {
        await this.installPluginFile(file.path)
      }
    },
    
    async handleFileDrop(event) {
      event.preventDefault()
      this.dragOver = false
      
      const files = Array.from(event.dataTransfer.files)
      const jsFile = files.find(file => file.name.endsWith('.js'))
      
      if (jsFile) {
        await this.installPluginFile(jsFile.path)
      } else {
        this.showToast('error', '请选择 .js 格式的插件文件')
      }
    },
    
    async installPluginFile(filePath) {
      try {
        // 如果没有文件路径，打开文件选择对话框
        if (!filePath) {
          const selected = await open({
            title: '选择插件文件',
            filters: [{
              name: 'JavaScript',
              extensions: ['js']
            }]
          })
          
          if (!selected) return
          filePath = selected
        }
        
        this.loading = true
        await this.pluginStore.loadBookSource(filePath)
        
        this.showInstallDialog = false
        this.installMethod = null
        this.showToast('success', '插件安装成功')
        
      } catch (error) {
        this.showToast('error', '安装失败: ' + error.message)
      } finally {
        this.loading = false
      }
    },
    
    showToast(type, message) {
      this.toast = {
        show: true,
        type,
        message
      }
      
      // 3秒后自动关闭
      setTimeout(() => {
        this.toast.show = false
      }, 3000)
    }
  }
}
</script>

<style scoped>
.plugins-view {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  gap: 2rem;
}

.page-header h1 {
  color: var(--text-primary);
  font-size: 1.8rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.page-description {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.header-actions {
  display: flex;
  gap: 1rem;
  flex-shrink: 0;
}

.btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
}

.btn-outline {
  background: transparent;
  color: var(--primary-color);
  border: 1px solid var(--primary-color);
}

.btn-outline:hover:not(:disabled) {
  background: var(--primary-color);
  color: white;
}

.btn-secondary {
  background: var(--text-secondary);
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background: var(--text-primary);
}

.btn-icon {
  font-size: 1rem;
}

/* 插件统计 */
.plugins-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
}

.stat-number {
  font-size: 2rem;
  font-weight: 700;
  color: var(--primary-color);
  margin-bottom: 0.5rem;
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.85rem;
  font-weight: 500;
}

/* 插件内容区域 */
.plugins-content {
  background: var(--bg-primary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  min-height: 400px;
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 3rem;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.empty-state h3 {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  font-size: 1.2rem;
  font-weight: 500;
}

.empty-state p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 1.5rem;
}

/* 插件列表 */
.plugins-list {
  padding: 1.5rem;
}

.plugins-filter {
  margin-bottom: 1.5rem;
}

.filter-tabs {
  display: flex;
  gap: 0.5rem;
  border-bottom: 1px solid var(--border-color);
}

.filter-tab {
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 0.9rem;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s ease;
}

.filter-tab:hover {
  color: var(--text-primary);
}

.filter-tab.active {
  color: var(--primary-color);
  border-bottom-color: var(--primary-color);
}

.plugins-grid {
  display: grid;
  gap: 1rem;
}

.plugin-monitor-section {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

/* 安装对话框 */
.install-dialog {
  padding: 1rem 0;
}

.install-methods {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.method-card {
  background: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.method-card:hover:not(.disabled) {
  border-color: var(--primary-color);
  background: var(--primary-bg);
}

.method-card.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.method-icon {
  font-size: 2rem;
  margin-bottom: 1rem;
}

.method-card h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.method-card p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0;
}

/* 文件安装 */
.file-install {
  margin-top: 1rem;
}

.file-drop-zone {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 2rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.file-drop-zone:hover,
.file-drop-zone.drag-over {
  border-color: var(--primary-color);
  background: var(--primary-bg);
}

.drop-content {
  pointer-events: none;
}

.drop-icon {
  font-size: 2rem;
  margin-bottom: 1rem;
  opacity: 0.6;
}

.drop-content p {
  color: var(--text-primary);
  font-size: 0.9rem;
  margin-bottom: 0.5rem;
}

.drop-content small {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .plugins-view {
    padding: 1rem;
  }
  
  .page-header {
    flex-direction: column;
    gap: 1rem;
  }
  
  .header-actions {
    width: 100%;
    justify-content: stretch;
  }
  
  .btn {
    flex: 1;
  }
  
  .plugins-stats {
    grid-template-columns: repeat(3, 1fr);
  }
  
  .stat-card {
    padding: 1rem;
  }
  
  .stat-number {
    font-size: 1.5rem;
  }
  
  .filter-tabs {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }
  
  .filter-tab {
    white-space: nowrap;
    flex-shrink: 0;
  }
  
  .install-methods {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 480px) {
  .plugins-stats {
    grid-template-columns: 1fr;
  }
}
</style>