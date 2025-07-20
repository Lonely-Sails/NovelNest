/**
 * 插件错误处理工具
 * 提供统一的错误处理、分类和报告功能
 */

export class PluginError extends Error {
    constructor(message, type = 'UNKNOWN', sourceId = null, details = null) {
        super(message)
        this.name = 'PluginError'
        this.type = type
        this.sourceId = sourceId
        this.details = details
        this.timestamp = new Date()
    }
}

// 错误类型常量
export const ERROR_TYPES = {
    LOAD_FAILED: 'LOAD_FAILED',           // 插件加载失败
    EXECUTION_TIMEOUT: 'EXECUTION_TIMEOUT', // 执行超时
    NETWORK_ERROR: 'NETWORK_ERROR',       // 网络错误
    PARSE_ERROR: 'PARSE_ERROR',           // 解析错误
    VALIDATION_ERROR: 'VALIDATION_ERROR', // 验证错误
    SANDBOX_VIOLATION: 'SANDBOX_VIOLATION', // 沙箱违规
    UNKNOWN: 'UNKNOWN'                    // 未知错误
}

// 错误消息映射
const ERROR_MESSAGES = {
    [ERROR_TYPES.LOAD_FAILED]: '插件加载失败',
    [ERROR_TYPES.EXECUTION_TIMEOUT]: '插件执行超时',
    [ERROR_TYPES.NETWORK_ERROR]: '网络连接错误',
    [ERROR_TYPES.PARSE_ERROR]: '数据解析错误',
    [ERROR_TYPES.VALIDATION_ERROR]: '数据验证失败',
    [ERROR_TYPES.SANDBOX_VIOLATION]: '插件违反安全规则',
    [ERROR_TYPES.UNKNOWN]: '未知错误'
}

export class PluginErrorHandler {
    constructor() {
        this.errorHistory = []
        this.maxHistorySize = 100
        this.errorCallbacks = new Map()
    }

    /**
     * 处理插件错误
     * @param {Error} error - 原始错误
     * @param {string} sourceId - 插件ID
     * @param {string} context - 错误上下文
     * @returns {PluginError} 处理后的插件错误
     */
    handleError(error, sourceId = null, context = null) {
        const errorType = this.classifyError(error)
        const pluginError = new PluginError(
            error.message,
            errorType,
            sourceId,
            {
                originalError: error,
                context,
                stack: error.stack
            }
        )

        // 记录错误历史
        this.recordError(pluginError)

        // 触发错误回调
        this.notifyErrorCallbacks(pluginError)

        return pluginError
    }

    /**
     * 分类错误类型
     * @param {Error} error - 错误对象
     * @returns {string} 错误类型
     */
    classifyError(error) {
        const message = error.message.toLowerCase()

        if (message.includes('timeout') || message.includes('超时')) {
            return ERROR_TYPES.EXECUTION_TIMEOUT
        }

        if (message.includes('network') || message.includes('fetch') ||
            message.includes('网络') || message.includes('连接')) {
            return ERROR_TYPES.NETWORK_ERROR
        }

        if (message.includes('parse') || message.includes('json') ||
            message.includes('解析') || message.includes('格式')) {
            return ERROR_TYPES.PARSE_ERROR
        }

        if (message.includes('validation') || message.includes('invalid') ||
            message.includes('验证') || message.includes('无效')) {
            return ERROR_TYPES.VALIDATION_ERROR
        }

        if (message.includes('sandbox') || message.includes('security') ||
            message.includes('沙箱') || message.includes('安全')) {
            return ERROR_TYPES.SANDBOX_VIOLATION
        }

        if (message.includes('load') || message.includes('import') ||
            message.includes('加载') || message.includes('导入')) {
            return ERROR_TYPES.LOAD_FAILED
        }

        return ERROR_TYPES.UNKNOWN
    }

    /**
     * 记录错误到历史记录
     * @param {PluginError} error - 插件错误
     */
    recordError(error) {
        this.errorHistory.unshift(error)

        // 限制历史记录大小
        if (this.errorHistory.length > this.maxHistorySize) {
            this.errorHistory = this.errorHistory.slice(0, this.maxHistorySize)
        }
    }

    /**
     * 获取错误历史记录
     * @param {string} sourceId - 插件ID（可选）
     * @param {string} errorType - 错误类型（可选）
     * @returns {PluginError[]} 错误列表
     */
    getErrorHistory(sourceId = null, errorType = null) {
        let errors = this.errorHistory

        if (sourceId) {
            errors = errors.filter(error => error.sourceId === sourceId)
        }

        if (errorType) {
            errors = errors.filter(error => error.type === errorType)
        }

        return errors
    }

