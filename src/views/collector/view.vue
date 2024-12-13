<template>
  <div class="collector-view con-wrap">
    <myTabs :tabList="tabList" :tabSelected="tabSelected" @changeTab="handleChangeTab" />
    <div class="view1" v-if="tabSelected === 1">
      <van-form colon label-align="right" label-width="168" ref="formRef">
        <div class="header-con flex-start">
          <van-field
            v-model="detailData.name"
            name="name"
            label="采集器名称"
            placeholder="请输入"
            maxlength="20"
            :rules="[{ required: true, message: '请输入采集器名称' }]"
          />
          <div class="ml40 mb24">采集器类型：{{ collectorTypeName }}</div>
          <div class="ml40 mb24 flex-start">
            <div>采集器状态：</div>
            <img src="@src/assets/icons/svg/connect2.svg" alt="" v-if="detailData.connect_status" />
            <img src="@src/assets/icons/svg/connect1.svg" alt="" v-else />
          </div>
        </div>
        <div class="form-block flex-start mt24">
          <van-field label="设备厂家">
            <template #input>
              <van-field
                v-model="detailData.maker_text"
                name="maker_text"
                is-link
                readonly
                placeholder="请选择"
                @click="showPickerFn('maker')"
                disabled
              />
            </template>
          </van-field>
          <van-field
            v-model="detailData.ip"
            name="ip"
            label="IP地址"
            placeholder="请输入"
            :rules="[{ pattern: ipReg, message: '请输入正确的IP地址' }]"
          />
          <van-field label="型号">
            <template #input>
              <van-field
                v-model="detailData.hardware_text"
                name="hardware_text"
                is-link
                readonly
                placeholder="请选择"
                @click="showPickerFn('hardware')"
                disabled
              />
            </template>
          </van-field>

          <van-field name="path_num" label="通道数">
            <template #input>
              <numberInput v-model="detailData.path_num" :min="1" :max="9" isDisabled />
            </template>
          </van-field>
          <van-field
            v-model="detailData.port"
            name="port"
            label="端口号"
            type="digit"
            placeholder="请输入"
            :rules="[{ required: true, message: '请输入端口号' }, { validator: portValidator }]"
          />
          <van-field
            v-model="detailData.freq"
            name="freq"
            label="采样频率"
            type="digit"
            placeholder="请输入"
            :rules="[{ required: true, message: '请输入采样频率' }]"
          />
        </div>
      </van-form>
    </div>
    <div class="view" v-if="tabSelected === 2">
      <van-form colon label-align="right" label-width="168" ref="controlFormRef">
        <div class="conbine-input flex-start">
          <van-field label="控制开关地址">
            <template #input>
              <van-field
                v-model="controlOnOff.ctrl_addr_type_name"
                is-link
                readonly
                placeholder="请选择"
                @click="showControlPicker(1)"
              />
            </template>
          </van-field>

          <van-field
            v-model="controlOnOff.ctrl_addr"
            name=""
            label=""
            placeholder="请输入"
            :rules="[{ required: true, message: '请输入' }]"
          />
        </div>
        <!-- 一期不做 -->
        <!-- <div class="conbine-input flex-start">
          <van-field
            v-model="controlSpeed.ctrl_addr_type_name"
            is-link
            readonly
            label="控制转速地址"
            placeholder="请选择"
            @click="showControlPicker(2)"
          />
          <van-field v-model="controlSpeed.ctrl_addr" name="" label="" placeholder="请输入" :rules="[{ required: true, message: '请输入' }]" />
        </div> -->
        <div class="conbine-input flex-start">
          <van-field label="控制进给地址">
            <template #input>
              <van-field
                v-model="controlFeed.ctrl_addr_type_name"
                is-link
                readonly
                placeholder="请选择"
                @click="showControlPicker(3)"
              />
            </template>
          </van-field>

          <van-field
            v-model="controlFeed.ctrl_addr"
            name=""
            label=""
            placeholder="请输入"
            :rules="[{ required: true, message: '请输入' }]"
          />
        </div>
        <van-popup v-model:show="controlPicker" round position="bottom">
          <van-picker
            option-height="54px"
            :columns="controlList"
            @cancel="controlPicker = false"
            @confirm="onConfirmControl"
            v-model="controlPickerVal"
          />
        </van-popup>
      </van-form>
    </div>
    <div class="view" v-if="tabSelected === 3">
      <Ch1Wrap
        :signalsList="signalsList"
        :signalsListItem="signalsListCH"
        :curAdapterId="curAdapter"
        :tabType="tabSelected"
        @reloadNC="reloadNC"
      />
    </div>
    <div class="view" v-if="tabSelected === 4">
      <!-- <ExtendedFields /> -->
      <Ch1Wrap
        :signalsList="signalsList"
        :signalsListItem="signalsListExtend"
        :curAdapterId="curAdapter"
        :tabType="tabSelected"
        @reloadNC="reloadNC"
      />
    </div>
    <van-popup v-model:show="showPicker" round position="bottom">
      <van-picker
        option-height="54px"
        :columns="pickerColumns"
        :columns-field-names="columnsNames"
        @cancel="showPicker = false"
        @confirm="onConfirm"
        v-model="pickerVal"
      />
    </van-popup>
  </div>
