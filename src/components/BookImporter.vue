<template>
  <div class="unified-importer">
    <!-- 导入区域 -->
    <div
      class="import-zone"
      :class="{ 'drop-active': isDragOver }"
      @drop="handleDrop"
      @dragover="handleDragOver"
      @dragenter="handleDragEnter"
      @dragleave="handleDragLeave"
    >
      <div class="import-content">
        <div class="import-icon">📚</div>
        <h3>{{ isDragOver ? "释放以导入文件" : "导入图书" }}</h3>
        <p>拖拽文件到此处，或点击下方按钮选择</p>

        <div class="import-actions">
          <BaseButton
            @click="importSingleFile"
            variant="primary"
            :loading="importing.single"
            icon="📖"
          >
            选择文件
          </BaseButton>

          <BaseButton
            @click="importFolder"
            variant="secondary"
            :loading="importing.folder"
            icon="📁"
          >
            选择文件夹
          </BaseButton>

          <BaseButton
            @click="showSettings = true"
            variant="outline"
            size="small"
            icon="⚙️"
            class="settings-btn"
          >
            设置
          </BaseButton>
        </div>
      </div>
    </div>

    <!-- 导入进度 -->
    <div v-if="importProgress.show" class="import-progress">
      <BaseCard>
        <div class="progress-header">
          <h3>{{ importProgress.title }}</h3>
          <BaseButton
            v-if="importProgress.canCancel"
            @click="cancelImport"
            variant="outline"
            size="small"
          >
            取消
          </BaseButton>
        </div>

        <div class="progress-bar">
          <div
            class="progress-fill"
            :style="{ width: importProgress.percentage + '%' }"
          ></div>
        </div>

        <div class="progress-info">
          <span>{{ importProgress.current }} / {{ importProgress.total }}</span>
          <span>{{ importProgress.percentage }}%</span>
        </div>

        <div v-if="importProgress.currentFile" class="current-file">
          正在处理: {{ importProgress.currentFile }}
        </div>

        <!-- 导入结果 -->
        <div v-if="importProgress.results.length > 0" class="import-results">
          <h4>导入结果</h4>
          <div class="results-list">
            <div
              v-for="result in importProgress.results"
              :key="result.file"
              class="result-item"
              :class="result.status"
            >
              <span class="result-icon">
                {{ result.status === "success" ? "✅" : "❌" }}
              </span>
              <span class="result-file">{{ result.file }}</span>
              <span v-if="result.error" class="result-error">{{
                result.error
              }}</span>
            </div>
          </div>
        </div>
      </BaseCard>
    </div>

    <!-- 导入预览 -->
    <Modal
      v-if="showPreview"
      @close="closePreview"
      title="导入预览"
      size="large"
    >
      <div class="import-preview">
        <div class="preview-header">
          <p>找到 {{ previewFiles.length }} 个文件，请确认导入：</p>
          <div class="preview-actions">
            <BaseButton @click="selectAll" variant="outline" size="small">
              {{ allSelected ? "取消全选" : "全选" }}
            </BaseButton>
            <BaseButton @click="filterByFormat" variant="outline" size="small">
              按格式筛选
            </BaseButton>
          </div>
        </div>

        <div class="preview-filters" v-if="showFilters">
          <div class="filter-group">
            <label>文件格式:</label>
            <div class="format-checkboxes">
              <label
                v-for="format in availableFormats"
                :key="format"
                class="checkbox-label"
              >
                <input
                  type="checkbox"
                  :value="format"
                  v-model="selectedFormats"
                  @change="applyFormatFilter"
                />
                {{ format.toUpperCase() }}
              </label>
            </div>
          </div>
        </div>

        <div class="preview-list">
          <div
            v-for="file in filteredPreviewFiles"
            :key="file.path"
            class="preview-item"
            :class="{ selected: file.selected }"
          >
            <input
              type="checkbox"
              v-model="file.selected"
              class="file-checkbox"
            />
            <div class="file-info">
              <div class="file-name">{{ file.name }}</div>
              <div class="file-details">
                <span class="file-format">{{ file.format.toUpperCase() }}</span>
                <span class="file-size">{{ formatFileSize(file.size) }}</span>
                <span class="file-path">{{ file.path }}</span>
              </div>
            </div>
            <div class="file-status">
              <span v-if="file.exists" class="status-warning">已存在</span>
              <span v-else class="status-new">新文件</span>
            </div>
          </div>
        </div>

        <div class="preview-summary">
          <p>
            选中 {{ selectedFiles.length }} 个文件
            <span v-if="newFiles.length > 0"
              >（{{ newFiles.length }} 个新文件）</span
            >
            <span v-if="existingFiles.length > 0"
              >（{{ existingFiles.length }} 个已存在）</span
            >
          </p>
        </div>
      </div>

      <template #actions>
        <BaseButton @click="closePreview" variant="secondary">
          取消
        </BaseButton>
        <BaseButton
          @click="confirmImport"
          variant="primary"
          :disabled="selectedFiles.length === 0"
        >
          确认导入 ({{ selectedFiles.length }})
        </BaseButton>
      </template>
    </Modal>

    <!-- 导入设置 -->
    <Modal
      v-if="showSettings"
      @close="closeSettings"
      title="导入设置"
      size="medium"
    >
      <div class="import-settings">
        <div class="setting-group">
          <label class="setting-label">支持的文件格式</label>
          <div class="format-list">
            <label
              v-for="format in supportedFormats"
              :key="format.ext"
              class="format-item"
            >
              <input type="checkbox" v-model="format.enabled" />
              <span class="format-name">{{ format.ext.toUpperCase() }}</span>
              <span class="format-desc">{{ format.description }}</span>
            </label>
          </div>
        </div>

        <div class="setting-group">
          <label class="setting-label">导入选项</label>
          <div class="option-list">
            <label class="option-item">
              <input type="checkbox" v-model="importSettings.skipExisting" />
              跳过已存在的文件
            </label>
            <label class="option-item">
              <input
                type="checkbox"
                v-model="importSettings.autoExtractMetadata"
              />
              自动提取元数据
            </label>
            <label class="option-item">
              <input type="checkbox" v-model="importSettings.createBackup" />
              创建备份
            </label>
          </div>
        </div>

        <div class="setting-group">
          <label class="setting-label">文件命名规则</label>
          <select v-model="importSettings.namingRule" class="naming-select">
            <option value="original">保持原文件名</option>
            <option value="title">使用书名</option>
            <option value="title_author">书名_作者</option>
            <option value="author_title">作者_书名</option>
          </select>
        </div>
      </div>

      <template #actions>
        <BaseButton @click="resetSettings" variant="outline"> 重置 </BaseButton>
        <BaseButton @click="saveSettings" variant="primary">
          保存设置
        </BaseButton>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useBookStore } from '@/stores/bookStore';
