/*
 * @Author: 何志祥
 * @Date: 2023-09-23 19:34:31
 * @LastEditors: 何志祥
 * @LastEditTime: 2023-10-27 17:23:01
 * @Description: 枚举
 */

export const sig_data_type = new Map([
  [0, 'int'],
  [1, 'float'],
  [2, 'string'],
  [3, 'boolean']
])

/**
 * @Author: 何志祥
 * @Date: 2023-10-12 14:08:58
 * @Description: 采集方式
 */
export const collectTypesObj = {
  0: 'CNC',
  1: 'PMC',
  2: '宏变量'
}

export const collectTypes = [
  {
    text: 'CNC',
    value: 0
  },
  {
    text: 'PMC',
    value: 1
  },
  {
    text: '宏变量',
    value: 2
  }
]

export const controlList = [
  {
    text: 'PMC',
    value: 0
  },
  {
    text: '宏变量',
    value: 1
  }
]
export const controlListObj = {
  0: 'PMC',
  1: '宏变量'
}

const singleSignal = { name: '单信号', value: 1 }
const xyzSignal = [
  { name: 'X', value: 1 },
  { name: 'Y', value: 2 },
  { name: 'Z', value: 3 }
]
const iavSignal = [
  { name: 'IA', value: 1 },
  { name: 'IB', value: 2 },
  { name: 'IC', value: 3 },
  { name: 'VA', value: 4 },
  { name: 'VB', value: 5 },
  { name: 'VC', value: 6 }
]
/**
 * 传感器与通道对应关系
 */
export const sensorChannelType = {
  USV002: [singleSignal],
  USV004: [singleSignal],
  USB5621AE: [singleSignal],
  USV003: xyzSignal,
  USV357: xyzSignal,
  USP910: iavSignal
}
/**
 * 报警反馈原因，type 0: 误报警 1: 真报警
 */
export const allAlarmReason = [
  {
    label: 'button.relearn',
    value: 128,
    type: 0,
    img: 'relearn.svg',
    width: '29px',
    height: '26px'
  },
  {
    label: 'button.changeKnife',
    value: 129,
    type: 0,
    img: 'relearn.svg',
    width: '29px',
    height: '26px'
  },
  {
    label: 'button.debug',
    value: 130,
    type: 0,
    img: 'debug.svg',
    width: '29px',
    height: '26px'
  },
  {
    label: 'button.falseAlarm',
    value: 131,
    type: 0,
    img: 'falseAlarm.svg',
    width: '33px',
    height: '26px'
  },
  {
    label: 'button.broken',
    value: 1,
    type: 1,
    img: 'broken.svg',
    width: '20px',
    height: '27px'
  },
  {
    label: 'button.tipping',
    value: 2,
    type: 1,
    img: 'tipping.svg',
    width: '38px',
    height: '24px'
  },
  {
    label: 'button.wear',
    value: 3,
    type: 1,
    img: 'wear.svg',
    width: '25px',
    height: '26px'
  },
  {
    label: 'button.clampingAbnormal',
    value: 4,
    type: 1,
    img: 'exception.svg',
    width: '31px',
    height: '24px'
  },
  {
    label: 'button.blankAbnormal',
    value: 5,
    type: 1,
    img: 'exception.svg',
    width: '31px',
    height: '24px'
  },
  {
    label: 'button.chipEntan',
    value: 6,
    type: 1,
    img: 'wear.svg',
    width: '25px',
    height: '26px'
  },
  {
    label: 'button.emptyProcess',
    value: 7,
    type: 1,
    img: 'kongjiagong.svg',
    width: '25px',
    height: '24px'
  },
  {
    label: 'button.other',
    value: 0,
    type: 1,
    img: 'other.svg',
    width: '24px',
    height: '24px'
  }
]
/**
 * 业务类型枚举值 1: 自动边界 2: 自动断刀 3 异常监控 4: IPS 5 手动边界
 */
export const businessIdType = {
  kFTAutoBoundaryId: 1,
  kFTAutoToolBrokenId: 2,
  kFTExceptionMonitorId: 3,
  kFTIPSId: 4,
  kFTManulBoundaryId: 5
}
/**
 * 报警结果类型
 */
// export const alarmResultType = [
//   'close',
//   'popAndDicision',
//   'dicisionAndNoPop',
//   'popAndNoDicision',
//   'noPopAndNoDicision'
// ]
export const alarmResultType = new Map([
  [-1, 'close'],
  [0, 'close'],
  [1, 'popAndDicision'],
  [2, 'dicisionAndNoPop'],
  [3, 'popAndNoDicision'],
  [4, 'noPopAndNoDicision']
])
/**
 * 换型方式枚举
 */
export const remodelWay = [
  {
    id: 1,
    label: 'MES关联'
  },
  {
    id: 2,
    label: '程序号关联'
  },
  {
    id: 3,
    label: '指定变量关联'
  },
  {
    id: 4,
    label: '手动换型'
  }
]

/**
 * 计件类型枚举
 */
export const atbmTypeOptions = [
  {
    id: '1',
    label: '通用计件'
  },
  {
    id: '2',
    label: '智能计件'
  }
]

/**
 * 系统取值枚举
 */
export const coefSourceTypeOptions = [
  {
    id: '1',
    label: '宏变量'
  },
  {
    id: '2',
    label: 'IOM加工出数'
  }
]

/**
 * 设备运行状态枚举
 */
export const dccDevNcCheckRun = [
  {
    id: 0,
    label: '未定义'
  },
  {
    id: 1,
    label: '连接出错'
  },
  {
    id: 2,
    label: '自动运行开始'
  },
  {
    id: 3,
    label: '自动运行停止'
  },
  {
    id: 4,
    label: '自动运行待机状态'
  },
  {
    id: 5,
    label: '自动运行保持状态'
  }
]

/**
 * 机床类型枚举
 */
export const msimMachineTypeOptions = [
  {
    id: '1',
    label: '单机床'
  },
  {
    id: '2',
    label: '多机床'
  }
]

/**
 * 加工计件统计类型枚举
 */
export const saasBatchNumCalTypeOptions = [
  {
    id: '0',
    label: '件数变大则计数+1'
  },
  {
    id: '1',
    label: '件数+N则计数+N'
  }
]

/**
 * 服务器类型枚举
 */
export const progFileServerTypeOptions = [
  {
    id: '0',
    label: '私有化服务器'
  },
  {
    id: '1',
    label: '阿里云服务器'
  }
]

/**
 * NC文件下载方式类型枚举
 */
export const ncFileDownloadTypeOptions = [
  {
    id: '0',
    label: 'NC API'
  },
  {
    id: '1',
    label: 'FTP'
  }
]