</template>

<script setup lang="jsx">
import {
  ref,
  onMounted,
  watch,
  inject,
  getCurrentInstance,
  computed,
  onBeforeUnmount,
  provide
} from 'vue'
const { proxy } = getCurrentInstance()
import { storeToRefs } from 'pinia'
import numberInput from '@components/common/numberInput/index.vue'
import popover from '@components/common/popover/inedx.vue'
import Ch1Wrap from './components/Ch1Wrap.vue'
// import ExtendedFields from './components/ExtendedFields.vue'
import { useApi } from '@src/hooks/useApi'
const { fetchPostApi } = useApi()
provide('fetchPostApi', fetchPostApi)
import { useMenuStore } from '@src/store/useMenu'
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
const { devId } = storeToRefs(sysStore)
import { useSignalController } from '@src/hooks/useSignalController'
const { signal } = useSignalController()
const _worker = inject('_worker')
import { useStoreSignal } from '@src/store/useSignals'
const storeSignal = useStoreSignal()
import { ipReg, portValidator } from '@src/utils/validate.js'
import { dccDevNcCheckRun, controlList } from '@src/utils/enum'
import { _public } from '@src/utils/common'
import { useRoute } from 'vue-router'
const route = useRoute()
const adapterIds = route.query.adapter_id
let curAdapter = Array.isArray(adapterIds) ? +route.query.adapter_id[0] : +adapterIds

_worker.addEventListener(
  'message',
  res => {
    const { type, payload } = res.data
    switch (type) {
      // ------ 实时数据回调
      case 'StartPushData':
        // 找到当前实时数据推送的数据是哪个信号，替换对应信号的值
        // 后端设计如此，前端要取出来匹配很复杂...
        console.log('-----------------实时数据列表', payload, signalsList.value)
        for (const i in signalsList.value) {
          for (const [key, value] of Object.entries(payload)) {
            if (+signalsList.value[i][0].id === +key) {
              for (const [childKey, childValue] of Object.entries(value.metrics)) {
                let val = ''
                switch (childValue.sig_id) {
                  // 设备运行状态从枚举中取出id对应的label并赋值
                  case 33:
                    val = dccDevNcCheckRun.find(d => d.id === childValue.val.Integer)?.label
                    signalsList.value[i][
                      findSigId(signalsList.value[i], childKey)
                    ].realTimeData.val[0] = val
                    break
                  // FEATURE 后续需要取枚举对应的label可以往下加
                  default:
                    setSignalData(childValue, childKey, i)
                    break
                }
              }
            }
          }
        }
        storeSignal.changeSignalList(signalsList.value)
        break
    }
  },
  { signal }
)

const setSignalData = (childValue, childKey, i) => {
  let data = null
  if (childValue.sig_data_type === 0) {
    data = childValue.val.Integer
  } else if (childValue.sig_data_type === 1) {
    data = childValue.val.Float.toFixed(4)
  } else if (childValue.sig_data_type === 3) {
    data = childValue.val.IntSingleValue
  } else {
    data = childValue.val.String
  }
  signalsList.value[i][findSigId(signalsList.value[i], childKey)].realTimeData.val[0] = data
}

const tabList = [
  {
    label: '硬件信息',
    value: 1
  },
  {
    label: '控制地址',
    value: 2
  },
  {
    label: 'Ch1',
    value: 3
  },
  {
    label: '扩展字段',
    value: 4
  }
]
const tabSelected = ref(1)
const handleChangeTab = item => {
  tabSelected.value = item.value
}

