<template>
  <div class="ch1-wrap">
    <div v-if="isEdit">
      <van-form colon label-align="right" label-width="180">
        <template v-for="child in signalsListItem" :key="child.sig_id + child.addr_type">
          <div class="conbine-input flex-start">
            <van-field
              v-model="collectTypesObj[child.addr_type]"
              is-link
              readonly
              :label="child.display_name"
              placeholder="请选择"
              @click="handleShowPicker(child)"
            />
            <van-field
              v-model="child.addr"
              name=""
              label=""
              :placeholder="child.addr_type === 0 ? '--' : '请输入'"
              :disabled="child.addr_type === 0"
            />
          </div>
        </template>
      </van-form>
    </div>
    <div class="list-con flex-start" v-else-if="signalsListItem.length">
      <template v-for="(child, i) in signalsListItem" :key="i">
        <div class="item">
          <div class="label">{{ child.display_name }}：</div>
          <div class="value">{{ child.realTimeData.val[0] }}</div>
        </div>
      </template>
    </div>

    <van-popup v-model:show="showPicker" round position="bottom">
      <van-picker
        option-height="54px"
        :columns="collectTypes"
        @cancel="showPicker = false"
        @confirm="onConfirm"
        v-model="curPicker"
      />
    </van-popup>
  </div>
</template>

<script setup lang="jsx">
import { ref, watch, onMounted, inject } from 'vue'
import { storeToRefs } from 'pinia'
import popover from '@components/common/popover/inedx.vue'
import { collectTypes, collectTypesObj } from '@src/utils/enum'
import { _public } from '@src/utils/common'
import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
const { devId } = storeToRefs(sysStore)
import { useMenuStore } from '@src/store/useMenu'
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const fetchPostApi = inject('fetchPostApi')
// import { useRouter } from 'vue-router'
// const router = useRouter()

const props = defineProps({
  // 信号数据源
  signalsList: {
    type: Array
  },
  signalsListItem: {
    type: Array
  },
  curAdapterId: {
    type: [String, Number]
  },
  tabType: {
    type: [String, Number]
  }
})

const curSignal = ref({})
const curPicker = ref([0])
const handleShowPicker = child => {
  curSignal.value = child
  curPicker.value = [child.addr_type]
  showPicker.value = true
}

const showPicker = ref(false)
const onConfirm = ({ selectedOptions }) => {
  curSignal.value.addr_type = selectedOptions[0].value
  if (curSignal.value.addr_type === 0) {
    curSignal.value.addr = ''
  }
  showPicker.value = false
  console.log(111, props.signalsList)
}
const isEdit = ref(false)

// const formData = ref({})
// 保存确认框
const popoverRef = ref()

const pathNumList = ref([]) // 所有的通道列表
// 当前需要展示通道下的指标
const tempSignal = ref([])
let copyTempSignal = []
// const tempSignalForm = ref({
//   tempSignal: tempSignal.value
// })
// 获取机床详情列表
const machineDetailList = ref([])
const getDevList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_dev_detail_info',
    id: '67',
    params: {}
  })
  machineDetailList.value = res.result.dev_info_list
  // 找出所有NC的通道
  const adapterInfoList = machineDetailList.value.find(item => item.dev_info.dev_id === devId.value)
  if (adapterInfoList) {
    adapterInfoList.adapter_info_list.forEach(val => {
      // = 1 代表是nc
      if (val.adapter_info.collector_type_id === 1) {
        pathNumList.value.push({
          id: val.adapter_info.id,
          path_num: val.adapter_info.path_num
        })
      }
    })
  }

  tempSignal.value = JSON.parse(
    JSON.stringify(
      props.signalsList[pathNumList.value.find(d => d.id === +props.curAdapterId).path_num - 1]
    )
  ).filter(item => {
    if (props.tabType === 3) {
      return item.sig_id < 1000
    } else {
      return item.sig_id >= 1000
    }
  })
  copyTempSignal = _public.deepCopy(tempSignal.value)
  // tempSignalForm.value.tempSignal = tempSignal.value
}
const findEditFields = () => {
  // console.log(9999888, copyTempSignal, props.signalsListItem)
  const result = []
  copyTempSignal.forEach(old => {
    props.signalsListItem.forEach(cur => {
      if (old.sig_id === cur.sig_id && !_public._equals(cur, old)) {
        result.push(cur)
      }
    })
  })
  return result
}
// 保存
const handleSave = async () => {
  const arr = findEditFields()
  try {
    for (const val of arr) {
      if ((val.addr && val.addr_type !== 0) || (!val.addr && val.addr_type !== 3)) {
        const tempId = pathNumList.value.filter(item => item.id === val.id)[0].id

        console.log(222222, arr, val, tempId)
        await fetchPostApi({
          version: '1.0',
          method: 'set_signal_info',
          id: '21',
          params: {
            dev_id: devId.value,
            adapter_id: parseInt(tempId),
            signal_info: {
              sig_id: val.sig_id,
              addr_type: val.addr_type,
              addr: val.addr.toString(),
              addr_len: isNaN(+val.addr_len) ? 10 : +val.addr_len
            }
          }
        })
      }
    }
  } catch {
    // loading.close()
  }
  // emit('reloadNC')
  // loading.close()
  // ElMessage.success(t('collector.operateSuccess'))
  // emit('update:isEditField', false)

  popoverRef.value.cancelFun()
  isEdit.value = false
}

watch(
  () => isEdit.value,
  val => {
    if (val) {
      asideList.value = [
        {
          key: 'back',
          text: '返回',
          cb: () => {
            isEdit.value = false
          },
          auth: '*' // 权限, *或者不提供该字段 为所有权限
        },
        {
          key: 'cancel',
          text: '取消',
          cb: () => {
            isEdit.value = false
          },
          auth: '*' // 权限, *或者不提供该字段 为所有权限
        },
        {
          key: 'save',
          text: '保存',
          cb: () => {
            // isEdit.value = false
          },
          render: () => {
            return (
              <popover ref={popoverRef} contentText="是否保存?" trigger="保存">
                {{
                  operate: () => {
                    return (
                      <>
                        <span
                          onClick={() => {
                            popoverRef.value.cancelFun()
                            isEdit.value = false
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
                  }
                }}
              </popover>
            )
          },
          auth: '*' // 权限, *或者不提供该字段 为所有权限
        }
      ]
    } else {
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
          key: 'edit',
          text: '编辑',
          cb: () => {
            isEdit.value = true
          },
          auth: '*' // 权限, *或者不提供该字段 为所有权限
        }
      ]
    }
  },
  {
    deep: true
  }
)

onMounted(() => {
  getDevList()
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
      key: 'edit',
      text: '编辑',
      cb: () => {
        isEdit.value = true
      },
      auth: '*' // 权限, *或者不提供该字段 为所有权限
    }
  ]
})
</script>

<style lang="scss" scoped>
@import '../style.scss';
.conbine-input {
  width: 50%;
  :deep(.van-cell) {
    width: auto !important;
    .van-cell__value {
      width: 170px;
    }
  }
}
.list-con {
  .item {
    width: 33.33%;
  }
  .label {
    width: 190px;
  }
}
</style>
