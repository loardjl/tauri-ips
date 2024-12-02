/*
 * @Author: chunlaizhang chunlai.zhang@ujoin-tech.com
 * @Date: 2024-06-06 09:57:44
 * @LastEditors: chunlaizhang
 * @LastEditTime: 2024-06-06 16:07:04
 * @FilePath: \kws\src\renderer\utils\authEnum.js
 */
import { BUSINESS_ENUM } from '@src/assets/static'

/**
 * 特殊权限枚举
 */
const authEnum = {
  COMMON: [], // 通用权限 无需特殊权限
  PROCESS_MONITOR: [] // 过程监控
}

const common = []
Object.entries(BUSINESS_ENUM).forEach(([key, value]) => {
  BUSINESS_ENUM[key] = 1 << value
  common.push(BUSINESS_ENUM[key])
})
authEnum.COMMON = common
authEnum.PROCESS_MONITOR = [
  BUSINESS_ENUM.OVERLOAD_MONITORING,
  BUSINESS_ENUM.BROKEN_TOOL_MONITORING,
  BUSINESS_ENUM.AUTO_BOUNDARY
]

export default { ...BUSINESS_ENUM, ...authEnum }
