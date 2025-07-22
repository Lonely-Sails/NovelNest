<template>
  <BaseSwitch
    v-model="isDarkMode"
    :icon="isDarkMode ? '🌙' : '☀️'"
    :title="isDarkMode ? '切换到浅色模式' : '切换到深色模式'"
    class="theme-toggle"
  />
</template>

<script>
import { computed } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'

export default {
  name: 'ThemeToggle',
  setup() {
    const settingsStore = useSettingsStore()

    // 使用计算属性实现双向绑定
    const isDarkMode = computed({
      get: () => settingsStore.currentTheme === 'dark',
      set: () => settingsStore.toggleTheme()
    })

    return {
      isDarkMode
    }
  }
}
</script>

<style scoped>
.theme-toggle {
  padding: 0.25rem;
}
</style>