// 获取厂商
const makerList = ref([])
const getMakerList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_maker',
    id: '6',
    params: {
      // 采集器类型ID
      collector_type_id: collectorTypeId.value
    }
  })
  // 一期只要FANUC
  makerList.value = [res.result.maker_list[0]] || []
  detailData.value.maker_text = makerList.value.find(
    item => item.maker_id === detailData.value.maker_id
  )?.display_name
  getHardwareList()
}
// 获取型号
const hardwareList = ref([])
const getHardwareList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_hardware',
    id: '7',
    params: {
      // 采集器类型ID
      collector_type_id: collectorTypeId.value,
      // 厂商ID
      maker_id: detailData.value.maker_id
    }
  })
  hardwareList.value = res.result.hardware_list || []
  detailData.value.hardware_text = hardwareList.value.find(
    item => item.id === detailData.value.hardware_id
  )?.display_name
}
// 硬件信息--下拉框
const pickerType = ref('')
const pickerVal = ref([])
const pickerColumns = ref([])
const columnsNames = ref({ text: 'text', value: 'value' })
const showPicker = ref(false)
const onConfirm = ({ selectedOptions }) => {
  switch (pickerType.value) {
    case 'maker':
      detailData.value.maker_id = selectedOptions[0].maker_id
      detailData.value.maker_text = selectedOptions[0].display_name
      break
    case 'hardware':
      detailData.value.hardware_id = selectedOptions[0].id
      detailData.value.hardware_text = selectedOptions[0].display_name
      break
  }
  showPicker.value = false
}

const showPickerFn = type => {
  pickerType.value = type
  switch (type) {
    case 'maker':
      columnsNames.value = {
        text: 'display_name',
        value: 'maker_id'
      }
      pickerColumns.value = JSON.parse(JSON.stringify(makerList.value))
      pickerVal.value = [detailData.value.maker_id]
      break
    case 'hardware':
      columnsNames.value = {
        text: 'display_name',
        value: 'id'
      }
      pickerColumns.value = JSON.parse(JSON.stringify(hardwareList.value))
      pickerVal.value = [detailData.value.hardware_id]
      break
  }
  showPicker.value = true
}

// 控制开关
const controlOnOff = ref({
  ctrl_type: 0,
  ctrl_addr_type: 0,
  ctrl_addr_type_name: 'PMC',
  ctrl_addr: ''
})
// 控制转速
const controlSpeed = ref({
  ctrl_type: 2,
  ctrl_addr_type: 0,
  ctrl_addr_type_name: 'PMC',
  ctrl_addr: ''
})
// 控制进给
const controlFeed = ref({
  ctrl_type: 1,
  ctrl_addr_type: 0,
  ctrl_addr_type_name: 'PMC',
  ctrl_addr: ''
})
const controlPicker = ref(false)
const controlPickerVal = ref([0])
const controlType = ref(0)
const showControlPicker = type => {
  controlType.value = type
  switch (type) {
    case 1:
      controlPickerVal.value = [controlOnOff.value.ctrl_addr_type]
      break
    case 2:
      controlPickerVal.value = [controlSpeed.value.ctrl_addr_type]
      break
    case 3:
      controlPickerVal.value = [controlFeed.value.ctrl_addr_type]
      break
  }
  controlPicker.value = true
}
const onConfirmControl = ({ selectedOptions }) => {
  const { value, text } = selectedOptions[0]
  switch (controlType.value) {
    case 1:
      controlOnOff.value.ctrl_addr_type_name = text
      controlOnOff.value.ctrl_addr_type = value
      break
    case 2:
      controlSpeed.value.ctrl_addr_type_name = text
      controlSpeed.value.ctrl_addr_type = value
      break
    case 3:
      controlFeed.value.ctrl_addr_type_name = text
      controlFeed.value.ctrl_addr_type = value
      break
  }
  controlPicker.value = false
}

// 查询采集器的倍率控制参数
const getRateCtrlParams = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'query_adapter_rate_ctrl_params',
    id: '530',
    params: {
      dev_id: devId.value,
      adapter_id: +curAdapter
    }
  })
  const tempList = res.result.adapter_rate_ctrl_params
  if (tempList.length) {
    const temp0 = tempList.find(item => item.ctrl_type === 0)
    if (temp0) {
      controlOnOff.value = temp0
      controlOnOff.value.ctrl_addr_type_name =
        controlList.find(item => item.value === temp0.ctrl_addr_type)?.text || ''
    }
    const temp1 = tempList.find(item => item.ctrl_type === 1)
    if (temp1) {
      controlFeed.value = temp1
      controlFeed.value.ctrl_addr_type_name =
        controlList.find(item => item.value === temp1.ctrl_addr_type)?.text || ''
    }
    const temp2 = tempList.find(item => item.ctrl_type === 2)
    if (temp2) {
      controlSpeed.value = temp2
      controlSpeed.value.ctrl_addr_type_name =
        controlList.find(item => item.value === temp2.ctrl_addr_type)?.text || ''
    }
  }
}

