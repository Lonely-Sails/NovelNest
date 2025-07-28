<template>
  <div class="settings-view">
    <div class="page-header">
      <h1>设置</h1>
      <p class="page-description">配置应用偏好和功能选项</p>
    </div>
    
    <div class="settings-layout">
      <!-- 设置导航 -->
      <BaseCard class="settings-nav">
        <div class="nav-section">
          <h3>通用设置</h3>
          <div class="nav-list">
            <BaseButton
              v-for="item in generalNavItems" 
              :key="item.key"
              :variant="activeTab === item.key ? 'primary' : 'ghost'"
              @click="activeTab = item.key"
              :icon="item.icon"
              class="nav-item"
            >
              {{ item.label }}
            </BaseButton>
          </div>
        </div>
        
        <div class="nav-section">
          <h3>阅读设置</h3>
          <div class="nav-list">
            <BaseButton
              v-for="item in readerNavItems" 
              :key="item.key"
              :variant="activeTab === item.key ? 'primary' : 'ghost'"
              @click="activeTab = item.key"
              :icon="item.icon"
              class="nav-item"
            >
              {{ item.label }}
            </BaseButton>
          </div>
        </div>
        
        <div class="nav-section">
          <h3>高级设置</h3>
          <div class="nav-list">
            <BaseButton
              v-for="item in advancedNavItems" 
              :key="item.key"
              :variant="activeTab === item.key ? 'primary' : 'ghost'"
              @click="activeTab = item.key"
              :icon="item.icon"
              class="nav-item"
            >
              {{ item.label }}
            </BaseButton>
          </div>
        </div>
      </BaseCard>
      
      <!-- 设置内容 -->
      <BaseCard class="settings-content">
        <!-- 应用设置 -->
        <div v-if="activeTab === 'app'" class="settings-panel">
          <AppSettings />
        </div>
        
        <!-- 主题设置 -->
        <div v-if="activeTab === 'theme'" class="settings-panel">
          <ThemeSettings />
        </div>
        
        <!-- 阅读器设置 -->
        <div v-if="activeTab === 'reader'" class="settings-panel">
          <ReaderSettings />
        </div>
        
        <!-- 字体设置 -->
        <div v-if="activeTab === 'font'" class="settings-panel">
          <FontSettings />
        </div>
        
        <!-- 书源设置 -->
        <div v-if="activeTab === 'sources'" class="settings-panel">
          <SourceSettings />
        </div>
        
        <!-- 下载设置 -->
        <div v-if="activeTab === 'download'" class="settings-panel">
          <DownloadSettings />
        </div>
        
        <!-- 数据管理 -->
        <div v-if="activeTab === 'data'" class="settings-panel">
          <DataManagement />
        </div>
        

      </BaseCard>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'
import AppSettings from '../components/settings/AppSettings.vue'
import ThemeSettings from '../components/settings/ThemeSettings.vue'
import ReaderSettings from '../components/settings/ReaderSettings.vue'
import FontSettings from '../components/settings/FontSettings.vue'
import SourceSettings from '../components/settings/SourceSettings.vue'
import DownloadSettings from '../components/settings/DownloadSettings.vue'
import DataManagement from '../components/settings/DataManagement.vue'

export default {
  name: 'SettingsView',
  components: {
    AppSettings,
    ThemeSettings,
    ReaderSettings,
    FontSettings,
    SourceSettings,
    DownloadSettings,
    DataManagement
  },
  setup() {
    const activeTab = ref('app')
    
    const generalNavItems = [
      { key: 'app', label: '应用设置', icon: '⚙️' },
      { key: 'theme', label: '主题外观', icon: '🎨' }
    ]
    
    const readerNavItems = [
      { key: 'reader', label: '阅读体验', icon: '📖' },
      { key: 'font', label: '字体设置', icon: '🔤' }
    ]
    
    const advancedNavItems = [
      { key: 'sources', label: '书源管理', icon: '🌐' },
      { key: 'download', label: '下载设置', icon: '⬇️' },
      { key: 'data', label: '数据管理', icon: '💾' }
    ]
    
    return {
      activeTab,
      generalNavItems,
      readerNavItems,
      advancedNavItems
    }
  }
}
</script>

<style scoped>
.settings-view {
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  height: calc(100vh - 3rem);
  overflow: hidden;
}

.page-header {
  margin-bottom: 1.5rem;
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

.settings-layout {
  display: flex;
  gap: 1.5rem;
  height: calc(100% - 5rem);
}

/* 设置导航 */
.settings-nav {
  width: 250px;
  padding: 1.5rem;
  overflow-y: auto;
  flex-shrink: 0;
}

.nav-section {
  margin-bottom: 2rem;
}

.nav-section:last-child {
  margin-bottom: 0;
}

.nav-section h3 {
  color: var(--text-primary);
  font-size: 0.85rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 0.75rem;
  opacity: 0.8;
}

.nav-list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.nav-item {
  width: 100%;
  justify-content: flex-start;
}

/* 设置内容 */
.settings-content {
  flex: 1;
  overflow-y: auto;
}

.settings-panel {
  height: 100%;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .settings-layout {
    flex-direction: column;
    height: auto;
  }
  
  .settings-nav {
    width: 100%;
    order: 2;
    max-height: 300px;
  }
  
  .settings-content {
    order: 1;
    min-height: 400px;
  }
  
  .nav-section {
    margin-bottom: 1rem;
  }
  
  .nav-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .nav-list li {
    flex: 0 0 auto;
    margin-bottom: 0;
    padding: 0.5rem 0.75rem;
    border-radius: 20px;
    font-size: 0.85rem;
  }
  
  .nav-icon {
    margin-right: 0.5rem;
  }
}

/* 滚动条样式 */
.settings-nav::-webkit-scrollbar,
.settings-content::-webkit-scrollbar {
  width: 6px;
}

.settings-nav::-webkit-scrollbar-track,
.settings-content::-webkit-scrollbar-track {
  background: transparent;
}

.settings-nav::-webkit-scrollbar-thumb,
.settings-content::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 3px;
}

.settings-nav::-webkit-scrollbar-thumb:hover,
.settings-content::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>