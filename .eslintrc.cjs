// Description: eslint配置文件
module.exports = {
  root: true,
  env: {
    node: true, // node环境
    browser: true // 浏览器环境
  },
  extends: [
    'eslint:recommended', // eslint默认推荐规则
    'plugin:vue/vue3-essential', // eslint-plugin-vue插件推荐规则
    'eslint-config-prettier', // eslint和prettier冲突解决
    'prettier' // prettier推荐规则
  ],
  overrides: [],
  parserOptions: {
    ecmaVersion: 'latest', // ECMAScript版本
    sourceType: 'module', // 默认script，如果代码是ECMAScript模块，设置为module
    jsxPragma: 'React', // jsx语法解析器
    ecmaFeatures: {
      jsx: true // 启用jsx
    }
  },
  plugins: ['vue', 'prettier'],
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'off',
    'no-unused-vars': ['error', { vars: 'all', args: 'after-used', ignoreRestSiblings: false }], // 未使用变量
    eqeqeq: 'error', // 强制使用全等
    'no-undef': 'error', // 关闭未定义变量
    'prettier/prettier': [
      'warn',
      {
        endOfLine: 'auto' // 结尾是 \n \r \n\r auto
      }
    ],
    'prefer-const': [
      'error',
      {
        destructuring: 'any',
        ignoreReadBeforeAssign: true
      }
    ],
    'no-useless-escape': 'off', // 关闭禁止转义字符
    'vue/multi-word-component-names': 'off', // 关闭组件名必须是多个单词
    complexity: ['error', 18] // 圈复杂度
  }
}