import { useToast } from '@/composables/useToast';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

// Emits
const emit = defineEmits(['import-complete']);

// Store
const bookStore = useBookStore();

// Toast消息提示
const { showSuccess, showError, showWarning, showInfo } = useToast();

// 响应式数据
const importing = ref({
  single: false,
  folder: false,
  batch: false,
});

const isDragOver = ref(false);
const showPreview = ref(false);
const showSettings = ref(false);
const showFilters = ref(false);

// 导入进度
const importProgress = ref({
  show: false,
  title: '',
  current: 0,
  total: 0,
  percentage: 0,
  currentFile: '',
  canCancel: false,
  results: [],
});

// 预览文件
const previewFiles = ref([]);
const selectedFormats = ref([]);

// 导入设置
const importSettings = ref({
  skipExisting: true,
  autoExtractMetadata: true,
  createBackup: false,
  namingRule: 'original',
});

// 支持的文件格式
const supportedFormats = ref([
  { ext: 'txt', description: '纯文本文件', enabled: true },
  { ext: 'epub', description: 'EPUB电子书', enabled: true },
  { ext: 'pdf', description: 'PDF文档', enabled: true },
  { ext: 'mobi', description: 'Kindle格式', enabled: false },
  { ext: 'azw3', description: 'Kindle格式', enabled: false },
]);

// 计算属性
const availableFormats = computed(() => {
  return [...new Set(previewFiles.value.map((f) => f.format))];
});

const filteredPreviewFiles = computed(() => {
  if (selectedFormats.value.length === 0) {
    return previewFiles.value;
  }
  return previewFiles.value.filter((f) =>
    selectedFormats.value.includes(f.format)
  );
});

const selectedFiles = computed(() => {
  return filteredPreviewFiles.value.filter((f) => f.selected);
});

