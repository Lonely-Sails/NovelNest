<script setup>
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import ThemeToggle from './components/ThemeToggle.vue'
import Toast from './components/Toast.vue'

const route = useRoute()

// 侧边栏展开状态，默认收起
const isExpanded = ref(false)

// 导航菜单项
const menuItems = [
  { name: '首页', path: '/', icon: '🏠' },
  { name: '图书库', path: '/library', icon: '📚' },
  { name: '在线下载', path: '/downloads', icon: '🌐' },
  { name: '插件管理', path: '/plugins', icon: '🔌' },
  { name: '设置', path: '/settings', icon: '⚙️' }
]

// 判断当前路由是否激活
const isActive = (path) => route.path === path

// 侧边栏交互处理
const handleMouseEnter = () => {
  isExpanded.value = true
}

const handleMouseLeave = () => {
  isExpanded.value = false
}
</script>

<template>
  <div class="app">
    <!-- 侧边导航栏 -->
    <nav class="sidebar" :class="{ expanded: isExpanded }" @mouseenter="handleMouseEnter"
      @mouseleave="handleMouseLeave">
      <div class="sidebar-content">
        <div class="logo">
          <div class="logo-content">
            <span class="logo-icon">📖</span>
            <h2 class="logo-text">NovelNest</h2>
          </div>
          <ThemeToggle class="theme-toggle" />
        </div>
        <ul class="nav-menu">
          <li v-for="item in menuItems" :key="item.path">
            <router-link :to="item.path" :class="['nav-link', { active: isActive(item.path) }]"
              :title="!isExpanded ? item.name : ''">
              <span class="nav-icon">{{ item.icon }}</span>
              <span class="nav-text">{{ item.name }}</span>
            </router-link>
          </li>
        </ul>
      </div>
    </nav>

    <!-- 主内容区域 -->
    <main class="main-content">
      <router-view />
    </main>

    <!-- 全局 Toast 组件 -->
    <Toast />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  font-weight: 400;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;

  /* 浅色主题 */
  --bg-primary: #ffffff;
  --bg-secondary: #fafafa;
  --text-primary: #333333;
  --text-secondary: #666666;
  --text-muted: #999999;
  --border-color: #e0e0e0;
  --border-color-light: #f0f0f0;
  --primary-color: #007bff;
  --primary-color-dark: #0056b3;
  --primary-color-light: rgba(0, 123, 255, 0.1);
  --success-color: #28a745;
  --warning-color: #ffc107;
  --error-color: #dc3545;
  --accent-color: #007bff;
  --accent-hover: #0056b3;
  --sidebar-bg: #f5f5f5;
  --sidebar-text: #333333;
  --sidebar-hover: #eeeeee;
  --sidebar-active: #007bff;
}

/* 暗色主题 */
.theme-dark {
  --bg-primary: #1e1e1e;
  --bg-secondary: #2d2d2d;
  --text-primary: #ffffff;
  --text-secondary: #cccccc;
  --text-muted: #999999;
  --border-color: #404040;
  --border-color-light: #353535;
  --primary-color: #1e90ff;
  --primary-color-dark: #1c7ed6;
  --primary-color-light: rgba(30, 144, 255, 0.1);
  --success-color: #28a745;
  --warning-color: #ffc107;
  --error-color: #dc3545;
  --accent-color: #1e90ff;
  --accent-hover: #1c7ed6;
  --sidebar-bg: #252525;
  --sidebar-text: #ffffff;
  --sidebar-hover: #333333;
  --sidebar-active: #1e90ff;
}

body {
  margin: 0;
  min-height: 100vh;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  transition: all 0.2s ease;
}

#app {
  min-height: 100vh;
}
</style>

<style scoped>
.app {
  display: flex;
  min-height: 100vh;
}

.sidebar {
  width: 60px;
  background: var(--sidebar-bg);
  color: var(--sidebar-text);
  position: fixed;
  height: 100vh;
  overflow: hidden;
  border-right: 1px solid var(--border-color);
  z-index: 1000;
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.sidebar.expanded {
  width: 240px;
  box-shadow: 2px 0 15px rgba(0, 0, 0, 0.15);
}

.sidebar-content {
  width: 240px;
  height: 100%;
  overflow-y: auto;
}

.logo {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 70px;
}

.logo-content {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-width: 0;
}

.logo-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.logo-text {
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
  white-space: nowrap;
}

.theme-toggle {
  flex-shrink: 0;
}

.nav-menu {
  list-style: none;
  padding: 1rem 0;
  margin: 0;
}

.nav-menu li {
  margin: 0;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  color: var(--text-secondary);
  text-decoration: none;
  transition: background-color 0.2s ease, color 0.2s ease;
  position: relative;
  min-width: 0;
}

.nav-link:hover {
  background-color: var(--sidebar-hover);
  color: var(--text-primary);
}

.nav-link.active {
  background-color: var(--accent-color);
  color: white;
}

.nav-icon {
  font-size: 1.1rem;
  width: 20px;
  text-align: center;
  flex-shrink: 0;
}

.nav-text {
  font-weight: 500;
  font-size: 0.9rem;
  white-space: nowrap;
  opacity: 0;
  transform: translateX(20px);
  transition: opacity 0.5s cubic-bezier(0.4, 0, 0.2, 1), transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.sidebar.expanded .nav-text {
  opacity: 1;
  transform: translateX(0);
}

/* 为不同的导航项添加渐进延迟 */
.sidebar.expanded .nav-menu li:nth-child(1) .nav-text {
  transition-delay: 0.1s;
}

.sidebar.expanded .nav-menu li:nth-child(2) .nav-text {
  transition-delay: 0.15s;
}

.sidebar.expanded .nav-menu li:nth-child(3) .nav-text {
  transition-delay: 0.2s;
}

.sidebar.expanded .nav-menu li:nth-child(4) .nav-text {
  transition-delay: 0.25s;
}

.sidebar.expanded .nav-menu li:nth-child(5) .nav-text {
  transition-delay: 0.3s;
}

.main-content {
  flex: 1;
  margin-left: 60px;
  min-height: 100vh;
  background: var(--bg-secondary);
  transition: margin-left 0.3s ease;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar {
    width: 50px;
  }

  .sidebar.expanded {
    width: 200px;
  }

  .main-content {
    margin-left: 50px;
  }
}

@media (max-width: 640px) {
  .sidebar {
    width: 100%;
    height: auto;
    position: relative;
    box-shadow: none;
  }

  .sidebar.expanded {
    width: 100%;
  }

  .main-content {
    margin-left: 0;
  }

  .nav-menu {
    display: flex;
    overflow-x: auto;
    padding: 0.5rem;
  }

  .nav-menu li {
    flex-shrink: 0;
  }

  .nav-link {
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.75rem;
    text-align: center;
    min-width: 80px;
  }

  .nav-text {
    font-size: 0.8rem;
  }
}
</style>
