import { createApp } from 'vue';
import { pinia } from './stores/index.js';
import { useSettingsStore } from './stores/settingsStore.js';
import router from './router/index.js';
import App from './App.vue';

import { registerGlobalComponents } from './plugins/components.js';

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

// 注册全局组件
registerGlobalComponents(app);

// Toast组件已在components.js中全局注册

app.mount("#app");
