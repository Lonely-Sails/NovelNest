<template>
  <div class="about-panel">
    <div class="settings-header">
      <h2>关于应用</h2>
      <p>了解 NovelNest 的版本信息和相关资源</p>
    </div>
    
    <div class="about-content">
      <!-- 应用信息 -->
      <div class="app-info-section">
        <div class="app-logo">
          <div class="logo-icon">📚</div>
          <div class="app-details">
            <h1 class="app-name">NovelNest</h1>
            <p class="app-tagline">优雅的桌面小说阅读器</p>
          </div>
        </div>
        
        <div class="version-info">
          <div class="info-grid">
            <div class="info-item">
              <span class="info-label">版本号</span>
              <span class="info-value">{{ appInfo.version }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">构建版本</span>
              <span class="info-value">{{ appInfo.buildNumber }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">发布日期</span>
              <span class="info-value">{{ appInfo.releaseDate }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">平台</span>
              <span class="info-value">{{ appInfo.platform }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 功能特性 -->
      <div class="features-section">
        <h3>主要特性</h3>
        <div class="features-grid">
          <div class="feature-item">
            <div class="feature-icon">📖</div>
            <div class="feature-content">
              <h4>多格式支持</h4>
              <p>支持 TXT、EPUB、PDF 等多种电子书格式</p>
            </div>
          </div>
          <div class="feature-item">
            <div class="feature-icon">🌐</div>
            <div class="feature-content">
              <h4>在线书源</h4>
              <p>通过插件系统支持多个在线小说网站</p>
            </div>
          </div>
          <div class="feature-item">
            <div class="feature-icon">🎨</div>
            <div class="feature-content">
              <h4>个性化阅读</h4>
              <p>丰富的主题和字体设置，打造专属阅读体验</p>
            </div>
          </div>
          <div class="feature-item">
            <div class="feature-icon">📊</div>
            <div class="feature-content">
              <h4>阅读管理</h4>
              <p>智能的阅读进度管理和书签系统</p>
            </div>
          </div>
          <div class="feature-item">
            <div class="feature-icon">⬇️</div>
            <div class="feature-content">
              <h4>批量下载</h4>
              <p>支持批量下载和智能队列管理</p>
            </div>
          </div>
          <div class="feature-item">
            <div class="feature-icon">💾</div>
            <div class="feature-content">
              <h4>数据安全</h4>
              <p>完善的数据备份和恢复机制</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 技术信息 -->
      <div class="tech-section">
        <h3>技术信息</h3>
        <div class="tech-grid">
          <div class="tech-item">
            <span class="tech-label">前端框架</span>
            <span class="tech-value">Vue.js 3</span>
          </div>
          <div class="tech-item">
            <span class="tech-label">桌面框架</span>
            <span class="tech-value">Tauri 2.0</span>
          </div>
          <div class="tech-item">
            <span class="tech-label">后端语言</span>
            <span class="tech-value">Rust</span>
          </div>
          <div class="tech-item">
            <span class="tech-label">数据库</span>
            <span class="tech-value">SQLite</span>
          </div>
          <div class="tech-item">
            <span class="tech-label">构建工具</span>
            <span class="tech-value">Vite</span>
          </div>
          <div class="tech-item">
            <span class="tech-label">状态管理</span>
            <span class="tech-value">Pinia</span>
          </div>
        </div>
      </div>
      
      <!-- 系统信息 */
      <div class="system-section">
        <h3>系统信息</h3>
        <div class="system-grid">
          <div class="system-item">
            <span class="system-label">操作系统</span>
            <span class="system-value">{{ systemInfo.os }}</span>
          </div>
          <div class="system-item">
            <span class="system-label">系统版本</span>
            <span class="system-value">{{ systemInfo.osVersion }}</span>
          </div>
          <div class="system-item">
            <span class="system-label">架构</span>
            <span class="system-value">{{ systemInfo.arch }}</span>
          </div>
          <div class="system-item">
            <span class="system-label">内存</span>
            <span class="system-value">{{ systemInfo.memory }}</span>
          </div>
          <div class="system-item">
            <span class="system-label">数据目录</span>
            <span class="system-value">{{ systemInfo.dataDir }}</span>
          </div>
          <div class="system-item">
            <span class="system-label">配置目录</span>
            <span class="system-value">{{ systemInfo.configDir }}</span>
          </div>
        </div>
      </div>
      
      <!-- 开发团队 -->
      <div class="team-section">
        <h3>开发团队</h3>
        <div class="team-info">
          <div class="team-member">
            <div class="member-avatar">👨‍💻</div>
            <div class="member-details">
              <h4>开发者</h4>
              <p>NovelNest Team</p>
              <p class="member-role">全栈开发 · 产品设计</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 链接和资源 -->
      <div class="links-section">
        <h3>相关链接</h3>
        <div class="links-grid">
          <a href="#" class="link-item" @click.prevent="openLink('homepage')">
            <div class="link-icon">🏠</div>
            <div class="link-content">
              <h4>官方网站</h4>
              <p>访问项目主页</p>
            </div>
          </a>
          <a href="#" class="link-item" @click.prevent="openLink('github')">
            <div class="link-icon">💻</div>
            <div class="link-content">
              <h4>源代码</h4>
              <p>查看 GitHub 仓库</p>
            </div>
          </a>
          <a href="#" class="link-item" @click.prevent="openLink('docs')">
            <div class="link-icon">📖</div>
            <div class="link-content">
              <h4>使用文档</h4>
              <p>查看详细使用说明</p>
            </div>
          </a>
          <a href="#" class="link-item" @click.prevent="openLink('issues')">
            <div class="link-icon">🐛</div>
            <div class="link-content">
              <h4>问题反馈</h4>
              <p>报告 Bug 或建议</p>
            </div>
          </a>
          <a href="#" class="link-item" @click.prevent="openLink('releases')">
            <div class="link-icon">🚀</div>
            <div class="link-content">
              <h4>版本发布</h4>
              <p>查看更新日志</p>
            </div>
          </a>
          <a href="#" class="link-item" @click.prevent="openLink('discussions')">
            <div class="link-icon">💬</div>
            <div class="link-content">
              <h4>社区讨论</h4>
              <p>参与用户交流</p>
            </div>
          </a>
        </div>
      </div>
      
      <!-- 许可证信息 -->
      <div class="license-section">
        <h3>许可证信息</h3>
        <div class="license-content">
          <div class="license-item">
            <h4>开源许可</h4>
            <p>本软件基于 MIT 许可证开源发布</p>
            <button class="btn-secondary" @click="showLicense">
              查看许可证
            </button>
          </div>
          <div class="license-item">
            <h4>第三方组件</h4>
            <p>使用了多个优秀的开源项目</p>
            <button class="btn-secondary" @click="showDependencies">
              查看依赖
            </button>
          </div>
        </div>
      </div>
      
      <!-- 更新检查 -->
      <div class="update-section">
        <h3>版本更新</h3>
        <div class="update-content">
          <div class="update-info">
            <div class="current-version">
              <span>当前版本：{{ appInfo.version }}</span>
              <span class="version-status" :class="updateStatus.class">
                {{ updateStatus.text }}
              </span>
            </div>
            <p class="update-desc">{{ updateStatus.description }}</p>
          </div>
          <div class="update-actions">
            <button 
              class="btn-primary" 
              @click="checkForUpdates" 
              :disabled="checking"
            >
              {{ checking ? '检查中...' : '检查更新' }}
            </button>
            <button 
              v-if="updateAvailable" 
              class="btn-success" 
              @click="downloadUpdate"
            >
              下载更新
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 许可证模态框 -->
    <Modal :show="showLicenseModal" @close="showLicenseModal = false" title="MIT 许可证" size="large">
      <div class="license-text">
        <pre>{{ licenseText }}</pre>
      </div>
    </Modal>
    
    <!-- 依赖信息模态框 -->
    <Modal :show="showDependenciesModal" @close="showDependenciesModal = false" title="第三方依赖" size="large">
      <div class="dependencies-list">
        <div v-for="dep in dependencies" :key="dep.name" class="dependency-item">
          <div class="dep-info">
            <h4>{{ dep.name }}</h4>
            <p>{{ dep.description }}</p>
          </div>
          <div class="dep-meta">
            <span class="dep-version">v{{ dep.version }}</span>
            <span class="dep-license">{{ dep.license }}</span>
          </div>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script>
import { ref, reactive, onMounted } from 'vue'
import { useToast } from '../../composables/useToast'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-shell'
import Modal from '../Modal.vue'

export default {
  name: 'AboutPanel',
  components: {
    Modal
  },
  setup() {
    const { showToast } = useToast()
    
    const checking = ref(false)
    const updateAvailable = ref(false)
    const showLicenseModal = ref(false)
    const showDependenciesModal = ref(false)
    
    // 应用信息
    const appInfo = reactive({
      version: '1.0.0',
      buildNumber: '20240115001',
      releaseDate: '2024-01-15',
      platform: 'macOS'
    })
    
    // 系统信息
    const systemInfo = reactive({
      os: 'macOS',
      osVersion: '14.0',
      arch: 'arm64',
      memory: '16 GB',
      dataDir: '~/Library/Application Support/NovelNest',
      configDir: '~/Library/Preferences/NovelNest'
    })
    
    // 更新状态
    const updateStatus = reactive({
      class: 'up-to-date',
      text: '已是最新版本',
      description: '您正在使用最新版本的 NovelNest'
    })
    
    // 许可证文本
    const licenseText = `MIT License

Copyright (c) 2024 NovelNest Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.`
    
    // 依赖列表
    const dependencies = [
      {
        name: 'Vue.js',
        version: '3.4.0',
        description: '渐进式 JavaScript 框架',
        license: 'MIT'
      },
      {
        name: 'Tauri',
        version: '2.0.0',
        description: '构建跨平台桌面应用的框架',
        license: 'Apache-2.0'
      },
      {
        name: 'Pinia',
        version: '2.1.0',
        description: 'Vue.js 的状态管理库',
        license: 'MIT'
      },
      {
        name: 'Vite',
        version: '5.0.0',
        description: '下一代前端构建工具',
        license: 'MIT'
      },
      {
        name: 'Rust',
        version: '1.75.0',
        description: '系统编程语言',
        license: 'Apache-2.0'
      },
      {
        name: 'SQLite',
        version: '3.44.0',
        description: '轻量级数据库引擎',
        license: 'Public Domain'
      }
    ]
    
    // 初始化
    onMounted(() => {
      loadAppInfo()
      loadSystemInfo()
    })
    
    // 加载应用信息
    const loadAppInfo = async () => {
      try {
        // 这里应该调用后端API获取实际的应用信息
        // const info = await invoke('get_app_info')
        // Object.assign(appInfo, info)
        
        // 检测平台
        const platform = navigator.platform
        if (platform.includes('Mac')) {
          appInfo.platform = 'macOS'
        } else if (platform.includes('Win')) {
          appInfo.platform = 'Windows'
        } else if (platform.includes('Linux')) {
          appInfo.platform = 'Linux'
        }
      } catch (error) {
        console.error('加载应用信息失败:', error)
      }
    }
    
    // 加载系统信息
    const loadSystemInfo = async () => {
      try {
        // 这里应该调用后端API获取实际的系统信息
        // const info = await invoke('get_system_info')
        // Object.assign(systemInfo, info)
        
        // 获取基本信息
        systemInfo.os = navigator.platform
        systemInfo.memory = `${Math.round(navigator.deviceMemory || 8)} GB`
      } catch (error) {
        console.error('加载系统信息失败:', error)
      }
    }
    
    // 打开链接
    const openLink = async (type) => {
      const links = {
        homepage: 'https://novelnest.app',
        github: 'https://github.com/novelnest/novelnest',
        docs: 'https://docs.novelnest.app',
        issues: 'https://github.com/novelnest/novelnest/issues',
        releases: 'https://github.com/novelnest/novelnest/releases',
        discussions: 'https://github.com/novelnest/novelnest/discussions'
      }
      
      const url = links[type]
      if (url) {
        try {
          await open(url)
        } catch (error) {
          console.error('打开链接失败:', error)
          showToast('打开链接失败', 'error')
        }
      }
    }
    
    // 检查更新
    const checkForUpdates = async () => {
      checking.value = true
      try {
        // 这里应该调用后端API检查更新
        // const updateInfo = await invoke('check_for_updates')
        
        // 模拟检查更新
        await new Promise(resolve => setTimeout(resolve, 2000))
        
        // 模拟有更新可用
        const hasUpdate = Math.random() > 0.7
        
        if (hasUpdate) {
          updateAvailable.value = true
          updateStatus.class = 'update-available'
          updateStatus.text = '有新版本可用'
          updateStatus.description = '发现新版本 v1.1.0，包含性能优化和新功能'
          showToast('发现新版本', 'success')
        } else {
          updateStatus.class = 'up-to-date'
          updateStatus.text = '已是最新版本'
          updateStatus.description = '您正在使用最新版本的 NovelNest'
          showToast('已是最新版本', 'info')
        }
      } catch (error) {
        console.error('检查更新失败:', error)
        updateStatus.class = 'check-failed'
        updateStatus.text = '检查失败'
        updateStatus.description = '无法连接到更新服务器，请稍后重试'
        showToast('检查更新失败', 'error')
      } finally {
        checking.value = false
      }
    }
    
    // 下载更新
    const downloadUpdate = async () => {
      try {
        // 这里应该调用后端API下载更新
        // await invoke('download_update')
        showToast('开始下载更新', 'success')
      } catch (error) {
        console.error('下载更新失败:', error)
        showToast('下载更新失败', 'error')
      }
    }
    
    // 显示许可证
    const showLicense = () => {
      showLicenseModal.value = true
    }
    
    // 显示依赖信息
    const showDependencies = () => {
      showDependenciesModal.value = true
    }
    
    return {
      appInfo,
      systemInfo,
      updateStatus,
      checking,
      updateAvailable,
      showLicenseModal,
      showDependenciesModal,
      licenseText,
      dependencies,
      openLink,
      checkForUpdates,
      downloadUpdate,
      showLicense,
      showDependencies
    }
  }
}
</script>

<style scoped>
.about-panel {
  padding: 2rem;
  height: 100%;
  overflow-y: auto;
}

.settings-header {
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.settings-header h2 {
  color: var(--text-primary);
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.settings-header p {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.about-content > div {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.about-content > div h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

/* 应用信息 */
.app-logo {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.logo-icon {
  font-size: 4rem;
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--accent-color);
  border-radius: 16px;
  color: white;
}

.app-name {
  color: var(--text-primary);
  font-size: 2rem;
  font-weight: 700;
  margin-bottom: 0.25rem;
}

.app-tagline {
  color: var(--text-secondary);
  font-size: 1rem;
  margin: 0;
}

.info-grid,
.tech-grid,
.system-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.info-item,
.tech-item,
.system-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.info-label,
.tech-label,
.system-label {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.info-value,
.tech-value,
.system-value {
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.9rem;
}

/* 功能特性 */
.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.feature-item {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.feature-icon {
  font-size: 2rem;
  flex-shrink: 0;
}

.feature-content h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.feature-content p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  line-height: 1.4;
  margin: 0;
}

/* 开发团队 */
.team-member {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.member-avatar {
  font-size: 3rem;
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--accent-color);
  border-radius: 50%;
}

.member-details h4 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.member-details p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin: 0;
}

.member-role {
  font-size: 0.8rem !important;
  opacity: 0.8;
}

/* 相关链接 */
.links-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  text-decoration: none;
  color: inherit;
  transition: all 0.2s ease;
}

.link-item:hover {
  border-color: var(--accent-color);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.link-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
}

.link-content h4 {
  color: var(--text-primary);
  font-size: 0.9rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.link-content p {
  color: var(--text-secondary);
  font-size: 0.8rem;
  margin: 0;
}

/* 许可证信息 */
.license-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.license-item {
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.license-item h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.license-item p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 1rem;
}

/* 版本更新 */
.update-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
}

.update-info {
  flex: 1;
}

.current-version {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.version-status {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.version-status.up-to-date {
  background: rgba(40, 167, 69, 0.1);
  color: #28a745;
}

.version-status.update-available {
  background: rgba(255, 193, 7, 0.1);
  color: #ffc107;
}

.version-status.check-failed {
  background: rgba(220, 53, 69, 0.1);
  color: #dc3545;
}

.update-desc {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin: 0;
}

.update-actions {
  display: flex;
  gap: 1rem;
}

/* 模态框内容 */
.license-text {
  max-height: 400px;
  overflow-y: auto;
  font-family: monospace;
  font-size: 0.8rem;
  line-height: 1.4;
  color: var(--text-primary);
  background: var(--bg-secondary);
  padding: 1rem;
  border-radius: 4px;
}

.dependencies-list {
  max-height: 400px;
  overflow-y: auto;
}

.dependency-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  margin-bottom: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.dep-info h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.dep-info p {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0;
}

.dep-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.25rem;
}

.dep-version {
  color: var(--accent-color);
  font-weight: 500;
  font-size: 0.9rem;
}

.dep-license {
  color: var(--text-secondary);
  font-size: 0.8rem;
}

/* 按钮样式 */
.btn-primary,
.btn-secondary,
.btn-success {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--accent-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background: var(--border-color);
}

.btn-success {
  background: #28a745;
  color: white;
}

.btn-success:hover {
  background: #218838;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .about-panel {
    padding: 1rem;
  }
  
  .app-logo {
    flex-direction: column;
    text-align: center;
  }
  
  .info-grid,
  .tech-grid,
  .system-grid {
    grid-template-columns: 1fr;
  }
  
  .features-grid,
  .links-grid {
    grid-template-columns: 1fr;
  }
  
  .update-content {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .update-actions {
    width: 100%;
    justify-content: flex-start;
  }
  
  .current-version {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .dependency-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .dep-meta {
    align-items: flex-start;
  }
}
</style>