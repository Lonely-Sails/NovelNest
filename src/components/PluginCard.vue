<template>
  <BaseCard class="plugin-card" :class="{ 'plugin-disabled': !plugin.enabled }">
    <template #header>
      <div class="plugin-info">
        <h3 class="plugin-name">{{ plugin.name }}</h3>
        <p class="plugin-author">作者: {{ plugin.author || '未知' }}</p>
        <p class="plugin-version">版本: {{ plugin.version }}</p>
      </div>
      <BaseBadge 
        :variant="plugin.enabled ? 'success' : 'warning'"
        class="plugin-status"
      >
        {{ plugin.enabled ? '已启用' : '已禁用' }}
      </BaseBadge>
    </template>
    
    <div class="plugin-description">
      <p>{{ plugin.description || '暂无描述' }}</p>
      <div class="plugin-url" v-if="plugin.base_url">
        <small>书源地址: {{ plugin.base_url }}</small>
      </div>
    </div>
    
    <template #actions>
      <BaseButton 
        :variant="plugin.enabled ? 'secondary' : 'primary'"
        @click="togglePlugin"
        :loading="loading"
      >
        {{ plugin.enabled ? '禁用' : '启用' }}
      </BaseButton>
      
      <BaseButton 
        variant="outline"
        @click="testPlugin"
        :disabled="loading || !plugin.enabled"
      >
        测试
      </BaseButton>
      
      <BaseButton 
        variant="danger"
        @click="removePlugin"
        :loading="loading"
      >
        删除
      </BaseButton>
    </template>
    
    <div class="plugin-error" v-if="error">
      <BaseBadge variant="error" class="error-message">
        <span class="error-icon">⚠️</span>
        {{ error }}
      </BaseBadge>
    </div>
    
    <div class="plugin-test-result" v-if="testResult">
      <BaseBadge 
        :variant="testResult.success ? 'success' : 'error'" 
        class="test-result-badge"
      >
        <span v-if="testResult.success">
          <span class="success-icon">✅</span>
          测试成功: 找到 {{ testResult.count }} 个搜索结果
        </span>
        <span v-else>
          <span class="error-icon">❌</span>
          测试失败: {{ testResult.error }}
        </span>
      </BaseBadge>
      
      <div class="test-details" v-if="testResult.success && testResult.details && testResult.details.length > 0">
        <ul>
          <li v-for="detail in testResult.details" :key="detail">{{ detail }}</li>
        </ul>
      </div>
    </div>
  </BaseCard>
</template>

<script>
export default {
  name: 'PluginCard',
  // 基础组件已全局注册，无需导入
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
  margin-bottom: 1rem;
}

.plugin-disabled {
  opacity: 0.7;
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

.plugin-error,
.plugin-test-result {
  margin-top: 1rem;
}

.test-result-badge {
  display: block;
  margin-bottom: 0.5rem;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.test-details {
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