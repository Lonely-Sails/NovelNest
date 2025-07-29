/**
 * 防抖函数
 * @param {Function} func 要防抖的函数
 * @param {number} delay 延迟时间（毫秒）
 * @returns {Function} 防抖后的函数
 */
export function debounce(func, delay) {
  let timeoutId = null
  
  return function (...args) {
    // 清除之前的定时器
    if (timeoutId) {
      clearTimeout(timeoutId)
    }
    
    // 设置新的定时器
    timeoutId = setTimeout(() => {
      func.apply(this, args)
      timeoutId = null
    }, delay)
  }
}

/**
 * 创建一个可以手动触发的防抖函数
 * @param {Function} func 要防抖的函数
 * @param {number} delay 延迟时间（毫秒）
 * @returns {Object} 包含防抖函数和立即执行函数的对象
 */
export function createDebouncedFunction(func, delay) {
  let timeoutId = null
  
  const debouncedFunc = function (...args) {
    // 清除之前的定时器
    if (timeoutId) {
      clearTimeout(timeoutId)
    }
    
    // 设置新的定时器
    timeoutId = setTimeout(() => {
      func.apply(this, args)
      timeoutId = null
    }, delay)
  }
  
  // 立即执行函数，取消防抖
  const immediate = function (...args) {
    if (timeoutId) {
      clearTimeout(timeoutId)
      timeoutId = null
    }
    func.apply(this, args)
  }
  
  // 取消防抖
  const cancel = function () {
    if (timeoutId) {
      clearTimeout(timeoutId)
      timeoutId = null
    }
  }
  
  return {
    debounced: debouncedFunc,
    immediate,
    cancel
  }
}