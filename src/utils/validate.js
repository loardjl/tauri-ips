export const ipReg =
  /^((25[0-5]|2[0-4]\d|((1\d{2})|([1-9]?\d)))\.){3}(25[0-5]|2[0-4]\d|((1\d{2})|([1-9]?\d)))$/

export const portValidator = val => {
  if (val > 65535 || val < 1) {
    return '请输入1-65535的端口号'
  }
}
