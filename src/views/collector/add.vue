<template>
  <div class="collector-add con-wrap">
    <myTabs
      :tabList="collectorList"
      :tabSelected="tabId"
      @changeTab="handleChangeTab"
      :itemName="{ label: 'display_name', value: 'type_id' }"
    />
    <div class="nc-wrap" v-if="tabName === 'NC'">
      <van-form colon label-align="right" label-width="168" ref="formRef">
        <van-field
          v-model="ruleForm.name"
          name="name"
          label="采集器名称"
          placeholder="请输入"
          maxlength="20"
          :rules="[{ required: true, message: '请输入采集器名称' }]"
        />
        <van-field label="设备厂家">
          <template #input>
            <van-field
              v-model="ruleForm.maker_text"
              name="maker_text"
              is-link
              readonly
              placeholder="请选择"
              @click="showPickerFn('maker')"
            />
          </template>
        </van-field>
        <van-field label="型号">
          <template #input>
            <van-field
              v-model="ruleForm.hardware_text"
              name="hardware_text"
              is-link
              readonly
              placeholder="请选择"
              @click="showPickerFn('hardware')"
            />
          </template>
        </van-field>

        <van-field
          v-model="ruleForm.ip"
          name="ip"
          label="IP地址"
          placeholder="请输入"
          :rules="[{ pattern: ipReg, message: '请输入正确的IP地址' }]"
        />
        <van-field
          v-model="ruleForm.port"
          name="port"
          label="端口号"
          type="digit"
          placeholder="请输入"
          :rules="[{ required: true, message: '请输入端口号' }, { validator: portValidator }]"
        />
        <van-field name="path_num" label="通道数">
          <template #input>
            <numberInput v-model="ruleForm.path_num" :min="1" :max="9" />
          </template>
        </van-field>

        <van-field
          v-model="ruleForm.freq"
          name="freq"
          label="采样频率"
          type="digit"
          placeholder="请输入"
          :rules="[{ required: true, message: '请输入采样频率' }]"
        />
      </van-form>
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
import { ref, onMounted, getCurrentInstance } from 'vue'
const { proxy } = getCurrentInstance()
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
const { devId } = storeToRefs(sysStore)
import { ipReg, portValidator } from '@src/utils/validate.js'
import numberInput from '@components/common/numberInput/index.vue'
import popover from '@components/common/popover/inedx.vue'
import myTabs from '@components/common/myTabs/index.vue'
import { useApi } from '@src/hooks/useApi'
const { fetchPostApi } = useApi()

const tabId = ref()
const tabName = ref('NC')
const handleChangeTab = item => {
  tabId.value = item.type_id
  tabName.value = item.display_name
}
const formRef = ref(null)
const ruleForm = ref({
  name: '',
  maker_id: 1,
  hardware_id: '',
  ip: '',
  port: '',
  path_num: 1,
  freq: ''
})

const protocolTypeList = [
  {
    type: 'rpc',
    value: '0'
  },
  {
    type: 'opc-ua',
    value: '1'
  },
  {
    type: 'opc-da',
    value: '2'
  },
  {
    type: 's7',
    value: '3'
  },
  {
    type: '530',
    value: '0'
  },
  {
    type: '640',
    value: '1'
  },
  {
    type: 'tcp',
    value: '0'
  },
  {
    type: 'ezsocket',
    value: '1'
  }
]
const protocolKeyList = [
  {
    type: 2,
    value: 'siemens_protocol'
  },
  {
    type: 3,
    value: 'mitsubishi_protocol'
  },
  {
    type: 4,
    value: 'haidehan_protocol'
  }
]
// 获取采集器类型
const collectorList = ref([])
const getCollectorList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_collector_type',
    id: '5',
    params: {}
  })
  collectorList.value = res.result.collector_type_list
  tabName.value = collectorList.value[0]?.name
  tabId.value = collectorList.value[0]?.type_id
  getMakerList(collectorList.value[0]?.type_id)
}

