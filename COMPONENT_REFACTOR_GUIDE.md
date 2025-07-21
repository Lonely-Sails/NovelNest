# 基础组件重构指南

## 概述

为了实现界面风格统一，我们创建了一套可复用的基础组件。这些组件遵循统一的设计规范，提供一致的用户体验。

## 基础组件列表

### 1. BaseCard - 卡片容器
统一的卡片容器组件，用于包装内容区域。

**主要特性：**
- 支持多种变体（default, primary, secondary, success, warning, error）
- 可配置悬停效果、选中状态、紧凑模式
- 支持头部、内容、底部插槽
- 内置操作按钮区域

**使用示例：**
```vue
<BaseCard title="卡片标题" variant="primary" hoverable>
  <p>卡片内容</p>
  <template #actions>
    <BaseButton size="small">操作</BaseButton>
  </template>
</BaseCard>
```

### 2. BaseButton - 按钮组件
统一的按钮组件，支持多种样式和状态。

**主要特性：**
- 8种变体样式（primary, secondary, success, warning, error, outline, ghost, link）
- 3种尺寸（small, medium, large）
- 支持加载状态、禁用状态、块级按钮
- 支持前后置图标

**使用示例：**
```vue
<BaseButton variant="primary" size="medium" :loading="loading" icon="📖">
  阅读书籍
</BaseButton>
```

### 3. BaseInput - 输入框组件
统一的输入框组件，支持多种输入类型。

**主要特性：**
- 支持文本、邮箱、密码、文本域等类型
- 3种尺寸配置
- 支持前后置图标、清除按钮
- 内置验证状态显示（错误、成功、帮助文本）

**使用示例：**
```vue
<BaseInput
  v-model="email"
  type="email"
  label="邮箱地址"
  placeholder="请输入邮箱"
  prefix-icon="📧"
  clearable
  :error-message="emailError"
/>
```

### 4. BaseSwitch - 开关组件
统一的开关切换组件。

**主要特性：**
- 3种尺寸配置
- 支持图标显示
- 支持标签和描述文本
- 禁用状态支持

**使用示例：**
```vue
<BaseSwitch
  v-model="darkMode"
  label="暗色模式"
  description="启用暗色主题"
  show-icon
/>
```

### 5. BaseBadge - 徽章组件
统一的徽章标签组件。

**主要特性：**
- 7种变体样式
- 支持轮廓样式、圆角样式
- 支持点状徽章
- 支持图标

**使用示例：**
```vue
<BaseBadge text="新功能" variant="primary" icon="✨" rounded />
<BaseBadge dot variant="success" />
```

## 设计令牌系统

我们使用CSS变量定义了统一的设计令牌，确保所有组件使用一致的颜色、间距、字体等样式。

### 主要变量类别：
- **颜色系统**：主色、辅助色、状态色、文本色、背景色
- **间距系统**：xs, sm, md, lg, xl, 2xl, 3xl
- **字体系统**：大小、权重、行高
- **圆角系统**：sm, md, lg, xl, full
- **阴影系统**：sm, md, lg, xl
- **动画系统**：过渡时间和缓动函数

### 主题支持：
- 明亮主题（默认）
- 暗色主题（.theme-dark）
- 紧凑模式（.compact-mode）
- 高对比度模式（.high-contrast）

## 重构现有组件

### 重构步骤：

1. **分析现有组件**
   - 识别可复用的UI模式
   - 确定组件的核心功能

2. **选择合适的基础组件**
   - 根据功能选择对应的基础组件
   - 确定需要的属性和插槽

3. **迁移样式**
   - 移除自定义样式，使用基础组件的内置样式
   - 使用CSS变量替代硬编码的颜色和尺寸

4. **测试功能**
   - 确保所有功能正常工作
   - 验证响应式设计
   - 测试主题切换

### 重构示例：

**重构前的书籍卡片：**
```vue
<div class="book-card">
  <div class="book-header">
    <h3>{{ book.title }}</h3>
    <div class="book-actions">
      <button class="action-btn">阅读</button>
    </div>
  </div>
  <div class="book-content">
    <!-- 内容 -->
  </div>
</div>
```

**重构后的书籍卡片：**
```vue
<BaseCard :title="book.title" hoverable clickable>
  <!-- 内容 -->
  <template #actions>
    <BaseButton size="small" icon="📖">阅读</BaseButton>
  </template>
</BaseCard>
```

## 使用指南

### 1. 引入组件
```javascript
import { BaseCard, BaseButton, BaseInput, BaseSwitch, BaseBadge } from '@/components/base'
```

### 2. 注册组件
```javascript
export default {
  components: {
    BaseCard,
    BaseButton,
    BaseInput,
    BaseSwitch,
    BaseBadge
  }
}
```

### 3. 使用CSS变量
```css
.custom-component {
  color: var(--text-primary);
  background: var(--bg-primary);
  padding: var(--spacing-lg);
  border-radius: var(--radius-md);
  transition: var(--transition-normal);
}
```

## 最佳实践

### 1. 保持一致性
- 优先使用基础组件而不是自定义组件
- 使用统一的设计令牌
- 遵循组件的API设计

### 2. 响应式设计
- 使用组件内置的响应式特性
- 测试不同屏幕尺寸下的表现
- 考虑移动端的交互体验

### 3. 可访问性
- 使用语义化的HTML结构
- 提供适当的ARIA标签
- 确保键盘导航功能

### 4. 性能优化
- 按需引入组件
- 避免不必要的重渲染
- 合理使用v-show和v-if

## 迁移计划

### 阶段1：核心组件重构
- [ ] BookCard.vue → 使用BaseCard
- [ ] PluginCard.vue → 使用BaseCard
- [ ] 设置面板中的按钮 → 使用BaseButton

### 阶段2：表单组件重构
- [ ] 搜索输入框 → 使用BaseInput
- [ ] 设置开关 → 使用BaseSwitch
- [ ] 状态标签 → 使用BaseBadge

### 阶段3：样式统一
- [ ] 更新CSS变量使用
- [ ] 移除重复的样式代码
- [ ] 优化主题切换功能

## 注意事项

1. **向后兼容**：重构时确保不破坏现有功能
2. **渐进式迁移**：可以逐步替换，不需要一次性全部重构
3. **测试覆盖**：每次重构后都要进行充分测试
4. **文档更新**：及时更新相关文档和注释

## 支持和反馈

如果在使用基础组件过程中遇到问题或有改进建议，请：
1. 查看组件示例文件：`src/components/examples/ComponentExamples.vue`
2. 参考设计令牌定义：`src/styles/variables.css`
3. 提交问题或建议到项目仓库

通过使用这套基础组件系统，我们可以：
- 提高开发效率
- 确保界面一致性
- 简化维护工作
- 提升用户体验