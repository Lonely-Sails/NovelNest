import { pluginErrorHandler, PluginError, ERROR_TYPES } from './pluginErrorHandler'

/**
 * 前端插件管理器
 * 负责插件的加载、执行和沙箱隔离
 */
export class PluginManager {
  constructor() {
    this.loadedPlugins = new Map()
    this.pluginErrors = new Map()
    this.executionTimeouts = new Map()
    this.defaultTimeout = 30000 // 30秒超时
  }

  /**
   * 加载插件
   * @param {string} sourceId - 插件ID
   * @param {string} pluginCode - 插件代码
   * @returns {Promise<Object>} 插件实例
   */
  async loadPlugin(sourceId, pluginCode) {
    try {
      // 检查是否已加载
      if (this.loadedPlugins.has(sourceId))
        return this.loadedPlugins.get(sourceId)

      // 创建沙箱环境
      const sandbox = this.createSandbox(sourceId)

      // 在沙箱中执行插件代码
      const pluginFunction = new Function('window', 'console', pluginCode)
      pluginFunction(sandbox, this.createPluginConsole(sourceId))

      // 获取插件实例
      const plugin = sandbox.bookSourcePlugin
      if (!plugin) throw new Error('插件未正确导出 bookSourcePlugin')

      // 验证插件接口
      this.validatePluginInterface(plugin)

      // 包装插件方法以添加错误处理和超时控制
      const wrappedPlugin = this.wrapPluginMethods(sourceId, plugin)

      // 缓存插件实例
      this.loadedPlugins.set(sourceId, wrappedPlugin)
      this.pluginErrors.delete(sourceId) // 清除之前的错误

      console.log(`插件 ${sourceId} 加载成功`)
      return wrappedPlugin

    } catch (error) {
      const pluginError = pluginErrorHandler.handleError(error, sourceId, 'plugin_load')
      this.pluginErrors.set(sourceId, pluginError.message)
      throw pluginError
    }
  }

  /**
   * 创建插件沙箱环境
   * @param {string} sourceId - 插件ID
   * @returns {Object} 沙箱对象
   */
  createSandbox(sourceId) {
    const sandbox = {
      // 提供受限的 Tauri API
      api: {
        invoke: window.__TAURI__.invoke
      },

      // 提供基本的全局对象
      setTimeout: window.setTimeout,
      clearTimeout: window.clearTimeout,
      setInterval: window.setInterval,
      clearInterval: window.clearInterval,

      // 提供 Promise 和基本类型
      Promise: window.Promise,
      Array: window.Array,
      Object: window.Object,
      String: window.String,
      Number: window.Number,
      Boolean: window.Boolean,
      Date: window.Date,
      RegExp: window.RegExp,
      JSON: window.JSON,
      Math: window.Math,

      // 提供编码解码函数
      encodeURIComponent: window.encodeURIComponent,
      decodeURIComponent: window.decodeURIComponent,
      btoa: window.btoa,
      atob: window.atob,

      // 插件实例将在这里设置
      bookSourcePlugin: null
    }

    // 防止访问危险的全局对象
    Object.defineProperty(sandbox, 'window', {
      get() {
        throw new Error('插件不能访问 window 对象')
      }
    })

    Object.defineProperty(sandbox, 'document', {
      get() {
        throw new Error('插件不能访问 document 对象')
      }
    })

    Object.defineProperty(sandbox, 'eval', {
      get() {
        throw new Error('插件不能使用 eval 函数')
      }
    })

    return sandbox
  }

  /**
   * 创建插件专用的 console 对象
   * @param {string} sourceId - 插件ID
   * @returns {Object} console 对象
   */
  createPluginConsole(sourceId) {
    const prefix = `[Plugin:${sourceId}]`

    return {
      log: (...args) => console.log(prefix, ...args),
      info: (...args) => console.info(prefix, ...args),
      warn: (...args) => console.warn(prefix, ...args),
      error: (...args) => console.error(prefix, ...args),
      debug: (...args) => console.debug(prefix, ...args)
    }
  }

  /**
   * 验证插件接口
   * @param {Object} plugin - 插件实例
   */
  validatePluginInterface(plugin) {
    const requiredMethods = ['search', 'getChapters', 'getChapterContent']

    for (const method of requiredMethods) {
      if (typeof plugin[method] !== 'function')
        throw new Error(`插件缺少必需的方法: ${method}`)
    }
  }

  /**
   * 包装插件方法以添加错误处理和超时控制
   * @param {string} sourceId - 插件ID
   * @param {Object} plugin - 原始插件实例
   * @returns {Object} 包装后的插件实例
   */
  wrapPluginMethods(sourceId, plugin) {
    const wrappedPlugin = {}

    // 包装所有方法
    for (const [methodName, method] of Object.entries(plugin)) {
      if (typeof method === 'function')
        wrappedPlugin[methodName] = this.wrapMethod(sourceId, methodName, method)
      else wrappedPlugin[methodName] = method
    }

    return wrappedPlugin
  }