// 获取厂商
const makerList = ref([])
const getMakerList = async type_id => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_maker',
    id: '6',
    params: {
      // 采集器类型ID
      collector_type_id: type_id
    }
  })
  // 一期只要FANUC
  makerList.value = [res.result.maker_list[0]] || []
  ruleForm.value.maker_id = makerList.value[0].maker_id
  ruleForm.value.maker_text = makerList.value[0].display_name
  setProtocolList()
  getHardwareList()
}
// 协议下拉选项
const setProtocolList = () => {
  // rules.path_num[0].required = true
  // rules.freq[0].required = true
  if (ruleForm.value.maker_id === 2) {
    protocolOptions.value = [
      {
        label: 'RPC',
        value: 'rpc'
      },
      {
        label: 'OPC-UA',
        value: 'opc-ua'
      },
      {
        label: 'OPC-DA',
        value: 'opc-da'
      },
      {
        label: 'S7',
        value: 's7'
      }
    ]
    ruleForm.value.protocol = 'rpc'
  } else if (ruleForm.value.maker_id === 6) {
    protocolOptions.value = [
      {
        label: 'MATRIX',
        value: 'matrix'
      }
    ]
    ruleForm.value.protocol = 'matrix'
  } else if (ruleForm.value.maker_id === 4) {
    protocolOptions.value = [
      {
        label: '530',
        value: '530'
      },
      {
        label: '640',
        value: '640'
      }
    ]
    ruleForm.value.protocol = '530'
  } else if (ruleForm.value.maker_id === 3) {
    protocolOptions.value = [
      {
        label: 'TCP',
        value: 'tcp'
      },
      {
        label: 'EZSOCKET',
        value: 'ezsocket'
      }
    ]
    ruleForm.value.protocol = 'tcp'
  } else if (ruleForm.value.maker_id === 10) {
    // rules.path_num[0].required = false
    // rules.freq[0].required = false
    ruleForm.value.freq = 100
  }
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
      collector_type_id: tabId.value,
      // 厂商ID
      maker_id: ruleForm.value.maker_id
    }
  })
  hardwareList.value = res.result.hardware_list || []
  ruleForm.value.hardware_id = hardwareList.value[0].id
  ruleForm.value.hardware_text = hardwareList.value[0].display_name
  setProtocolList()
}
// 不同厂商显示不同的协议
const protocolOptions = ref([])
// const changeProtocol = () => {
//   ruleForm.value.opcda_server_name = ''
//   ruleForm.value.s7_ctrl_type = ''
//   ruleForm.value.mitsubishi_machine_id = ''
//   if (ruleForm.value.protocol === 'opc-da') {
//     ruleForm.value.opcda_server_name = 'OPC.SINUMERIK.Machineswitch'
//   } else if (ruleForm.value.protocol === 's7') {
//     ruleForm.value.s7_ctrl_type = '0'
//   } else if (ruleForm.value.protocol === 'ezsocket') {
//     ruleForm.value.mitsubishi_machine_id = '6'
//   }
// }
// // 获取当前厂商显示的字段
// const fieldsCanShow = (type, protocol) => {
//   if (tabName.value !== 'NC') {
//     return false
//   }
//   //1: fanuc 2: siemens 3: mitsubishi 4: haidehan
//   if (Array.isArray(type)) {
//     return type.includes(ruleForm.value.maker_id)
//   }
//   return type === ruleForm.value.maker_id && ruleForm.value.protocol === protocol
// }
const isNcAndSepical = temp_extra_params => {
  if (ruleForm.value.protocol === 'rpc') {
    temp_extra_params.push(
      {
        key: 'rpc_machine_id',
        value: ruleForm.value.rpc_machine_id
      },
      {
        key: 'rpc_machine_port',
        value: ruleForm.value.rpc_machine_port
      },
      {
        key: 'rpc_host_id',
        value: ruleForm.value.rpc_host_id
      },
      {
        key: 'rpc_host_port',
        value: ruleForm.value.rpc_host_port
      }
    )
  } else if (ruleForm.value.protocol === 'opc-ua') {
    temp_extra_params.push(
      {
        key: 'opcua_user_name',
        value: ruleForm.value.opcua_user_name
      },
      {
        key: 'opcua_password',
        value: ruleForm.value.opcua_password
      }
    )
  } else if (ruleForm.value.protocol === 'opc-da') {
    temp_extra_params.push({
      key: 'opcda_server_name',
      value: ruleForm.value.opcda_server_name
    })
  } else if (ruleForm.value.protocol === 's7') {
    temp_extra_params.push({
      key: 's7_ctrl_type',
      value: ruleForm.value.s7_ctrl_type
    })
  } else if (ruleForm.value.protocol === 'ezsocket') {
    temp_extra_params.push({
      key: 'mitsubishi_machine_id',
      value: ruleForm.value.mitsubishi_machine_id
    })
  }
}

