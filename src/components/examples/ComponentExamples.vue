<template>
  <div class="component-examples">
    <div class="container">
      <h1>基础组件使用示例</h1>
      <p>以下展示了如何使用新的基础组件来重构现有界面</p>
      
      <!-- 卡片示例 -->
      <section class="example-section">
        <h2>卡片组件 (BaseCard)</h2>
        <div class="example-grid">
          <!-- 默认卡片 -->
          <BaseCard title="默认卡片" hoverable>
            <p>这是一个默认样式的卡片，具有悬停效果。</p>
            <template #actions>
              <BaseButton size="small" variant="outline">编辑</BaseButton>
            </template>
          </BaseCard>
          
          <!-- 主要卡片 -->
          <BaseCard title="主要卡片" variant="primary" hoverable>
            <p>这是一个主要样式的卡片。</p>
            <template #footer>
              <BaseButton variant="secondary" size="small">取消</BaseButton>
              <BaseButton size="small">确认</BaseButton>
            </template>
          </BaseCard>
          
          <!-- 成功卡片 -->
          <BaseCard title="成功状态" variant="success" compact>
            <p>操作已成功完成！</p>
          </BaseCard>
        </div>
      </section>
      
      <!-- 按钮示例 -->
      <section class="example-section">
        <h2>按钮组件 (BaseButton)</h2>
        <div class="button-groups">
          <div class="button-group">
            <h3>按钮变体</h3>
            <BaseButton>主要按钮</BaseButton>
            <BaseButton variant="secondary">次要按钮</BaseButton>
            <BaseButton variant="success">成功按钮</BaseButton>
            <BaseButton variant="warning">警告按钮</BaseButton>
            <BaseButton variant="error">错误按钮</BaseButton>
            <BaseButton variant="outline">轮廓按钮</BaseButton>
            <BaseButton variant="ghost">幽灵按钮</BaseButton>
            <BaseButton variant="link">链接按钮</BaseButton>
          </div>
          
          <div class="button-group">
            <h3>按钮大小</h3>
            <BaseButton size="small">小按钮</BaseButton>
            <BaseButton size="medium">中按钮</BaseButton>
            <BaseButton size="large">大按钮</BaseButton>
          </div>
          
          <div class="button-group">
            <h3>按钮状态</h3>
            <BaseButton icon="📖">带图标</BaseButton>
            <BaseButton :loading="loading" @click="toggleLoading">
              {{ loading ? '加载中...' : '点击加载' }}
            </BaseButton>
            <BaseButton disabled>禁用按钮</BaseButton>
            <BaseButton block>块级按钮</BaseButton>
          </div>
        </div>
      </section>
      
      <!-- 输入框示例 -->
      <section class="example-section">
        <h2>输入框组件 (BaseInput)</h2>
        <div class="input-examples">
          <div class="input-group">
            <BaseInput
              v-model="formData.name"
              label="姓名"
              placeholder="请输入您的姓名"
              required
              help-text="这是一个必填字段"
            />
            
            <BaseInput
              v-model="formData.email"
              type="email"
              label="邮箱地址"
              placeholder="example@email.com"
              prefix-icon="📧"
              clearable
            />
            
            <BaseInput
              v-model="formData.password"
              type="password"
              label="密码"
              placeholder="请输入密码"
              :error-message="passwordError"
            />
            
            <BaseInput
              v-model="formData.bio"
              type="textarea"
              label="个人简介"
              placeholder="介绍一下自己..."
              :rows="4"
              success-message="格式正确"
            />
          </div>
        </div>
      </section>
      
      <!-- 开关示例 -->
      <section class="example-section">
        <h2>开关组件 (BaseSwitch)</h2>
        <div class="switch-examples">
          <BaseSwitch
            v-model="settings.notifications"
            label="推送通知"
            description="接收应用推送通知"
          />
          
          <BaseSwitch
            v-model="settings.darkMode"
            label="暗色模式"
            description="启用暗色主题"
            show-icon
          />
          
          <BaseSwitch
            v-model="settings.autoSave"
            label="自动保存"
            size="small"
          />
          
          <BaseSwitch
            v-model="settings.premium"
            label="高级功能"
            description="此功能需要升级账户"
            disabled
          />
        </div>
      </section>
      
      <!-- 徽章示例 -->
      <section class="example-section">
        <h2>徽章组件 (BaseBadge)</h2>
        <div class="badge-examples">
          <div class="badge-group">
            <h3>基础徽章</h3>
            <BaseBadge text="默认" />
            <BaseBadge text="主要" variant="primary" />
            <BaseBadge text="成功" variant="success" />
            <BaseBadge text="警告" variant="warning" />
            <BaseBadge text="错误" variant="error" />
            <BaseBadge text="信息" variant="info" />
          </div>
          
          <div class="badge-group">
            <h3>轮廓徽章</h3>
            <BaseBadge text="轮廓" outline />
            <BaseBadge text="主要" variant="primary" outline />
            <BaseBadge text="成功" variant="success" outline />
          </div>
          
          <div class="badge-group">
            <h3>特殊样式</h3>
            <BaseBadge text="圆角" rounded />
            <BaseBadge text="99+" variant="error" rounded />
            <BaseBadge dot variant="success" />
            <BaseBadge dot variant="warning" />
            <BaseBadge text="新" variant="primary" icon="✨" />
          </div>
        </div>
      </section>
      
      <!-- 重构示例 -->
      <section class="example-section">
        <h2>重构示例：书籍卡片</h2>
        <div class="refactor-example">
          <BaseCard 
            title="重构后的书籍卡片" 
            hoverable 
            clickable
            @click="handleBookClick"
          >
            <div class="book-content">
              <div class="book-cover">
                <img src="/api/placeholder/120/160" alt="书籍封面" />
                <BaseBadge text="EPUB" variant="primary" size="small" />
              </div>
              <div class="book-info">
                <h3>《示例小说》</h3>
                <p class="author">作者：张三</p>
                <p class="description line-clamp-2">
                  这是一本非常有趣的小说，讲述了一个关于冒险和友谊的故事...
                </p>
                <div class="book-meta">
                  <BaseBadge text="2.5MB" variant="secondary" size="small" outline />
                  <BaseBadge text="85%" variant="success" size="small" />
                </div>
              </div>
            </div>
            
            <template #actions>
              <BaseButton size="small" variant="ghost" icon="📖">阅读</BaseButton>
              <BaseButton size="small" variant="ghost" icon="✏️">编辑</BaseButton>
              <BaseButton size="small" variant="ghost" icon="🗑️">删除</BaseButton>
            </template>
          </BaseCard>
        </div>
      </section>
    </div>
  </div>
