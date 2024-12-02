import dayjs from 'dayjs'
/**
 * 日期格式化
 * @param {日期} value
 * @param {日期格式} fmt
 * @returns
 */
const dateFormatter = (value, fmt = 'YYYY/MM/DD') => {
  let str = ''
  if (value) {
    str = dayjs(value).format(fmt)
  }
  return str
}

export default { dateFormatter }
