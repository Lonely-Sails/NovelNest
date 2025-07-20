<template>
  <button
    @click="toggleTheme"
    class="theme-toggle"
    :class="{ 'theme-toggle-dark': isDark }"
    :title="isDark ? '切换到浅色模式' : '切换到深色模式'"
  >
    <div class="theme-toggle-track">
      <div class="theme-toggle-thumb">
        <span class="theme-icon">{{ isDark ? '🌙' : '☀️' }}</span>
      </div>
    </div>
  </button>
</template>

<script>
import { computed } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'

export default {
  name: 'ThemeToggle',
  setup() {
    const settingsStore = useSettingsStore()

    const isDark = computed(() => settingsStore.currentTheme === 'dark')

    const toggleTheme = () => {
      settingsStore.toggleTheme()
    }

    return {
      isDark,
      toggleTheme
    }
  }
}
</script>

<style scoped>
.theme-toggle {
  background: none;
  border: 1px solid var(--border-color);
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 6px;
  transition: all 0.2s ease;
  font-size: 1rem;
}

.theme-toggle:hover {
  border-color: var(--accent-color);
}

.theme-toggle-track {
  width: 40px;
  height: 20px;
  background-color: var(--border-color);
  border-radius: 10px;
  position: relative;
  transition: background-color 0.2s ease;
}

.theme-toggle-dark .theme-toggle-track {
  background-color: var(--accent-color);
}

.theme-toggle-thumb {
  width: 16px;
  height: 16px;
  background-color: white;
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: transform 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-toggle-dark .theme-toggle-thumb {
  transform: translateX(20px);
}

.theme-icon {
  font-size: 0.6rem;
}
</style>