// 设置采集器的倍率控制参数
const setRateCtrlParams = async () => {
  const params = [{ ...controlOnOff.value }, { ...controlSpeed.value }, { ...controlFeed.value }]
  const res = await fetchPostApi({
    version: '1.0',
    method: 'set_adapter_rate_ctrl_params',
    id: '531',
    params: {
      dev_id: devId.value,
      adapter_id: +curAdapter,
      adapter_rate_ctrl_params: params
    }
  })
  if (res.result.adapter_id) {
    proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
  } else {
    proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
  }
}
// 硬件信息form
const formRef = ref(null)
// 控制地址
const controlFormRef = ref(null)
const handleFormValid = () => {
  if (tabSelected.value === 1) {
    // 硬件信息
    formRef.value
      .validate()
      .then(() => {
        popoverRef.value.handleClick()
      })
      .catch(() => {})
  } else if (tabSelected.value === 2) {
    // 控制地址
    controlFormRef.value
      .validate()
      .then(() => {
        popoverRef.value.handleClick()
      })
      .catch(() => {})
  }
}
const editDetail = async () => {
  // 判断采集器名称有没有重复
  if (adapterInitName !== detailData.value.name) {
    if (adapterList.value.find(item => item.name === detailData.value.name)) {
      proxy.$alertMsg('clear', '', '该名称已存在，请设置新的名称！', { type: 'danger' })
      return
    }
  }

  let addTimes = 0 // 需要调用几次新增接口
  let editTimes = 1 // 调用几次编辑接口
  let deleteTimes = 0 // 调用几次删除接口
  pathNum = tempPathNumList.value?.length
  if (detailData.value.collector_type_name === 'NC') {
    // 如果通道数比如 3 -> 5, 那么需要调用3遍编辑，2遍新增
    // 如果通道数比如 5 -> 3, 那么需要调用2遍删除
    if (pathNum > detailData.value.path_num) {
      // 减少了通道数
      deleteTimes = pathNum - detailData.value.path_num
      editTimes = pathNum - deleteTimes
    } else if (pathNum < detailData.value.path_num) {
      // 增加了通道数
      addTimes = detailData.value.path_num - pathNum
      editTimes = pathNum + addTimes
    }
  }

  // 编辑接口
  const temp_extra_param = []
  for (let i = 0; i < editTimes; i++) {
    await fetchPostApi({
      version: '1.0',
      method: 'update_adapter',
      id: '15',
      params: {
        dev_id: devId.value,
        adapter_info: {
          id: tempPathNumList.value?.[i]?.id || parseInt(curAdapter),
          collector_type_id: parseInt(detailData.value.collector_type_id),
          collect_freq: detailData.value.collector_type_name === 'USV' ? 4000 : 1000,
          maker_id: parseInt(detailData.value.maker_id) || 0,
          hardware_id: parseInt(detailData.value.hardware_id) || 0,
          path_num: i + 1,
          name: detailData.value.name || '',
          ip: detailData.value.ip || '',
          port: parseInt(detailData.value.port) || 0,
          freq: parseInt(detailData.value.freq) || 10,
          adapter_type: detailData.value.collector_type_name,
          extra_param: temp_extra_param
        }
      }
    })
    try {
      await fetchPostApi({
        version: '1.0',
        method: 'adapter_connect',
        id: '52',
        params: {
          dev_id: devId.value,
          adapter_id: tempPathNumList.value?.[i]?.id || parseInt(curAdapter)
        }
      })
    } catch {
      // loading.close()
      // ElMessage.error(t('collector.connectFail'))
    }
  }
  proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
  getDetail()
}

// 保存
const handleSave = () => {
  if (tabSelected.value === 1) {
    // 硬件信息
    editDetail()
  } else if (tabSelected.value === 2) {
    // 控制地址
    setRateCtrlParams()
  }
  // else if (tabSelected.value === 3) {
  //   // CH1
  // } else if (tabSelected.value === 4) {
  //   // 扩展字段
  // }
  popoverRef.value.cancelFun()
}