const newFiles = computed(() => {
  return selectedFiles.value.filter((f) => !f.exists);
});

const existingFiles = computed(() => {
  return selectedFiles.value.filter((f) => f.exists);
});

const allSelected = computed(() => {
  return (
    filteredPreviewFiles.value.length > 0 &&
    filteredPreviewFiles.value.every((f) => f.selected)
  );
});

// 方法
const importSingleFile = async () => {
  try {
    importing.value.single = true;

    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: '电子书文件',
          extensions: supportedFormats.value
            .filter((file) => file.enabled)
            .map((file) => file.ext),
        },
      ],
    });

    if (filePath) await processImport([filePath], '导入单个文件');
  } catch (error) {
    console.error('导入单个文件失败:', error);
    showError('导入失败: ' + error.message);
  } finally {
    importing.value.single = false;
  }
};

const importFolder = async () => {
  try {
    importing.value.folder = true;

    const folderPath = await open({
      directory: true,
      multiple: false,
    });

    if (folderPath) {
      // 扫描文件夹中的文件
      const files = await scanFolder(folderPath);
      if (files.length > 0) await showImportPreview(files);
      else showWarning('文件夹中没有找到支持的电子书文件');
    }
  } catch (error) {
    console.error('导入文件夹失败:', error);
    showError('导入失败: ' + error.message);
  } finally {
    importing.value.folder = false;
  }
};

const handleDrop = async (event) => {
  event.preventDefault();
  isDragOver.value = false;

  const files = Array.from(event.dataTransfer.files);

  try {
    const filePaths = [];

    // 处理拖拽的文件
    for (const file of files) {
      // 检查文件类型
      const extension = file.name.split('.').pop()?.toLowerCase();
      const supportedExts = supportedFormats.value
        .filter((f) => f.enabled)
        .map((f) => f.ext);

      if (supportedExts.includes(extension)) {
        // 在Tauri环境中，拖拽文件应该有完整路径
        // 如果没有路径信息，则跳过该文件
        if (file.path) {
          filePaths.push(file.path);
        } else {
          console.warn('拖拽的文件缺少路径信息:', file.name);
        }
      }
    }

    if (filePaths.length > 0) {
      if (filePaths.length === 1) {
        await processImport(filePaths, '拖拽导入');
      } else {
        await showImportPreview(filePaths);
      }
    } else {
      showWarning('没有找到支持的文件格式');
    }
  } catch (error) {
    console.error('拖拽导入失败:', error);
    showError('拖拽导入失败: ' + error.message);
  }
};

const handleDragOver = (event) => {
  event.preventDefault();
};

const handleDragEnter = (event) => {
  event.preventDefault();
  isDragOver.value = true;
};

const handleDragLeave = (event) => {
  event.preventDefault();
  if (!event.currentTarget.contains(event.relatedTarget)) {
    isDragOver.value = false;
  }
};

const scanFolder = async (folderPath) => {
  try {
    const files = await invoke('scan_folder_for_books', { folderPath });
    return files;
  } catch (error) {
    console.error('扫描文件夹失败:', error);
    return [];
  }
};

const showImportPreview = async (filePaths) => {
  try {
    // 获取文件信息
    const fileInfos = await Promise.all(
      filePaths.map(async (path) => {
        try {
          const info = await invoke('get_file_info', { filePath: path });
          const exists = await checkBookExists(path);

          return {
            path,
            name: info.name,
            size: info.size,
            format: info.extension.toLowerCase(),
            selected: !exists || !importSettings.value.skipExisting,
            exists,
          };
        } catch (error) {
          console.error('获取文件信息失败:', path, error);
          return null;
        }
      })
    );

    previewFiles.value = fileInfos.filter((info) => info !== null);
    selectedFormats.value = [];
    showPreview.value = true;
  } catch (error) {
    console.error('显示导入预览失败:', error);
    showError('预览失败: ' + error.message);
  }
};

const checkBookExists = async (filePath) => {
  try {
    const exists = await invoke('check_book_exists', { filePath });
    return exists;
  } catch (error) {
    return false;
  }
};

const selectAll = () => {
  const newValue = !allSelected.value;
  filteredPreviewFiles.value.forEach((file) => {
    file.selected = newValue;
  });
};

const filterByFormat = () => {
  showFilters.value = !showFilters.value;
};

