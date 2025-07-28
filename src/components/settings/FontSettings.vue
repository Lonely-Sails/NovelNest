<template>
  <div class="font-settings">
    <div class="settings-header">
      <h2>字体设置</h2>
      <p>自定义阅读文本的字体样式和排版</p>
    </div>
    
    <div class="settings-sections">
      <!-- 字体选择 -->
      <div class="setting-section">
        <h3>字体系列</h3>
        <div class="font-grid">
          <div 
            v-for="font in fontFamilies" 
            :key="font.value"
            :class="['font-card', { active: localSettings.fontFamily === font.value }]"
            @click="selectFont(font.value)"
          >
            <div class="font-preview" :style="{ fontFamily: font.style }">
              <div class="preview-text">春江潮水连海平</div>
              <div class="preview-text-en">The quick brown fox</div>
            </div>
            <div class="font-info">
              <h4>{{ font.name }}</h4>
              <p>{{ font.description }}</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 自定义字体 -->
      <div class="setting-section">
        <h3>自定义字体</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">启用自定义字体</span>
            <span class="label-desc">使用本地安装的字体文件</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.useCustomFont"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div v-if="localSettings.useCustomFont" class="custom-font-section">
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">字体文件路径</span>
              <span class="label-desc">选择 TTF 或 OTF 字体文件</span>
            </label>
            <div class="font-file-input">
              <input 
                type="text" 
                v-model="localSettings.customFontPath" 
                class="setting-input"
                placeholder="选择字体文件..."
                readonly
              >
              <button class="btn-secondary" @click="selectFontFile">
                选择文件
              </button>
            </div>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span class="label-text">字体名称</span>
              <span class="label-desc">自定义字体的显示名称</span>
            </label>
            <input 
              type="text" 
              v-model="localSettings.customFontName" 
              class="setting-input"
              placeholder="输入字体名称..."
              @input="updateSettings"
            >
          </div>
        </div>
      </div>
      
      <!-- 字体样式 -->
      <div class="setting-section">
        <h3>字体样式</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">字体粗细</span>
            <span class="label-desc">调整文字的粗细程度</span>
          </label>
          <BaseSelect
            v-model="localSettings.fontWeight"
            :options="fontWeightOptions"
            size="small"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">字体样式</span>
            <span class="label-desc">选择字体的倾斜样式</span>
          </label>
          <BaseSelect
            v-model="localSettings.fontStyle"
            :options="fontStyleOptions"
            size="small"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">字符间距</span>
            <span class="label-desc">调整字符之间的间距</span>
          </label>
          <div class="letter-spacing-control">
            <input 
              type="range" 
              v-model.number="localSettings.letterSpacing" 
              min="-2"
              max="5"
              step="0.1"
              class="spacing-slider"
              @input="updateSettings"
            >
            <span class="spacing-value">{{ localSettings.letterSpacing.toFixed(1) }}px</span>
          </div>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">词间距</span>
            <span class="label-desc">调整单词之间的间距</span>
          </label>
          <div class="word-spacing-control">
            <input 
              type="range" 
              v-model.number="localSettings.wordSpacing" 
              min="-5"
              max="10"
              step="0.5"
              class="spacing-slider"
              @input="updateSettings"
            >
            <span class="spacing-value">{{ localSettings.wordSpacing.toFixed(1) }}px</span>
          </div>
        </div>
      </div>
      
      <!-- 文本渲染 -->
      <div class="setting-section">
        <h3>文本渲染</h3>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">字体平滑</span>
            <span class="label-desc">启用字体抗锯齿渲染</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.fontSmoothing"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">子像素渲染</span>
            <span class="label-desc">启用子像素级别的字体渲染</span>
          </label>
          <label class="setting-switch">
            <input 
              type="checkbox" 
              v-model="localSettings.subpixelRendering"
              @change="updateSettings"
            >
            <span class="switch-slider"></span>
          </label>
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">文本对齐</span>
            <span class="label-desc">设置段落文本的对齐方式</span>
          </label>
          <BaseSelect
            v-model="localSettings.textAlign"
            :options="textAlignOptions"
            size="small"
            @change="updateSettings"
          />
        </div>
        
        <div class="setting-item">
          <label class="setting-label">
            <span class="label-text">段落缩进</span>
            <span class="label-desc">设置段落首行缩进</span>
          </label>
          <div class="indent-control">
            <input 
              type="range" 
              v-model.number="localSettings.textIndent" 
              min="0"
              max="4"
              step="0.5"
              class="indent-slider"
              @input="updateSettings"
            >
            <span class="indent-value">{{ localSettings.textIndent }}em</span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 字体预览 -->
    <div class="preview-section">
      <h3>字体效果预览</h3>
      <div class="font-preview-area" :style="previewStyles">
        <h4 class="preview-title">字体预览</h4>
        <p class="preview-paragraph">
          这是一段中文预览文本，用于展示当前字体设置的效果。通过调整字体系列、大小、粗细、间距等参数，您可以找到最适合自己阅读习惯的字体配置。
        </p>
        <p class="preview-paragraph">
          This is an English preview text to demonstrate the current font settings. You can adjust font family, size, weight, spacing and other parameters to find the most suitable font configuration for your reading habits.
        </p>
        <p class="preview-paragraph">
          数字测试：1234567890<br>
          标点符号：，。！？；：""''（）【】
        </p>
      </div>
    </div>
    
    <!-- 操作按钮 -->
    <div class="settings-actions">
      <button class="btn-secondary" @click="resetToDefaults">
        恢复默认字体
      </button>
      <button class="btn-primary" @click="saveSettings" :disabled="saving">
        {{ saving ? '保存中...' : '保存设置' }}
      </button>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import { useSettingsStore } from '../../stores/settingsStore'
