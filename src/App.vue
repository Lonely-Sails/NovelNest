<script setup>
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from './composables/useToast'
import router from './router'
import ThemeToggle from './components/ThemeToggle.vue'
// Toast组件已全局注册，无需导入

const route = useRoute()
// toastState和hideToast在模板中使用，不是未使用的变量
const { toastState, hideToast } = useToast()

// 导航菜单项
const menuItems = [
  { name: '首页', path: '/', icon: '🏠', type: 'route' },
  { name: '图书库', path: '/library', icon: '📚', type: 'route' },
  { name: '在线下载', path: '/downloads', icon: '🌐', type: 'route' },
  { name: '插件管理', path: '/plugins', icon: '🔌', type: 'route' },
  { name: '设置', path: '/settings', icon: '⚙️', type: 'window' }
]

const sidebar = ref(true);
// 判断当前路由是否激活
const isActive = (path) => route.path === path

// 处理菜单项点击
const handleMenuClick = async (item) => {
  if (item.type === 'window' && item.path === '/settings') {
    try {
      await invoke('open_window', { router: 'settings', title: '设置' })
    } catch (error) {
      console.error('打开设置窗口失败:', error)
    }
  }
}

onMounted(() => {
  const url = new URL(window.location.href)
  const routerName = url.searchParams.get('router')
  if (routerName) {
    sidebar.value = false
    router.push(routerName)
  }
})
</script>

<template>
  <div class="app">
    <!-- 侧边导航栏 -->
    <nav class="sidebar" v-if="sidebar">
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
            <!-- 路由链接 -->
            <router-link v-if="item.type === 'route'" :title="item.name" :to="item.path"
              :class="['nav-link', { active: isActive(item.path) }]">
              <span class="nav-icon">{{ item.icon }}</span>
              <span class="nav-text">{{ item.name }}</span>
            </router-link>

            <!-- 窗口按钮 -->
            <button v-else-if="item.type === 'window'" :title="item.name" :class="['nav-link', 'nav-button']"
              @click="handleMenuClick(item)">
              <span class="nav-icon">{{ item.icon }}</span>
              <span class="nav-text">{{ item.name }}</span>
            </button>
          </li>
        </ul>
      </div>
    </nav>

    <!-- 主内容区域 -->
    <main class="main-content">
      <router-view />
    </main>

    <!-- 全局 Toast 组件 -->
    <Toast 
      v-if="toastState.show" 
      :message="toastState.message" 
      :type="toastState.type" 
      :duration="toastState.duration"
      @close="hideToast" 
    />
  </div>
</template>

<style scoped>
.app {
  display: flex;
}

.sidebar {
  width: 240px;
  box-shadow: var(--shadow-md);
  color: var(--sidebar-text);
  height: 100vh;
  overflow: hidden;
  border-right: 1px solid var(--border-color);
  z-index: 1000;
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: var(--shadow-sm);
  background: var(--sidebar-bg);
}

.sidebar-content {
  width: 240px;
  height: 100%;
  overflow-y: auto;
}

.logo {
  padding: 1.75rem;
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
  transition: all var(--transition-normal);
  position: relative;
  min-width: 0;
}

.nav-link:hover {
  background-color: var(--sidebar-hover);
  color: var(--text-primary);
}

.nav-link.active {
  color: white;
  background-color: var(--accent-color);
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
}

.nav-button {
  background: none;
  border: none;
  cursor: pointer;
  font-family: inherit;
  width: 100%;
  float: bottom;
}

.main-content {
  flex: 1;
  max-height: 100vh;
  overflow-x: auto;
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
