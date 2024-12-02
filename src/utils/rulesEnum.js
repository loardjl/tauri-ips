/*
 * @Author: chunlaizhang chunlai.zhang@ujoin-tech.com
 * @Date: 2024-06-06 14:37:46
 * @LastEditors: chunlaizhang
 * @LastEditTime: 2024-06-06 14:37:51
 * @FilePath: \kws\src\renderer\utils\rulesEnum.js
 */
const READ = 1 << 0 // 1 可读权限
const WRITE = 1 << 1 // 2 可写权限
const DELETE = 1 << 2 // 4 可删除权限
const CREATE = 1 << 3 // 8 可新增权限
const UPDATE = 1 << 4 // 16 可编辑权限

/**
 * @description 所有权限
 */
export const allRules = READ | WRITE | DELETE | CREATE | UPDATE
/**
 * @description 除了读权限外的所有权限
 */
export const allButRead = WRITE | DELETE | CREATE | UPDATE
/**
 * @description 除了写权限外的所有权限
 */
export const allButWrite = READ | DELETE | CREATE | UPDATE
/**
 * @description 除了删除权限外的所有权限
 */
export const allButDelete = READ | WRITE | CREATE | UPDATE
/**
 * @description 除了新增权限外的所有权限
 */
export const allButCreate = READ | WRITE | DELETE | UPDATE
/**
 * @description 除了编辑权限外的所有权限
 */
export const allButUpdate = READ | WRITE | DELETE | CREATE