// 获取采集器类型
const collectorList = ref([])
const collectorTypeId = ref(0)
const collectorTypeName = ref('')
const getCollectorList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_collector_type',
    id: '5',
    params: {}
  })
  collectorList.value = res.result.collector_type_list
  const temp = collectorList.value.find(
    item => item.type_id === +detailData.value.collector_type_id
  )
  collectorTypeId.value = temp?.type_id || 0
  collectorTypeName.value = temp?.display_name || ''
  if (collectorTypeId.value) {
    getMakerList()
  }
}

// 详情数据
// const connetWay = ref(1) // 连接方式
const collectMode = ref('20') // 采集模式
const detailData = ref({})
const tempPathNumList = ref([])
const adapterList = ref([])
let adapterInitName = '' // 采集器没修改前的名字
let pathNum = 0
const getDetail = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_adapters',
    id: '8',
    params: {
      dev_id: devId.value
    }
  })
  tempPathNumList.value = []
  adapterList.value = res.result.adapter_list
  detailData.value = res.result.adapter_list.filter(item => {
    if (item.collector_type_id === 1) {
      tempPathNumList.value.push(item)
    }
    return item.id === curAdapter
  })[0]
  detailData.value.path_num = tempPathNumList.value?.length || 0
  adapterInitName = detailData.value.name
  // 获取采集器状态
  const resStatus = await fetchPostApi({
    version: '1.0',
    method: 'get_adapter_status',
    id: '46',
    params: {
      dev_id: parseInt(devId.value),
      adapter_id: +curAdapter
    }
  })
  detailData.value = {
    ...detailData.value,
    connect_status: resStatus.result.connect_status,
    connect_status_name: resStatus.result.connect_status ? '已连接' : '未连接',
    collector_type_name: collectorList.value.filter(
      val => val.type_id === +detailData.value.collector_type_id
    )[0].name,
    collectMode: collectMode.value
  }
  getCollectorList()
  getDevList()
}

// 获取机床详情列表
const machineDetailList = ref([])
const machineDetail = ref([]) // 当前机床当前采集器数据
const pathNumList = ref([])
const getDevList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_dev_detail_info',
    id: '67',
    params: {}
  })
  machineDetailList.value = res.result.dev_info_list
  pathNumList.value = []
  machineDetail.value = []
  // 找出所有NC的通道
  for (const item of machineDetailList.value) {
    if (item.dev_info.dev_id === devId.value && item.dev_info.dev_type === 'ARM') {
      for (const val of item.adapter_info_list) {
        curAdapter = +adapterIds.find(d => +d === val.adapter_info.id)
        if (curAdapter) {
          machineDetail.value.push({
            ...val,
            adapter_id: +curAdapter
          })
        }
        // = 1 代表是nc
        if (val.adapter_info.collector_type_id === 1) {
          // 代表是 NC
          pathNumList.value.push({
            id: val.adapter_info.id,
            path_num: val.adapter_info.path_num
          })
        }
      }
    }
  }

  detailData.value.path_num = pathNumList.value.length
  unsubscribeRealTime()
  // 订阅实时数据
  getRealTime()
}
let signalsList = ref([]) // 信号列表
const signalsListCH = computed(() => {
  let temp = []
  if (signalsList.value[0]) {
    temp = signalsList.value[0].filter(item => item.sig_id < 1000)
  }
  return temp
})
const signalsListExtend = computed(() => {
  let temp = []
  if (signalsList.value[0]) {
    temp = signalsList.value[0].filter(item => item.sig_id >= 1000)
  }
  return temp
})
// 准备订阅实时数据
const getRealTime = async callback => {
  // ----------------- 获取采集器信号列表
  signalsList.value = []
  // 获取信号详情（包括采集方式地址等等）
  const path_num = detailData.value.path_num || 1
  for (let i = 0; i < path_num; i++) {
    // 采集器id
    const tempId = pathNumList.value.filter(child => +child.path_num === i + 1)[0].id

    const resSignalDetail = await fetchPostApi({
      version: '1.0',
      method: 'enum_collector_detail_signal_info',
      id: '68',
      params: {
        dev_id: devId.value,
        adapter_id: parseInt(tempId)
      }
    })
    signalsList.value.push(resSignalDetail.result.signal_info_list)

    // 获取采集器信号信息枚举（一些比如中文是不会返回在详情接口的，需要从枚举中拿）
    const resMenu = await fetchPostApi({
      version: '1.0',
      method: 'enum_signals',
      id: '4',
      params: {}
    })

    const tempList = [...resMenu.result.fixed_signals, ...resMenu.result.dynamic_signals]

    // 格式化信号枚举（通过信号id -> 信号obj）
    for (const j in signalsList.value[i]) {
      signalsList.value[i][j] = {
        id: tempId,
        ...signalsList.value[i][j],
        ..._public.findObj(tempList, 'sig_id', signalsList.value[i][j].sig_id),
        // isChecked: 1,
        // isChecked: storeSignal.signalEnumList
        //   ? storeSignal.signalEnumList[i].filter(item => item.isChecked)[0]?.isChecked
        //   : 1,
        // machineDetail.value.enable_signal_id_list：已经采集的信号
        isChecked: machineDetail.value
          .find(v => v.adapter_id === tempId)
          .enable_signal_id_list.includes(signalsList.value[i][j].sig_id)
          ? 1
          : 0,
        // 初始化 - 为了v-model不报错
        realTimeData: {
          val: ['--']
        }
      }
    }
  }
  storeSignal.changeSignalList(signalsList.value)
  if (callback) {
    callback()
  }

  // 开始订阅
  subscribeRealTime()
}