    /**
     * 获取错误统计信息
     * @returns {Object} 统计信息
     */
    getErrorStats() {
        const stats = {
            total: this.errorHistory.length,
            byType: {},
            bySource: {},
            recent: this.errorHistory.slice(0, 10)
        }

        // 按类型统计
        for (const errorType of Object.values(ERROR_TYPES)) {
            stats.byType[errorType] = this.errorHistory.filter(
                error => error.type === errorType
            ).length
        }

        // 按插件统计
        const sourceIds = [...new Set(this.errorHistory.map(error => error.sourceId).filter(Boolean))]
        for (const sourceId of sourceIds) {
            stats.bySource[sourceId] = this.errorHistory.filter(
                error => error.sourceId === sourceId
            ).length
        }

        return stats
    }

    /**
     * 清除错误历史记录
     * @param {string} sourceId - 插件ID（可选，清除特定插件的错误）
     */
    clearErrorHistory(sourceId = null) {
        if (sourceId) {
            this.errorHistory = this.errorHistory.filter(error => error.sourceId !== sourceId)
        } else {
            this.errorHistory = []
        }
    }

    /**
     * 注册错误回调
     * @param {string} id - 回调ID
     * @param {Function} callback - 回调函数
     */
    onError(id, callback) {
        this.errorCallbacks.set(id, callback)
    }

    /**
     * 移除错误回调
     * @param {string} id - 回调ID
     */
    offError(id) {
        this.errorCallbacks.delete(id)
    }

    /**
     * 通知错误回调
     * @param {PluginError} error - 插件错误
     */
    notifyErrorCallbacks(error) {
        for (const callback of this.errorCallbacks.values()) {
            try {
                callback(error)
            } catch (callbackError) {
                console.error('错误回调执行失败:', callbackError)
            }
        }
    }

    /**
     * 格式化错误消息
     * @param {PluginError} error - 插件错误
     * @returns {string} 格式化的错误消息
     */
    formatError(error) {
        const typeMessage = ERROR_MESSAGES[error.type] || ERROR_MESSAGES[ERROR_TYPES.UNKNOWN]
        const sourceInfo = error.sourceId ? `[${error.sourceId}]` : ''
        const timeInfo = error.timestamp.toLocaleString()

        return `${sourceInfo} ${typeMessage}: ${error.message} (${timeInfo})`
    }

    /**
     * 生成错误报告
     * @param {string} sourceId - 插件ID（可选）
     * @returns {Object} 错误报告
     */
    generateErrorReport(sourceId = null) {
        const errors = this.getErrorHistory(sourceId)
        const stats = this.getErrorStats()

        return {
            summary: {
                totalErrors: errors.length,
                errorTypes: Object.keys(stats.byType).filter(type => stats.byType[type] > 0),
                timeRange: errors.length > 0 ? {
                    earliest: errors[errors.length - 1].timestamp,
                    latest: errors[0].timestamp
                } : null
            },
            statistics: stats,
            recentErrors: errors.slice(0, 10).map(error => ({
                type: error.type,
                message: error.message,
                sourceId: error.sourceId,
                timestamp: error.timestamp,
                context: error.details?.context
            })),
            recommendations: this.generateRecommendations(errors)
        }
    }

    /**
     * 生成错误处理建议
     * @param {PluginError[]} errors - 错误列表
     * @returns {string[]} 建议列表
     */
    generateRecommendations(errors) {
        const recommendations = []
        const errorTypes = [...new Set(errors.map(error => error.type))]

        if (errorTypes.includes(ERROR_TYPES.EXECUTION_TIMEOUT)) {
            recommendations.push('考虑增加插件执行超时时间或优化插件性能')
        }

        if (errorTypes.includes(ERROR_TYPES.NETWORK_ERROR)) {
            recommendations.push('检查网络连接状态和目标网站的可访问性')
        }

        if (errorTypes.includes(ERROR_TYPES.PARSE_ERROR)) {
            recommendations.push('检查目标网站的页面结构是否发生变化')
        }

        if (errorTypes.includes(ERROR_TYPES.SANDBOX_VIOLATION)) {
            recommendations.push('插件可能尝试访问受限资源，请检查插件代码')
        }

        if (errorTypes.includes(ERROR_TYPES.LOAD_FAILED)) {
            recommendations.push('检查插件文件格式和语法是否正确')
        }

        // 如果同一插件频繁出错
        const sourceErrorCounts = {}
        for (const error of errors) {
            if (error.sourceId) {
                sourceErrorCounts[error.sourceId] = (sourceErrorCounts[error.sourceId] || 0) + 1
            }
        }

        for (const [sourceId, count] of Object.entries(sourceErrorCounts)) {
            if (count > 5) {
                recommendations.push(`插件 ${sourceId} 频繁出错，建议禁用或更新`)
            }
        }

        return recommendations
    }
}

// 创建全局错误处理器实例
export const pluginErrorHandler = new PluginErrorHandler()

// 在开发环境下暴露到全局作用域以便调试
if (import.meta.env.DEV) {
    window.pluginErrorHandler = pluginErrorHandler
}