import { useToast } from '../../composables/useToast'
import { open } from '@tauri-apps/plugin-dialog'

export default {
  name: 'FontSettings',
  setup() {
    const settingsStore = useSettingsStore()
    const { showToast } = useToast()
    const saving = ref(false)
    
    // 本地设置状态
    const localSettings = reactive({
      fontFamily: 'system',
      useCustomFont: false,
      customFontPath: '',
      customFontName: '',
      fontWeight: '400',
      fontStyle: 'normal',
      letterSpacing: 0,
      wordSpacing: 0,
      fontSmoothing: true,
      subpixelRendering: true,
      textAlign: 'justify',
      textIndent: 2
    })
    
    // 字体粗细选项
    const fontWeightOptions = [
      { label: '细体 (Light)', value: '300' },
      { label: '正常 (Normal)', value: '400' },
      { label: '中等 (Medium)', value: '500' },
      { label: '半粗 (Semi Bold)', value: '600' },
      { label: '粗体 (Bold)', value: '700' }
    ]
    
    // 字体样式选项
    const fontStyleOptions = [
      { label: '正常', value: 'normal' },
      { label: '斜体', value: 'italic' },
      { label: '倾斜', value: 'oblique' }
    ]
    
    // 文本对齐选项
    const textAlignOptions = [
      { label: '左对齐', value: 'left' },
      { label: '居中对齐', value: 'center' },
      { label: '右对齐', value: 'right' },
      { label: '两端对齐', value: 'justify' }
    ]
    
    // 字体系列选项
    const fontFamilies = [
      {
        name: '系统默认',
        value: 'system',
        style: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
        description: '使用系统默认字体，兼容性最佳'
      },
      {
        name: '宋体',
        value: 'serif',
        style: '"Times New Roman", Times, serif, "SimSun"',
        description: '经典的衬线字体，适合长时间阅读'
      },
      {
        name: '黑体',
        value: 'sans-serif',
        style: 'Arial, Helvetica, sans-serif, "SimHei"',
        description: '现代的无衬线字体，清晰易读'
      },
      {
        name: '等宽字体',
        value: 'monospace',
        style: '"Courier New", Courier, monospace',
        description: '等宽字体，适合代码和特殊格式'
      },
      {
        name: '微软雅黑',
        value: 'microsoft-yahei',
        style: '"Microsoft YaHei", "微软雅黑", sans-serif',
        description: 'Windows 系统优化的中文字体'
      },
      {
        name: '苹方',
        value: 'pingfang',
        style: '"PingFang SC", "苹方-简", sans-serif',
        description: 'macOS 系统优化的中文字体'
      },
      {
        name: '思源黑体',
        value: 'source-han-sans',
        style: '"Source Han Sans SC", "思源黑体", sans-serif',
        description: '开源的高质量中文字体'
      },
      {
        name: '文泉驿',
        value: 'wenquanyi',
        style: '"WenQuanYi Micro Hei", "文泉驿微米黑", sans-serif',
        description: '开源的中文字体，Linux 系统常用'
      }
    ]
    
    // 预览样式
    const previewStyles = computed(() => {
      const selectedFont = fontFamilies.find(f => f.value === localSettings.fontFamily)
      let fontFamily = selectedFont ? selectedFont.style : 'system'
      
      if (localSettings.useCustomFont && localSettings.customFontName) {
        fontFamily = `"${localSettings.customFontName}", ${fontFamily}`
      }
      
      return {
        fontFamily,
        fontWeight: localSettings.fontWeight,
        fontStyle: localSettings.fontStyle,
        letterSpacing: `${localSettings.letterSpacing}px`,
        wordSpacing: `${localSettings.wordSpacing}px`,
        textAlign: localSettings.textAlign,
        textIndent: `${localSettings.textIndent}em`,
        WebkitFontSmoothing: localSettings.fontSmoothing ? 'antialiased' : 'auto',
        MozOsxFontSmoothing: localSettings.fontSmoothing ? 'grayscale' : 'auto',
        fontOpticalSizing: localSettings.subpixelRendering ? 'auto' : 'none'
      }
    })
    
    // 初始化设置
    onMounted(() => {
      loadSettings()
    })
    
    // 加载设置
    const loadSettings = () => {
      const readerSettings = settingsStore.reader
      Object.keys(localSettings).forEach(key => {
        if (readerSettings[key] !== undefined) {
          localSettings[key] = readerSettings[key]
        }
      })
    }
    
    // 更新设置
    const updateSettings = () => {
      settingsStore.updateReaderSettings(localSettings)
    }
    
    // 选择字体
    const selectFont = (fontValue) => {
      localSettings.fontFamily = fontValue
      updateSettings()
    }
    
    // 选择字体文件
    const selectFontFile = async () => {
      try {
        const selected = await open({
          filters: [
            {
              name: '字体文件',
              extensions: ['ttf', 'otf', 'woff', 'woff2']
            }
          ],
          title: '选择字体文件'
        })
        
        if (selected) {
          localSettings.customFontPath = selected
          // 从文件路径提取字体名称
          const fileName = selected.split('/').pop().split('\\').pop()
          const fontName = fileName.replace(/\.(ttf|otf|woff|woff2)$/i, '')
          localSettings.customFontName = fontName
          updateSettings()
          showToast('字体文件已选择', 'success')
        }
      } catch (error) {
        console.error('选择字体文件失败:', error)
        showToast('选择字体文件失败', 'error')
      }
    }
    
    // 保存设置
    const saveSettings = async () => {
      saving.value = true
      try {
        await settingsStore.saveToBackend()
        showToast('字体设置已保存', 'success')
      } catch (error) {
        console.error('保存字体设置失败:', error)
        showToast('保存字体设置失败', 'error')
      } finally {
        saving.value = false
      }
    }
    
    // 恢复默认设置
    const resetToDefaults = () => {
      if (confirm('确定要恢复默认字体设置吗？')) {
        Object.assign(localSettings, {
          fontFamily: 'system',
          useCustomFont: false,
          customFontPath: '',
          customFontName: '',
          fontWeight: '400',
          fontStyle: 'normal',
          letterSpacing: 0,
          wordSpacing: 0,
          fontSmoothing: true,
          subpixelRendering: true,
          textAlign: 'justify',
          textIndent: 2
        })
        updateSettings()
        showToast('已恢复默认字体设置', 'success')
      }
    }
    
    return {
      localSettings,
      saving,
      fontFamilies,
      fontWeightOptions,
      fontStyleOptions,
      textAlignOptions,
      previewStyles,
      updateSettings,
      selectFont,
      selectFontFile,
      saveSettings,
      resetToDefaults
    }
  }
}
</script>

