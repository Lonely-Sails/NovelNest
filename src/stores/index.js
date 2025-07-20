import { createPinia } from 'pinia'

// Import all stores
export { useBookStore } from './bookStore'
export { useReaderStore } from './readerStore'
export { usePluginStore } from './pluginStore'
export { useSettingsStore } from './settingsStore'

// Main Pinia instance creation
export const pinia = createPinia()