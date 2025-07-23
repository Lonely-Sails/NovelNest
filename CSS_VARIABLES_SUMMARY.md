# CSS 变量统一工作总结

## 完成的工作

### 1. 创建统一的 CSS 变量系统
- **文件位置**: `src/styles/variables.css`
- **包含内容**:
  - 颜色系统（主色、辅助色、状态色等）
  - 文本颜色（主要、次要、静音、禁用）
  - 背景颜色（主要、次要、第三级）
  - 边框颜色（普通、浅色、深色）
  - 侧边栏专用颜色
  - 阴影系统（小、中、大、超大）
  - 圆角系统（小、中、大、超大、完全圆角）
  - 间距系统（xs 到 3xl）
  - 字体大小系统（xs 到 3xl）
  - 字体权重（普通、中等、半粗、粗体）
  - 行高（紧密、普通、宽松）
  - 过渡动画（快、普通、慢）
  - Z-index 层级系统
  - 阅读器主题颜色

### 2. 主题支持
- **浅色主题**: 默认的明亮主题
- **暗色主题**: `.theme-dark` 类
- **紧凑模式**: `.compact-mode` 类
- **高对比度模式**: `.high-contrast` 类
- **动画偏好**: 支持 `prefers-reduced-motion` 媒体查询

### 3. 更新的组件文件
以下组件已经从硬编码颜色值更新为使用 CSS 变量：

#### 核心组件
- `src/App.vue` - 移除重复的 CSS 变量定义，使用统一变量
- `src/components/SearchBar.vue` - 替换所有硬编码颜色
- `src/components/Modal.vue` - 使用 CSS 变量和 z-index 系统
- `src/components/Toast.vue` - 使用阴影和 z-index 变量
- `src/components/BookCard.vue` - 使用圆角和阴影变量
- `src/components/Loading.vue` - 替换硬编码颜色
- `src/components/base/BaseCard.vue` - 使用主色调变量
- `src/components/base/BaseBadge.vue` - 使用信息色变量

#### 设置组件
- `src/components/settings/ThemeSettings.vue` - 使用阴影和主色调变量

### 4. 创建基础样式文件
- **文件位置**: `src/styles/base.css`
- **包含内容**:
  - 全局重置样式
  - 基础元素样式（链接、按钮、输入框等）
  - 滚动条样式
  - 选择文本样式
  - 焦点样式
  - 表格、代码、引用等样式
  - 打印样式

### 5. 工具类
添加了常用的工具类：
- `.container` - 响应式容器
- `.sr-only` - 屏幕阅读器专用
- `.truncate` - 文本截断
- `.line-clamp-2` / `.line-clamp-3` - 多行文本截断

## CSS 变量命名规范

### 颜色变量
- `--primary-color` - 主色调
- `--primary-hover` - 主色调悬停状态
- `--primary-light` - 主色调浅色版本
- `--text-primary` - 主要文本颜色
- `--text-secondary` - 次要文本颜色
- `--bg-primary` - 主要背景色
- `--border-color` - 边框颜色

### 尺寸变量
- `--spacing-{size}` - 间距（xs, sm, md, lg, xl, 2xl, 3xl）
- `--font-{size}` - 字体大小（xs, sm, md, lg, xl, 2xl, 3xl）
- `--radius-{size}` - 圆角（sm, md, lg, xl, full）
- `--shadow-{size}` - 阴影（sm, md, lg, xl）

### 功能变量
- `--transition-{speed}` - 过渡动画（fast, normal, slow）
- `--z-{layer}` - Z-index 层级（dropdown, modal, toast 等）

## 主题切换机制

### 实现方式
1. 通过在 `<body>` 或根元素上添加主题类来切换主题
2. 每个主题类重新定义相应的 CSS 变量值
3. 所有组件自动继承新的变量值

### 支持的主题
- 默认浅色主题（无需额外类）
- 暗色主题（`.theme-dark`）
- 紧凑模式（`.compact-mode`）
- 高对比度模式（`.high-contrast`）

## 使用建议

### 在组件中使用 CSS 变量
```css
.my-component {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-normal);
}
```

### 响应式设计
```css
.responsive-component {
  padding: var(--spacing-lg);
}

@media (max-width: 768px) {
  .responsive-component {
    padding: var(--spacing-md);
  }
}
```

### 主题适配
```css
.theme-aware-component {
  background: var(--bg-primary);
  color: var(--text-primary);
}

/* 不需要额外的 .theme-dark 样式，变量会自动切换 */
```

## 优势

1. **一致性**: 所有组件使用相同的设计令牌
2. **可维护性**: 修改一个变量值影响所有使用该变量的地方
3. **主题支持**: 轻松实现多主题切换
4. **响应式**: 支持不同屏幕尺寸的适配
5. **可访问性**: 支持高对比度和动画偏好设置
6. **性能**: CSS 变量的原生支持，无需 JavaScript 处理

## 后续建议

1. **扩展主题**: 可以添加更多主题变体（如护眼模式、高对比度等）
2. **组件库**: 基于这套变量系统构建完整的组件库
3. **设计系统**: 将这套变量系统文档化为完整的设计系统
4. **自动化**: 可以考虑使用工具自动检测和替换硬编码颜色值
5. **测试**: 添加主题切换的自动化测试

## 文件结构

```
src/
├── styles/
│   ├── variables.css    # CSS 变量定义
│   └── base.css        # 基础样式
├── components/
│   ├── base/           # 基础组件（已更新）
│   └── ...            # 其他组件（已更新）
└── main.js            # 样式文件导入
```

这套 CSS 变量系统为项目提供了一个坚实的样式基础，支持主题切换、响应式设计和可访问性需求。