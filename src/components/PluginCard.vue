<template>
  <div class="plugin-card" :class="{ 'plugin-disabled': !plugin.enabled }">
    <div class="plugin-header">
      <div class="plugin-info">
        <h3 class="plugin-name">{{ plugin.name }}</h3>
        <p class="plugin-author">作者: {{ plugin.author || '未知' }}</p>
        <p class="plugin-version">版本: {{ plugin.version }}</p>
      </div>
      <div class="plugin-status">
        <span class="status-badge" :class="plugin.enabled ? 'status-enabled' : 'status-disabled'">
          {{ plugin.enabled ? '已启用' : '已禁用' }}
        </span>
      </div>
    </div>
    
    <div class="plugin-description">
      <p>{{ plugin.description || '暂无描述' }}</p>
      <div class="plugin-url" v-if="plugin.base_url">
        <small>书源地址: {{ plugin.base_url }}</small>
      </div>
    </div>
    
    <div class="plugin-actions">
      <button 
        class="btn btn-primary"
        :class="{ 'btn-secondary': plugin.enabled }"
        @click="togglePlugin"
        :disabled="loading"
      >
        {{ plugin.enabled ? '禁用' : '启用' }}
      </button>
      
      <button 
        class="btn btn-outline"
        @click="testPlugin"
        :disabled="loading || !plugin.enabled"
      >
        测试
      </button>
      
      <button 
        class="btn btn-danger"
        @click="removePlugin"
        :disabled="loading"
      >
        删除
      </button>
    </div>
    
    <div class="plugin-error" v-if="error">
      <div class="error-message">
        <span class="error-icon">⚠️</span>
        {{ error }}
      </div>
    </div>
    
    <div class="plugin-test-result" v-if="testResult">
      <div class="test-success" v-if="testResult.success">
        <div class="test-summary">
          <span class="success-icon">✅</span>
          测试成功: 找到 {{ testResult.count }} 个搜索结果
        </div>
        <div class="test-details" v-if="testResult.details && testResult.details.length > 0">
          <ul>
            <li v-for="detail in testResult.details" :key="detail">{{ detail }}</li>
          </ul>
        </div>
      </div>
      <div class="test-error" v-else>
        <span class="error-icon">❌</span>
        测试失败: {{ testResult.error }}
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PluginCard',
  props: {
    plugin: {
      type: Object,
      required: true
    }
  },
  emits: ['toggle', 'remove', 'test'],
  data() {
    return {
      loading: false,
      error: null,
      testResult: null
    }
  },
  methods: {
    async togglePlugin() {
      this.loading = true
      this.error = null
      
      try {
        await this.$emit('toggle', this.plugin.id, !this.plugin.enabled)
      } catch (error) {
        this.error = error.message || '操作失败'
      } finally {
        this.loading = false
      }
    },
    
    async removePlugin() {
      if (!confirm(`确定要删除插件 "${this.plugin.name}" 吗？`)) {
        return
      }
      
      this.loading = true
      this.error = null
      
      try {
        await this.$emit('remove', this.plugin.id)
      } catch (error) {
        this.error = error.message || '删除失败'
      } finally {
        this.loading = false
      }
    },
    
    async testPlugin() {
      this.loading = true
      this.error = null
      this.testResult = null
      
      try {
        const result = await this.$emit('test', this.plugin.id)
        this.testResult = result
        
        // 3秒后清除测试结果
        setTimeout(() => {
          this.testResult = null
        }, 3000)
      } catch (error) {
        this.testResult = {
          success: false,
          error: error.message || '测试失败'
        }
        
        setTimeout(() => {
          this.testResult = null
        }, 5000)
      } finally {
        this.loading = false
      }
    }
  }
}
</script>

<style scoped>
.plugin-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 1rem;
  transition: all 0.2s ease;
}

.plugin-card:hover {
  border-color: var(--primary-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.plugin-disabled {
  opacity: 0.7;
}

.plugin-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.plugin-info h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0 0 0.25rem 0;
}

.plugin-author,
.plugin-version {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0.125rem 0;
}

.plugin-status {
  flex-shrink: 0;
}

.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
}

.status-enabled {
  background: var(--success-bg);
  color: var(--success-color);
}

.status-disabled {
  background: var(--warning-bg);
  color: var(--warning-color);
}

.plugin-description {
  margin-bottom: 1.5rem;
}

.plugin-description p {
  color: var(--text-primary);
  font-size: 0.9rem;
  line-height: 1.5;
  margin: 0 0 0.5rem 0;
}

.plugin-url {
  margin-top: 0.5rem;
}

.plugin-url small {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

.plugin-actions {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
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
  min-width: 60px;
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

.btn-secondary {
  background: var(--text-secondary);
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background: var(--text-primary);
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

.btn-danger {
  background: var(--error-color);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: var(--error-hover);
}

.plugin-error,
.plugin-test-result {
  margin-top: 1rem;
  padding: 0.75rem;
  border-radius: 6px;
  font-size: 0.85rem;
}

.plugin-error {
  background: var(--error-bg);
  border: 1px solid var(--error-color);
}

.error-message {
  color: var(--error-color);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.plugin-test-result {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
}

.test-success {
  color: var(--success-color);
}

.test-summary {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.test-details {
  margin-left: 1.5rem;
  font-size: 0.8rem;
}

.test-details ul {
  margin: 0;
  padding-left: 1rem;
  list-style-type: disc;
}

.test-details li {
  margin-bottom: 0.25rem;
  color: var(--text-secondary);
}

.test-error {
  color: var(--error-color);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.error-icon,
.success-icon {
  font-size: 1rem;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .plugin-header {
    flex-direction: column;
    gap: 1rem;
  }
  
  .plugin-actions {
    justify-content: stretch;
  }
  
  .btn {
    flex: 1;
    min-width: auto;
  }
}
</style>