<style scoped>
.font-settings {
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

.settings-sections {
  margin-bottom: 2rem;
}

.setting-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.setting-section h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

/* 字体网格 */
.font-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.font-card {
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-primary);
}

.font-card:hover {
  border-color: var(--accent-color);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.font-card.active {
  border-color: var(--accent-color);
  background: rgba(0, 123, 255, 0.05);
}

.font-preview {
  margin-bottom: 0.75rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 4px;
  text-align: center;
}

.preview-text {
  font-size: 1.1rem;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.preview-text-en {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.font-info h4 {
  color: var(--text-primary);
  font-size: 0.9rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.font-info p {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.3;
}

/* 设置项样式 */
.setting-item {
  display: flex;
  align-items: center;
  padding: 1rem 0;
  border-bottom: 1px solid var(--border-color);
  gap: 1rem;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  flex: 1;
  min-width: 0;
}

.label-text {
  display: block;
  color: var(--text-primary);
  font-weight: 500;
  margin-bottom: 0.25rem;
}

.label-desc {
  display: block;
  color: var(--text-secondary);
  font-size: 0.85rem;
  line-height: 1.4;
}

/* 自定义字体区域 */
.custom-font-section {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.font-file-input {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.font-file-input .setting-input {
  flex: 1;
}

/* 输入控件样式 */
.setting-input {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
  min-width: 150px;
}

.setting-input:focus {
  outline: none;
  border-color: var(--accent-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

/* 滑块控制 */
.letter-spacing-control,
.word-spacing-control,
.indent-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.spacing-slider,
.indent-slider {
  width: 120px;
}

.spacing-value,
.indent-value {
  color: var(--text-primary);
  font-weight: 500;
  min-width: 50px;
  text-align: center;
}

/* 开关样式 */
.setting-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.setting-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .switch-slider {
  background-color: var(--accent-color);
}

input:checked + .switch-slider:before {
  transform: translateX(26px);
}

/* 字体预览区域 */
.preview-section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.preview-section h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1rem;
}

.font-preview-area {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 2rem;
  min-height: 200px;
}

.preview-title {
  color: var(--text-primary);
  font-size: 1.2rem;
  margin-bottom: 1rem;
  text-align: center;
}

.preview-paragraph {
  color: var(--text-primary);
  margin-bottom: 1.5rem;
  line-height: 1.8;
}

.preview-paragraph:last-child {
  margin-bottom: 0;
}

/* 按钮样式 */
.btn-primary,
.btn-secondary {
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

.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .font-settings {
    padding: 1rem;
  }
  
  .font-grid {
    grid-template-columns: 1fr;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }
  
  .setting-label {
    margin-right: 0;
  }
  
  .font-file-input {
    width: 100%;
    flex-direction: column;
  }
  
  .letter-spacing-control,
  .word-spacing-control,
  .indent-control {
    width: 100%;
    justify-content: space-between;
  }
  
  .settings-actions {
    flex-direction: column;
  }
  
  .font-preview-area {
    padding: 1rem;
  }
}
</style>