  /**
   * 包装单个方法
   * @param {string} sourceId - 插件ID
   * @param {string} methodName - 方法名
   * @param {Function} method - 原始方法
   * @returns {Function} 包装后的方法
   */
  wrapMethod(sourceId, methodName, method) {
    return async (...args) => {
      const timeoutId = `${sourceId}_${methodName}_${Date.now()}`

      try {
        // 设置超时控制
        const timeoutPromise = new Promise((_, reject) => {
          const timeout = setTimeout(() => {
            reject(new PluginError(
              `插件方法 ${methodName} 执行超时`,
              ERROR_TYPES.EXECUTION_TIMEOUT,
              sourceId
            ))
          }, this.defaultTimeout)

          this.executionTimeouts.set(timeoutId, timeout)
        })

        // 执行插件方法
        const methodPromise = method.apply(plugin, args)

        // 等待方法执行或超时
        const result = await Promise.race([methodPromise, timeoutPromise])

        // 清除超时
        this.clearTimeout(timeoutId)

        return result

      } catch (error) {
        // 清除超时
        this.clearTimeout(timeoutId)

        // 使用错误处理器处理错误
        const pluginError = error instanceof PluginError
          ? error
          : pluginErrorHandler.handleError(error, sourceId, `method_${methodName}`)

        this.pluginErrors.set(`${sourceId}_${methodName}`, pluginError.message)

        // 重新抛出错误
        throw pluginError
      }
    }
  }

  /**
   * 清除执行超时
   * @param {string} timeoutId - 超时ID
   */
  clearTimeout(timeoutId) {
    const timeout = this.executionTimeouts.get(timeoutId)
    if (timeout) {
      clearTimeout(timeout)
      this.executionTimeouts.delete(timeoutId)
    }
  }

  /**
   * 卸载插件
   * @param {string} sourceId - 插件ID
   */
  unloadPlugin(sourceId) {
    this.loadedPlugins.delete(sourceId)
    this.pluginErrors.delete(sourceId)

    // 清除相关的超时
    for (const [timeoutId, timeout] of this.executionTimeouts.entries()) {
      if (timeoutId.startsWith(sourceId)) {
        clearTimeout(timeout)
        this.executionTimeouts.delete(timeoutId)
      }
    }

    console.log(`插件 ${sourceId} 已卸载`)
  }

  /**
   * 获取插件实例
   * @param {string} sourceId - 插件ID
   * @returns {Object|null} 插件实例
   */
  getPlugin(sourceId) {
    return this.loadedPlugins.get(sourceId) || null
  }

  /**
   * 检查插件是否已加载
   * @param {string} sourceId - 插件ID
   * @returns {boolean} 是否已加载
   */
  isPluginLoaded(sourceId) {
    return this.loadedPlugins.has(sourceId)
  }

  /**
   * 获取插件错误信息
   * @param {string} sourceId - 插件ID
   * @returns {string|null} 错误信息
   */
  getPluginError(sourceId) {
    return this.pluginErrors.get(sourceId) || null
  }

  /**
   * 获取所有已加载的插件ID
   * @returns {string[]} 插件ID列表
   */
  getLoadedPluginIds() {
    return Array.from(this.loadedPlugins.keys())
  }

  /**
   * 清除所有插件
   */
  clearAllPlugins() {
    // 清除所有超时
    for (const timeout of this.executionTimeouts.values()) {
      clearTimeout(timeout)
    }

    this.loadedPlugins.clear()
    this.pluginErrors.clear()
    this.executionTimeouts.clear()

    console.log('所有插件已清除')
  }

  /**
   * 设置执行超时时间
   * @param {number} timeout - 超时时间（毫秒）
   */
  setTimeout(timeout) {
    this.defaultTimeout = timeout
  }

  /**
   * 获取插件统计信息
   * @returns {Object} 统计信息
   */
  getStats() {
    const errorStats = pluginErrorHandler.getErrorStats()
    return {
      loadedCount: this.loadedPlugins.size,
      errorCount: this.pluginErrors.size,
      activeTimeouts: this.executionTimeouts.size,
      totalErrors: errorStats.total,
      errorsByType: errorStats.byType
    }
  }

  /**
   * 获取错误处理器实例
   * @returns {PluginErrorHandler} 错误处理器
   */
  getErrorHandler() {
    return pluginErrorHandler
  }

  /**
   * 获取插件错误报告
   * @param {string} sourceId - 插件ID（可选）
   * @returns {Object} 错误报告
   */
  getErrorReport(sourceId = null) {
    return pluginErrorHandler.generateErrorReport(sourceId)
  }

  /**
   * 清除插件错误历史
   * @param {string} sourceId - 插件ID（可选）
   */
  clearErrorHistory(sourceId = null) {
    pluginErrorHandler.clearErrorHistory(sourceId)
    if (sourceId) {
      // 清除插件管理器中的相关错误
      for (const key of this.pluginErrors.keys()) {
        if (key.startsWith(sourceId)) {
          this.pluginErrors.delete(key)
        }
      }
    } else {
      this.pluginErrors.clear()
    }
  }
}

// 创建全局插件管理器实例
export const pluginManager = new PluginManager()

// 在开发环境下暴露到全局作用域以便调试
if (import.meta.env.DEV) {
  window.pluginManager = pluginManager
}