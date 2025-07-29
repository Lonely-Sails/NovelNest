<template>
  <div class="plugins-view">
    <div class="page-header">
      <div class="header-content">
        <h1>插件管理</h1>
        <p class="page-description">管理书源插件，扩展应用功能</p>
      </div>

      <div class="header-actions">
        <BaseButton @click="showInstallDialog = true" variant="primary" icon="📦">
          安装插件
        </BaseButton>
        <BaseButton
          @click="refreshPlugins"
          variant="outline"
          :loading="loading"
          icon="🔄"
        >
          刷新
        </BaseButton>
      </div>
    </div>

    <!-- 插件统计 -->
    <div class="plugins-stats" v-if="bookSources.length > 0">
      <BaseCard class="stat-card" compact>
        <div class="stat-number">{{ bookSources.length }}</div>
        <div class="stat-label">总插件数</div>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <div class="stat-number">{{ enabledSources.length }}</div>
        <div class="stat-label">已启用</div>
      </BaseCard>
      <BaseCard class="stat-card" compact>
        <div class="stat-number">{{ disabledSources.length }}</div>
        <div class="stat-label">已禁用</div>
      </BaseCard>
    </div>

    <!-- 插件列表 -->
    <div class="plugins-content">
      <Loading v-if="loading" message="加载插件中..." />

      <BaseCard v-else-if="!bookSources.length" class="empty-state">
        <div class="empty-icon">🔌</div>
        <h3>暂无插件</h3>
        <p>点击"安装插件"按钮添加书源插件</p>
        <template #actions>
          <BaseButton @click="showInstallDialog = true" variant="primary">
            安装第一个插件
          </BaseButton>
        </template>
      </BaseCard>

      <div v-else class="plugins-list">
        <div class="plugins-filter">
          <div class="filter-tabs">
            <BaseButton
              :variant="currentFilter === 'all' ? 'primary' : 'outline'"
              size="small"
              @click="currentFilter = 'all'"
            >
              全部 ({{ bookSources.length }})
            </BaseButton>
            <BaseButton
              :variant="currentFilter === 'enabled' ? 'primary' : 'outline'"
              size="small"
              @click="currentFilter = 'enabled'"
            >
              已启用 ({{ enabledSources.length }})
            </BaseButton>
            <BaseButton
              :variant="currentFilter === 'disabled' ? 'primary' : 'outline'"
              size="small"
              @click="currentFilter = 'disabled'"
            >
              已禁用 ({{ disabledSources.length }})
            </BaseButton>
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
    <Modal v-if="showInstallDialog" title="安装插件" @close="showInstallDialog = false">
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
          <div
            class="file-drop-zone"
            :class="{ 'drag-over': dragOver }"
            @drop="handleFileDrop"
            @dragover.prevent="dragOver = true"
            @dragleave="dragOver = false"
            @click="selectFile"
          >
            <div class="drop-content">
              <div class="drop-icon">📄</div>
              <p>拖拽插件文件到此处，或点击选择文件</p>
              <small>支持 .js 格式的插件文件</small>
            </div>
          </div>
        </div>
      </div>

      <template #actions>
        <BaseButton @click="showInstallDialog = false" variant="secondary">
          取消
        </BaseButton>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import PluginCard from '@/components/PluginCard.vue'
import PluginMonitor from '@/components/PluginMonitor.vue'
import Loading from '@/components/Loading.vue'
import { useToast } from '@/composables/useToast'
import { usePluginStore } from '@/stores/pluginStore'
import { open } from '@tauri-apps/plugin-dialog'
import { computed, ref, onMounted } from 'vue'

const pluginStore = usePluginStore()
const { showSuccess, showError } = useToast()

const loading = ref(false)
const currentFilter = ref('all')
const showInstallDialog = ref(false)
const installMethod = ref(null)
const dragOver = ref(false)

const bookSources = computed(() => pluginStore.bookSources)
const enabledSources = computed(() => pluginStore.enabledSources)
const disabledSources = computed(() => pluginStore.disabledSources)
const filteredSources = computed(() => {
  switch (currentFilter.value) {
    case 'enabled':
      return enabledSources.value
    case 'disabled':
      return disabledSources.value
    default:
      return bookSources.value
  }
})

const loadPlugins = async () => {
  loading.value = true
  try {
    await pluginStore.loadBookSources()
  } catch (error) {
    showError('加载插件失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

const refreshPlugins = async () => {
  await loadPlugins()
  showSuccess('插件列表已刷新')
}

const handleTogglePlugin = async (pluginId, enabled) => {
  try {
    await pluginStore.toggleBookSource(pluginId, enabled)
    const action = enabled ? '启用' : '禁用'
    showSuccess(`插件${action}成功`)
  } catch (error) {
    showError('操作失败: ' + error.message)
    throw error
  }
}

const handleRemovePlugin = async (pluginId) => {
  try {
    await pluginStore.removeBookSource(pluginId)
    showSuccess('插件删除成功')
  } catch (error) {
    showError('删除失败: ' + error.message)
    throw error
  }
}

const handleTestPlugin = async (pluginId) => {
  try {
    // 使用插件商店的测试方法
    const result = await pluginStore.testPlugin(pluginId)

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
}

const selectInstallMethod = (method) => {
  installMethod.value = method
}

const selectFile = async () => {
  await installPluginFile()
}

const handleFileDrop = async (event) => {
  event.preventDefault()
  dragOver.value = false

  const files = Array.from(event.dataTransfer.files)
  const jsFile = files.find(file => file.name.endsWith('.js'))

  if (jsFile) {
    // 在 Tauri 中，拖拽的文件可能有 path 属性
    const filePath = jsFile.path || jsFile.name
    if (filePath && filePath !== jsFile.name) {
      await installPluginFile(filePath)
    } else {
      showError('无法获取文件路径，请使用文件选择功能')
    }
  } else {
    showError('请选择 .js 格式的插件文件')
  }
}

const installPluginFile = async (filePath) => {
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

    loading.value = true
    await pluginStore.loadBookSource(filePath)

    showInstallDialog.value = false
    installMethod.value = null
    showSuccess('插件安装成功')

  } catch (error) {
    showError('安装失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadPlugins()
})
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

.header-content h1 {
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

/* 响应式设计 */
@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: 1rem;
  }

  .header-actions {
    width: 100%;
    justify-content: stretch;
  }

  .header-actions > * {
    flex: 1;
  }
}

/* 按钮样式已由BaseButton组件提供 */

/* 插件统计 */
.plugins-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
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
  padding-bottom: 1rem;
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
