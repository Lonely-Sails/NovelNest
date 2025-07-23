import { createApp } from 'vue';
import { pinia } from './stores/index.js';
import { useSettingsStore } from './stores/settingsStore.js';
import router from './router/index.js';
import App from './App.vue';
import Toast from './components/Toast.vue';
import baseComponents from './plugins/baseComponents.js';

// 引入基础样式和变量
import './styles/base.css'
import './styles/variables.css';

const app = createApp(App);

// 使用 Pinia
app.use(pinia);

// 初始化设置
const settingsStore = useSettingsStore();
settingsStore.initializeSettings();

// 使用路由
app.use(router);

// 注册基础组件
app.use(baseComponents);

// 全局注册 Toast 组件
app.component('Toast', Toast);

app.mount("#app");
