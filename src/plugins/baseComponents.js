// 基础组件全局注册插件
import { BaseCard, BaseButton, BaseInput, BaseSwitch, BaseBadge } from '@/components/base'

export default {
  install(app) {
    // 全局注册基础组件
    app.component('BaseCard', BaseCard)
    app.component('BaseButton', BaseButton)
    app.component('BaseInput', BaseInput)
    app.component('BaseSwitch', BaseSwitch)
    app.component('BaseBadge', BaseBadge)
  }
}