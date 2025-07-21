<template>
  <BaseCard 
    :class="{ 'plugin-disabled': !plugin.enabled }"
    hoverable
    clickable
    @click="$emit('click', plugin)"
  >
    <!-- 卡片头部 -->
    <template #header>
      <div class="plugin-info">
        <h3 class="plugin-name">{{ plugin.name }}</h3>
        <p class="plugin-meta">
          作者: {{ plugin.author || '未知' }} · 版本: {{ plugin.version }}
        </p>
      </div>
      <BaseBadge 
        :text="plugin.enabled ? '已启用' : '已禁用'"
        :variant="plugin.enabled ? 'success' : 'warning'"
        size="small"
      />
    </template>
    
    <!-- 卡片内容 -->
    <div class="plugin-content">
      <p class="plugin-description">{{ plugin.description || '暂无描述' }}</p>
      <div v-if="plugin.base_url" class="plugin-url">
        <small>书源地址: {{ plugin.base_url }}</small>
      </div>
    </div>
    
    <!-- 错误信息 -->
    <div v-if="error" class="plugin-error">
      <BaseBadge text="错误" variant="error" icon="⚠️" />
      <span class="error-text">{{ error }}</span>
    </div>
    
    <!-- 测试结果 -->
    <div v-if="testResult" class="plugin-test-result">
      <BaseBadge 
        :text="testResult.success ? '测试成功' : '测试失败'"
        :variant="testResult.success ? 'success' : 'error'"
        :icon="testResult.success ? '✅' : '❌'"
      />
      <div v-if="testResult.success" class="test-details">
        找到 {{ testResult.count }} 个搜索结果
      </div>
      <div v-else class="test-error">
        {{ testResult.error }}
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <template #footer>
      <div class="plugin-actions">
        <BaseButton 
          :variant="plugin.enabled ? 'secondary' : 'primary'"
          size="small"
          :loading="loading"
          @click.stop="togglePlugin"
        >
          {{ plugin.enabled ? '禁用' : '启用' }}
        </BaseButton>
        
        <BaseButton 
          variant="outline"
          size="small"
          :disabled="loading || !plugin.enabled"
          @click.stop="testPlugin"
        >
          测试
        </BaseButton>
        
        <BaseButton 
          variant="error"
          size="small"
          :disabled="loading"
          @click.stop="removePlugin"
        >
          删除
        </BaseButton>
      </div>
    </template>
  </BaseCard>
</template>

<script>
import { BaseCard, BaseButton, BaseBadge } from '../base'

export default {
  name: 'PluginCardRefactored',
  components: {
    BaseCard,
    BaseButton,
    BaseBadge
  },
  props: {
    plugin: {
      type: Object,
      required: true
    }
  },
  emits: ['click', 'toggle', 'remove', 'test'],
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
.plugin-disabled {
  opacity: 0.7;
}

.plugin-info {
  flex: 1;
  min-width: 0;
}

.plugin-name {
  color: var(--text-primary);
  font-size: var(--font-lg);
  font-weight: var(--font-semibold);
  margin: 0 0 var(--spacing-xs) 0;
}

.plugin-meta {
  color: var(--text-secondary);
  font-size: var(--font-sm);
  margin: 0;
}

.plugin-content {
  margin-bottom: var(--spacing-lg);
}

.plugin-description {
  color: var(--text-primary);
  font-size: var(--font-md);
  line-height: var(--line-height-normal);
  margin: 0 0 var(--spacing-md) 0;
}

.plugin-url {
  margin-top: var(--spacing-sm);
}

.plugin-url small {
  color: var(--text-secondary);
  font-size: var(--font-xs);
}

.plugin-error,
.plugin-test-result {
  margin: var(--spacing-lg) 0;
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.plugin-error {
  background: var(--error-light);
  border: 1px solid var(--error-color);
}

.error-text {
  color: var(--error-color);
  font-size: var(--font-sm);
}

.test-details,
.test-error {
  font-size: var(--font-sm);
  color: var(--text-secondary);
}

.test-error {
  color: var(--error-color);
}

.plugin-actions {
  display: flex;
  gap: var(--spacing-md);
  flex-wrap: wrap;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .plugin-actions {
    justify-content: stretch;
  }
  
  .plugin-actions .base-button {
    flex: 1;
  }
}
</style>