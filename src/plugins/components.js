/**
 * 全局组件注册
 * 使用Vue 3组合式API自动注册基础组件，避免在每个组件中重复导入
 */
import Modal from '@/components/Modal.vue'
import Toast from '@/components/Toast.vue'
import Loading from '@/components/Loading.vue'
import { BaseButton, BaseCard, BaseBadge, BaseInput, BaseSwitch } from '@/components/base'

// 要全局注册的组件列表
const components = {
  // 基础UI组件
  BaseButton,
  BaseCard,
  BaseBadge,
  BaseInput,
  BaseSwitch,

  // 功能性组件
  Modal,
  Toast,
  Loading,
}

/**
 * 注册全局组件
 * @param {import('vue').App} app - Vue应用实例
 */
export function registerGlobalComponents(app) {
  // 注册每个组件
  Object.entries(components).forEach(([name, component]) => {
    app.component(name, component)
  })
}