const applyFormatFilter = () => {
  // 过滤逻辑已在计算属性中处理
};

const closePreview = () => {
  showPreview.value = false;
  previewFiles.value = [];
  selectedFormats.value = [];
};

const confirmImport = async () => {
  const filesToImport = selectedFiles.value.map((f) => f.path);
  closePreview();
  await processImport(filesToImport, '批量导入');
};

const processImport = async (filePaths, title) => {
  if (filePaths.length === 0) return;

  // 显示进度
  importProgress.value = {
    show: true,
    title,
    current: 0,
    total: filePaths.length,
    percentage: 0,
    currentFile: '',
    canCancel: filePaths.length > 1,
    results: [],
  };

  try {
    if (filePaths.length === 1) {
      // 单个文件导入
      const filePath = filePaths[0];
      const fileName = filePath.split('/').pop() || filePath.split('\\').pop();

      importProgress.value.current = 1;
      importProgress.value.currentFile = fileName;
      importProgress.value.percentage = 50;

      try {
        const book = await bookStore.importBook(filePath);
        importProgress.value.results.push({
          file: fileName,
          status: 'success',
          book,
        });
        importProgress.value.percentage = 100;
      } catch (error) {
        console.error('导入文件失败:', filePath, error);
        importProgress.value.results.push({
          file: fileName,
          status: 'error',
          error: error.message,
        });
        importProgress.value.percentage = 100;
      }
    } else {
      // 批量导入
      try {
        const result = await invoke('batch_import_books', {
          filePaths,
          skipExisting: importSettings.value.skipExisting,
        });

        // 更新进度
        importProgress.value.current = result.total;
        importProgress.value.percentage = 100;
        importProgress.value.results = result.results.map((r) => ({
          file: r.file_path.split('/').pop() || r.file_path.split('\\').pop(),
          status: r.status,
          error: r.error,
          book: r.book,
        }));
      } catch (error) {
        console.error('批量导入失败:', error);
        // 回退到逐个导入
        for (let i = 0; i < filePaths.length; i++) {
          const filePath = filePaths[i];
          const fileName =
            filePath.split('/').pop() || filePath.split('\\').pop();

          importProgress.value.current = i + 1;
          importProgress.value.currentFile = fileName;
          importProgress.value.percentage = Math.round(
            ((i + 1) / filePaths.length) * 100
          );

          try {
            const book = await bookStore.importBook(filePath);
            importProgress.value.results.push({
              file: fileName,
              status: 'success',
              book,
            });
          } catch (error) {
            console.error('导入文件失败:', filePath, error);
            importProgress.value.results.push({
              file: fileName,
              status: 'error',
              error: error.message,
            });
          }
        }
      }
    }

    const successCount = importProgress.value.results.filter(
      (r) => r.status === 'success'
    ).length;
    const errorCount = importProgress.value.results.filter(
      (r) => r.status === 'error'
    ).length;
    const skippedCount = importProgress.value.results.filter(
      (r) => r.status === 'skipped'
    ).length;

    if (successCount > 0) {
      let message = `成功导入 ${successCount} 本图书`;
      if (errorCount > 0) message += `，${errorCount} 个失败`;
      if (skippedCount > 0) message += `，${skippedCount} 个跳过`;

      showSuccess(message);
      emit('import-complete', {
        success: successCount,
        error: errorCount,
        skipped: skippedCount,
        results: importProgress.value.results,
      });
    } else if (skippedCount > 0) {
      showInfo(`${skippedCount} 个文件已存在，已跳过`);
    } else {
      showError('导入失败，请检查文件格式');
    }
  } catch (error) {
    console.error('导入过程中发生错误:', error);
    showError('导入过程中发生错误: ' + error.message);
  }

  // 延迟隐藏进度，让用户看到结果
  setTimeout(() => {
    importProgress.value.show = false;
  }, 3000);
};

const cancelImport = () => {
  // TODO: 实现取消导入逻辑
  importProgress.value.show = false;
  showInfo('导入已取消');
};

