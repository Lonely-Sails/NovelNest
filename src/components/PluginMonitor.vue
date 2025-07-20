<template>
  <div class="plugin-monitor">
    <div class="monitor-header">
      <h4>插件执行监控</h4>
      <button class="btn btn-small" @click="refreshStats">刷新</button>
    </div>
    
    <div class="monitor-stats">
      <div class="stat-item">
        <span class="stat-label">已加载插件:</span>
        <span class="stat-value">{{ stats.loadedCount }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">错误数量:</span>
        <span class="stat-value error">{{ stats.errorCount }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">活跃任务:</span>
        <span class="stat-value">{{ stats.activeTimeouts }}</span>
      </div>
    </div>
    
    <div class="monitor-actions">
      <button class="btn btn-outline btn-small" @click="clearErrors">
        清除错误
      </button>
      <button class="btn btn-danger btn-small" @click="clearAllPlugins">
        清除所有插件
      </button>
    </div>
    
    <div class="plugin-errors" v-if="errorReport && errorReport.recentErrors.length > 0">
      <h5>最近错误 ({{ errorReport.summary.totalErrors }})</h5>
      
      <div class="error-types" v-if="errorReport.summary.errorTypes.length > 0">
        <span 
          v-for="type in errorReport.summary.errorTypes" 
          :key="type"
          class="error-type-badge"
        >
          {{ getErrorTypeLabel(type) }}
        </span>
      </div>
      
      <div class="error-list">
        <div 
          v-for="error in errorReport.recentErrors.slice(0, 5)" 
          :key="`${error.sourceId}_${error.timestamp}`"
          class="error-item"
          :class="`error-type-${error.type}`"
        >
          <div class="error-header">
            <span class="error-source">{{ error.sourceId || '未知插件' }}</span>
            <span class="error-type">{{ getErrorTypeLabel(error.type) }}</span>
            <span class="error-time">{{ formatTime(error.timestamp) }}</span>
          </div>
          <div class="error-message">{{ error.message }}</div>
          <div class="error-context" v-if="error.context">
            <small>上下文: {{ error.context }}</small>
          </div>
        </div>
      </div>
      
      <div class="error-recommendations" v-if="errorReport.recommendations.length > 0">
        <h6>建议</h6>
        <ul>
          <li v-for="recommendation in errorReport.recommendations" :key="recommendation">
            {{ recommendation }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
import { usePluginStore } from '@/stores/pluginStore'
import { pluginManager } from '@/utils/pluginManager'

export default {
  name: 'PluginMonitor',
  data() {
    return {
      stats: {
        loadedCount: 0,
        errorCount: 0,
        activeTimeouts: 0,
        totalErrors: 0,
        errorsByType: {}
      },
      errorReport: null,
      refreshInterval: null
    }
  },
  computed: {
    pluginStore() {
      return usePluginStore()
    }
  },
  mounted() {
    this.refreshStats()
    this.startAutoRefresh()
  },
  beforeUnmount() {
    this.stopAutoRefresh()
  },
  methods: {
    refreshStats() {
      this.stats = this.pluginStore.getPluginManagerStats()
      this.errorReport = this.pluginStore.getPluginErrorReport()
    },
    
    clearErrors() {
      this.pluginStore.clearPluginErrorHistory()
      this.refreshStats()
    },
    
    async clearAllPlugins() {
      if (confirm('确定要清除所有已加载的插件吗？这将停止所有正在执行的插件任务。')) {
        this.pluginStore.clearPluginManager()
        this.refreshStats()
      }
    },
    
    startAutoRefresh() {
      this.refreshInterval = setInterval(() => {
        this.refreshStats()
      }, 5000) // 每5秒刷新一次
    },
    
    stopAutoRefresh() {
      if (this.refreshInterval) {
        clearInterval(this.refreshInterval)
        this.refreshInterval = null
      }
    },
    
    formatTime(time) {
      return new Date(time).toLocaleTimeString()
    },
    
    getErrorTypeLabel(type) {
      const labels = {
        'LOAD_FAILED': '加载失败',
        'EXECUTION_TIMEOUT': '执行超时',
        'NETWORK_ERROR': '网络错误',
        'PARSE_ERROR': '解析错误',
        'VALIDATION_ERROR': '验证失败',
        'SANDBOX_VIOLATION': '安全违规',
        'UNKNOWN': '未知错误'
      }
      return labels[type] || type
    }
  }
}
</script>

<style scoped>
.plugin-monitor {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 1rem;
  font-size: 0.85rem;
}

.monitor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.monitor-header h4 {
  color: var(--text-primary);
  font-size: 0.9rem;
  font-weight: 600;
  margin: 0;
}

.monitor-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--bg-primary);
  border-radius: 4px;
  border: 1px solid var(--border-color);
}

.stat-label {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

.stat-value {
  color: var(--text-primary);
  font-weight: 600;
}

.stat-value.error {
  color: var(--error-color);
}

.monitor-actions {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.btn {
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  border: none;
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-small {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
}

.btn-outline {
  background: transparent;
  color: var(--primary-color);
  border: 1px solid var(--primary-color);
}

.btn-outline:hover {
  background: var(--primary-color);
  color: white;
}

.btn-danger {
  background: var(--error-color);
  color: white;
}

.btn-danger:hover {
  background: var(--error-hover);
}

.plugin-errors {
  border-top: 1px solid var(--border-color);
  padding-top: 1rem;
}

.plugin-errors h5 {
  color: var(--text-primary);
  font-size: 0.85rem;
  font-weight: 600;
  margin: 0 0 0.75rem 0;
}

.plugin-errors h6 {
  color: var(--text-primary);
  font-size: 0.8rem;
  font-weight: 600;
  margin: 1rem 0 0.5rem 0;
}

.error-types {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-bottom: 0.75rem;
}

.error-type-badge {
  background: var(--warning-bg);
  color: var(--warning-color);
  padding: 0.125rem 0.5rem;
  border-radius: 10px;
  font-size: 0.7rem;
  font-weight: 500;
}

.error-list {
  max-height: 200px;
  overflow-y: auto;
}

.error-item {
  background: var(--error-bg);
  border: 1px solid var(--error-color);
  border-radius: 4px;
  padding: 0.5rem;
  margin-bottom: 0.5rem;
}

.error-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.25rem;
  gap: 0.5rem;
}

.error-source {
  color: var(--error-color);
  font-weight: 600;
  font-size: 0.75rem;
  flex-shrink: 0;
}

.error-type {
  background: var(--error-bg);
  color: var(--error-color);
  padding: 0.125rem 0.375rem;
  border-radius: 8px;
  font-size: 0.65rem;
  font-weight: 500;
  flex-shrink: 0;
}

.error-time {
  color: var(--text-secondary);
  font-size: 0.7rem;
}

.error-message {
  color: var(--error-color);
  font-size: 0.75rem;
  line-height: 1.4;
  word-break: break-word;
  margin-bottom: 0.25rem;
}

.error-context {
  color: var(--text-secondary);
  font-size: 0.7rem;
  font-style: italic;
}

.error-recommendations {
  margin-top: 1rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-color);
}

.error-recommendations ul {
  margin: 0;
  padding-left: 1rem;
  list-style-type: disc;
}

.error-recommendations li {
  color: var(--text-secondary);
  font-size: 0.75rem;
  line-height: 1.4;
  margin-bottom: 0.25rem;
}

/* 错误类型样式 */
.error-type-EXECUTION_TIMEOUT {
  border-left: 3px solid var(--warning-color);
}

.error-type-NETWORK_ERROR {
  border-left: 3px solid var(--error-color);
}

.error-type-PARSE_ERROR {
  border-left: 3px solid var(--primary-color);
}

.error-type-LOAD_FAILED {
  border-left: 3px solid var(--error-color);
}

.error-type-SANDBOX_VIOLATION {
  border-left: 3px solid var(--error-color);
  background: rgba(var(--error-color-rgb), 0.05);
}
</style>