const doAddColloctor = async (times, temp_extra_params) => {
  const tempAddId = []
  let isConnect = false
  for (let i = 1; i <= times; i++) {
    const res = await fetchPostApi({
      version: '1.0',
      method: 'add_adapter',
      id: '14',
      params: {
        dev_id: devId.value,
        collector_type_id: tabId.value,
        collect_freq: tabName.value === 'USV' ? 4000 : 1000,
        maker_id: parseInt(ruleForm.value.maker_id) || 0,
        hardware_id: parseInt(ruleForm.value.hardware_id) || 0,
        path_num: i,
        name: ruleForm.value.name || '',
        ip: ruleForm.value.ip || '',
        port: parseInt(ruleForm.value.port) || 0,
        freq: parseInt(ruleForm.value.freq) || 100,
        collect_type: undefined,
        collect_frequency: undefined,
        full_scale: undefined,
        extra_params: temp_extra_params
      }
    })
    tempAddId.push(res.result.adapter_info.id)

    // const loading = ElLoading.service({
    //   lock: true,
    //   text: t('collector.connectStatusList'),
    //   background: 'rgba(0, 0, 0, 0.7)'
    // })

    // 连接采集器
    try {
      await fetchPostApi({
        version: '1.0',
        method: 'adapter_connect',
        id: '52',
        params: {
          dev_id: devId.value,
          adapter_id: res.result.adapter_info.id
        }
      })
      await fetchPostApi({
        version: '1.0',
        method: 'add_nc_default_dynamic_signals',
        id: '529',
        params: {
          dev_id: devId.value,
          adapter_id: res.result.adapter_info.id
        }
      })
      isConnect = true
    } catch {
      isConnect = false
      // loading.close()
      // 没连接成功的需要全部删除
      for (const child of tempAddId) {
        await fetchPostApi({
          version: '1.0',
          method: 'delete_adapter',
          id: '16',
          params: {
            dev_id: devId.value,
            adapter_id: child
          }
        })
        // ElMessage.error('连接失败！')
      }
      // emit('update:isAddTool', false)
      break
    }
    // loading.close()
  }

  return {
    isConnect,
    tempAddId
  }
}
const handleFormValid = () => {
  formRef.value
    .validate()
    .then(() => {
      popoverRef.value.handleClick()
    })
    .catch(() => {})
}

// 表单提交
const handleSave = async () => {
  if (!formRef.value) return
  // 判断采集器名称有没有重复
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_adapters',
    id: '8',
    params: {
      dev_id: devId.value
    }
  })
  const adapterList = res.result.adapter_list

  if (adapterList.find(item => item.name === ruleForm.value.name)) {
    proxy.$alertMsg('clear', '', '该名称已存在，请设置新的名称！', { type: 'danger' })
    return
  }
  // 判断只能添加一个NC
  if (adapterList.find(item => item.collector_type_id === 1 && tabName.value === 'NC')) {
    proxy.$alertMsg('clear', '', 'NC采集器不能重复添加！', { type: 'danger' })
    return
  }

  formRef.value
    .validate()
    .then(() => {
      // 验证通过后的操作
      const times = parseInt(ruleForm.value.path_num) > 0 ? parseInt(ruleForm.value.path_num) : 1
      let isConnect = false
      const tempAddId = [] // 新增成功的id

      // 处理额外的扩展参数
      let temp_extra_params = []
      if (tabName.value === 'NC' && [2, 3, 4].includes(ruleForm.value.maker_id)) {
        temp_extra_params = [
          {
            key: protocolKeyList.find(item => item.type === ruleForm.value.maker_id).value,
            value: protocolTypeList.find(item => item.type === ruleForm.value.protocol).value
          }
        ]
        isNcAndSepical(temp_extra_params)
      } else if (tabName.value === 'NC' && ruleForm.value.maker_id === 30) {
        temp_extra_params = [
          {
            key: 'tcmachineid',
            value: ruleForm.value.tcmachineid
          }
        ]
      }
      console.log(1111, times, temp_extra_params)
      // return false
      doAddColloctor(times, temp_extra_params)
        .then(res => {
          isConnect = res.isConnect
          tempAddId.push(...res.tempAddId)
          if (isConnect) {
            proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
            setTimeout(() => {
              sessionStorage.setItem('newColllectId', JSON.stringify(tempAddId))
              history.back(-1)
            }, 50)
          } else {
            proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
          }
        })
        .catch(() => {
          proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
        })
    })
    .catch(() => {
      // 验证失败后的操作
      console.log('验证失败')
    })

  // return false
}
// 下拉框
const pickerType = ref('')
const pickerVal = ref([])
const pickerColumns = ref([])
const columnsNames = ref({ text: 'text', value: 'value' })
const showPicker = ref(false)

const onConfirm = ({ selectedOptions }) => {
  switch (pickerType.value) {
    case 'maker':
      ruleForm.value.maker_id = selectedOptions[0].maker_id
      ruleForm.value.maker_text = selectedOptions[0].display_name
      break
    case 'hardware':
      ruleForm.value.hardware_id = selectedOptions[0].id
      ruleForm.value.hardware_text = selectedOptions[0].display_name
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
      pickerVal.value = [ruleForm.value.maker_id]
      break
    case 'hardware':
      columnsNames.value = {
        text: 'display_name',
        value: 'id'
      }
      pickerColumns.value = JSON.parse(JSON.stringify(hardwareList.value))
      pickerVal.value = [ruleForm.value.hardware_id]
      break
  }
  showPicker.value = true
}

// 二次确认pop
const popoverRef = ref()
onMounted(() => {
  getCollectorList()
  asideList.value = [
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
                        handleSave()
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
})
</script>

<style lang="scss" scoped>
@import './style.scss';
.collector-add {
  .van-form {
    margin-top: 76px;
    padding-right: 70px;
    .van-cell {
      padding: 0 0 32px;
    }
  }
}
</style>