const formatFileSize = (bytes) => {
  if (!bytes || bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const closeSettings = () => {
  showSettings.value = false;
};

const resetSettings = () => {
  importSettings.value = {
    skipExisting: true,
    autoExtractMetadata: true,
    createBackup: false,
    namingRule: 'original',
  };
};

const saveSettings = () => {
  // 保存设置到本地存储
  localStorage.setItem('import_settings', JSON.stringify(importSettings.value));
  localStorage.setItem(
    'supported_formats',
    JSON.stringify(supportedFormats.value)
  );
  showSuccess('设置已保存');
  closeSettings();
};

// 组件挂载时加载设置
onMounted(() => {
  try {
    const savedSettings = localStorage.getItem('import_settings');
    if (savedSettings) {
      importSettings.value = {
        ...importSettings.value,
        ...JSON.parse(savedSettings),
      };
    }

    const savedFormats = localStorage.getItem('supported_formats');
    if (savedFormats) {
      supportedFormats.value = JSON.parse(savedFormats);
    }
  } catch (error) {
    console.error('加载导入设置失败:', error);
  }
});
</script>

<style scoped>
.unified-importer {
  padding: 1rem;
}

.import-zone {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 2rem;
  margin-bottom: 2rem;
  transition: all 0.3s ease;
  background-color: var(--bg-secondary);
}

.import-zone:hover,
.import-zone.drop-active {
  border-color: var(--accent-color);
  background-color: var(--accent-color-light);
}

.import-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.import-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
  opacity: 0.7;
}

.import-content h3 {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.import-content p {
  color: var(--text-secondary);
  margin-bottom: 2rem;
  font-size: 1rem;
}

.import-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  justify-content: center;
}

.settings-btn {
  margin-left: 0.5rem;
}

/* 导入进度 */
.import-progress {
  margin-top: 2rem;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.progress-header h3 {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-color), var(--accent-hover));
  transition: width 0.3s ease;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 0.9rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.current-file {
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-bottom: 1rem;
  padding: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 4px;
}

/* 导入结果 */
.import-results {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.import-results h4 {
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.75rem;
}

.results-list {
  max-height: 200px;
  overflow-y: auto;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
  margin-bottom: 0.25rem;
  border-radius: 4px;
  font-size: 0.85rem;
}

.result-item.success {
  background: rgba(40, 167, 69, 0.1);
}

.result-item.error {
  background: rgba(220, 53, 69, 0.1);
}

.result-icon {
  flex-shrink: 0;
}

.result-file {
  flex: 1;
  color: var(--text-primary);
  font-weight: 500;
}

.result-error {
  color: var(--error-color);
  font-size: 0.8rem;
}

/* 导入预览 */
.import-preview {
  max-height: 70vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.preview-actions {
  display: flex;
  gap: 0.5rem;
}

.preview-filters {
  margin-bottom: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 6px;
}

.filter-group label {
  display: block;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.format-checkboxes {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
  cursor: pointer;
}

.preview-list {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 1rem;
}

.preview-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin-bottom: 0.5rem;
  transition: all 0.2s ease;
}

.preview-item:hover {
  background: var(--bg-secondary);
}

.preview-item.selected {
  border-color: var(--accent-color);
  background: var(--accent-color-light);
}

.file-checkbox {
  flex-shrink: 0;
}

.file-info {
  flex: 1;
}

.file-name {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.file-details {
  display: flex;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.file-format {
  background: var(--accent-color);
  color: white;
  padding: 0.1rem 0.4rem;
  border-radius: 3px;
  font-weight: 500;
}

.file-status {
  flex-shrink: 0;
}

.status-warning {
  color: var(--warning-color);
  font-size: 0.8rem;
}

.status-new {
  color: var(--success-color);
  font-size: 0.8rem;
}

.preview-summary {
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 6px;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

/* 导入设置 */
.import-settings {
  max-height: 60vh;
  overflow-y: auto;
}

.setting-group {
  margin-bottom: 2rem;
}

.setting-label {
  display: block;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
}

.format-list,
.option-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.format-item,
.option-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.format-item:hover,
.option-item:hover {
  background: var(--bg-secondary);
}

.format-name {
  font-weight: 500;
  color: var(--text-primary);
  min-width: 60px;
}

.format-desc {
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.naming-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
}

.naming-select:focus {
  outline: none;
  border-color: var(--accent-color);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .import-actions {
    flex-direction: column;
    width: 100%;
  }

  .preview-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }

  .file-details {
    flex-direction: column;
    gap: 0.25rem;
  }

  .format-checkboxes {
    flex-direction: column;
  }
}
</style>