</template>

<script>
import { ref, reactive } from 'vue'
import { BaseCard, BaseButton, BaseInput, BaseSwitch, BaseBadge } from '../base'

export default {
  name: 'ComponentExamples',
  components: {
    BaseCard,
    BaseButton,
    BaseInput,
    BaseSwitch,
    BaseBadge
  },
  setup() {
    const loading = ref(false)
    
    const formData = reactive({
      name: '',
      email: '',
      password: '',
      bio: ''
    })
    
    const settings = reactive({
      notifications: true,
      darkMode: false,
      autoSave: true,
      premium: false
    })
    
    const passwordError = ref('')
    
    const toggleLoading = () => {
      loading.value = true
      setTimeout(() => {
        loading.value = false
      }, 2000)
    }
    
    const handleBookClick = () => {
      console.log('书籍卡片被点击')
    }
    
    return {
      loading,
      formData,
      settings,
      passwordError,
      toggleLoading,
      handleBookClick
    }
  }
}
</script>

<style scoped>
.component-examples {
  padding: var(--spacing-2xl) 0;
  background: var(--bg-secondary);
  min-height: 100vh;
}

.container h1 {
  color: var(--text-primary);
  font-size: var(--font-3xl);
  font-weight: var(--font-bold);
  margin-bottom: var(--spacing-lg);
}

.container > p {
  color: var(--text-secondary);
  font-size: var(--font-lg);
  margin-bottom: var(--spacing-3xl);
}

.example-section {
  margin-bottom: var(--spacing-3xl);
  padding: var(--spacing-2xl);
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
}

.example-section h2 {
  color: var(--text-primary);
  font-size: var(--font-2xl);
  font-weight: var(--font-semibold);
  margin-bottom: var(--spacing-xl);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--border-color);
}

/* 卡片示例 */
.example-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-xl);
}

/* 按钮示例 */
.button-groups {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2xl);
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.button-group h3 {
  color: var(--text-primary);
  font-size: var(--font-xl);
  font-weight: var(--font-medium);
  margin-bottom: var(--spacing-md);
}

.button-group > div {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
}

/* 输入框示例 */
.input-examples {
  max-width: 500px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
}

/* 开关示例 */
.switch-examples {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
  max-width: 400px;
}

/* 徽章示例 */
.badge-examples {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2xl);
}

.badge-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.badge-group h3 {
  color: var(--text-primary);
  font-size: var(--font-xl);
  font-weight: var(--font-medium);
}

.badge-group > div {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  align-items: center;
}

/* 重构示例 */
.refactor-example {
  max-width: 600px;
}

.book-content {
  display: flex;
  gap: var(--spacing-xl);
}

.book-cover {
  position: relative;
  flex-shrink: 0;
}

.book-cover img {
  width: 120px;
  height: 160px;
  object-fit: cover;
  border-radius: var(--radius-md);
}

.book-cover .base-badge {
  position: absolute;
  top: -4px;
  right: -4px;
}

.book-info {
  flex: 1;
  min-width: 0;
}

.book-info h3 {
  color: var(--text-primary);
  font-size: var(--font-xl);
  font-weight: var(--font-semibold);
  margin-bottom: var(--spacing-sm);
}

.author {
  color: var(--text-secondary);
  font-size: var(--font-md);
  margin-bottom: var(--spacing-md);
}

.description {
  color: var(--text-primary);
  font-size: var(--font-sm);
  line-height: var(--line-height-normal);
  margin-bottom: var(--spacing-lg);
}

.book-meta {
  display: flex;
  gap: var(--spacing-md);
  align-items: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .component-examples {
    padding: var(--spacing-xl) 0;
  }
  
  .example-section {
    padding: var(--spacing-xl);
  }
  
  .example-grid {
    grid-template-columns: 1fr;
  }
  
  .button-group > div {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .book-content {
    flex-direction: column;
    text-align: center;
  }
  
  .book-cover {
    align-self: center;
  }
}
</style>