// 找到当前的sigId对应的下标
const findSigId = (arr, sig_id) => {
  for (const i in arr) {
    if (+arr[i].sig_id === +sig_id) {
      return i
    }
  }
}

// 订阅实时数据
const subscribeRealTime = async () => {
  if (!sessionStorage.getItem('token')) {
    return false
  }
  // ------------ 开始订阅数据
  for (const adapter_id of adapterIds) {
    await fetchPostApi({
      version: '1.0',
      method: 'subscribe_single_signal',
      id: '32',
      params: {
        dev_id: devId.value,
        adapter_id: parseInt(adapter_id),
        sig_id_list: [],
        token: sessionStorage.getItem('token')
        // token: sessionStorage.getItem('token') || '0DBF5237-382E-43ad-92A6-7050009BEC29'
      }
    })
  }
  _worker.postMessage({
    type: 'startPushData',
    payload: {}
  })
}
// 刷新NC实时信息指标
const reloadNC = async () => {
  await unsubscribeRealTime()
  storeSignal.changeSignalList(null)
  // 订阅实时数据
  getRealTime()
}
// 停止订阅实时数据
const unsubscribeRealTime = async () => {
  // 退订实时数据
  _worker.postMessage({
    type: 'stopPushData',
    payload: {}
  })
  // 取消订阅实时数据
  for (const adapter_id of adapterIds) {
    await fetchPostApi({
      version: '1.0',
      method: 'unsubscribe_single_signal',
      id: '33',
      params: {
        dev_id: devId.value,
        adapter_id: parseInt(adapter_id),
        token: sessionStorage.getItem('token')
      }
    })
  }
}
const popoverRef = ref()
const asideTemp = [
  {
    key: 'back',
    text: '返回',
    cb: () => {
      history.back(-1)
    },
    auth: '*' // 权限, *或者不提供该字段 为所有权限
  },
  {
    key: 'save',
    text: '保存',
    cb: () => {},
    render: () => {
      return (
        <popover ref={popoverRef} contentText="是否保存?">
          {{
            operate: () => {
              return (
                <>
                  <span
                    onClick={() => {
                      popoverRef.value.cancelFun()
                    }}
                  >
                    取消
                  </span>
                  <span
                    onClick={() => {
                      {
                        handleSave()
                      }
                    }}
                  >
                    确认
                  </span>
                </>
              )
            },
            referBtn: () => {
              return <div onClick={() => handleFormValid()}>保存</div>
            }
          }}
        </popover>
      )
    },
    auth: '*' // 权限, *或者不提供该字段 为所有权限
  }
]
watch(
  () => tabSelected.value,
  val => {
    if (val === 1 || val === 2) {
      asideList.value = [...asideTemp]
    }
  },
  {
    deep: true
  }
)

onMounted(() => {
  getDetail()
  getCollectorList()
  getRateCtrlParams()
  asideList.value = [...asideTemp]
})
onBeforeUnmount(() => {
  unsubscribeRealTime()
})
</script>

<style lang="scss" scoped>
@import './style.scss';
.collector-view {
  .header-con {
    width: 100%;
    padding: 24px 70px 0 0;
    padding-right: 70px;
    border-bottom: 1px solid rgb(225, 225, 225);
  }
  .form-block {
    flex-wrap: wrap;
    margin-right: 70px;
  }
  .view {
    padding: 24px;
    .conbine-input {
      :deep(.van-cell) {
        width: auto;
        .van-cell__value {
          width: 217px;
        }
      }
    }
  }
}
</style>
