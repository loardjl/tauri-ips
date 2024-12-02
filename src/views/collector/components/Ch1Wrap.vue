<template>
  <div class="ch1-wrap">
    <div v-if="isEdit">
      <van-form colon label-align="right" label-width="180">
        <div
          class="conbine-input flex-start"
          v-for="child in signalsList[0]"
          :key="child.sig_id + child.addr_type"
        >
          <van-field
            v-model="controlListObj[child.addr_type]"
            is-link
            readonly
            :label="child.display_name"
            placeholder="请选择"
            @click="handleShowPicker(child)"
          />
          <van-field v-model="child.addr" name="" label="" placeholder="请输入" />
        </div>
      </van-form>
    </div>
    <div class="list-con flex-start" v-else-if="signalsList.length">
      <div class="item" v-for="(child, i) in signalsList[0]" :key="i">
        <div class="label">{{ child.display_name }}：</div>
        <div class="value">{{ child.realTimeData.val[0] }}</div>
      </div>
    </div>

    <van-popup v-model:show="showPicker" round position="bottom">
      <van-picker
        option-height="54px"
        :columns="controlList"
        @cancel="showPicker = false"
        @confirm="onConfirm"
        v-model="curPicker"
      />
    </van-popup>
  </div>
</template>

<script setup lang="jsx">
import { ref, watch, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import popover from '@components/common/popover/inedx.vue'
import { controlList, controlListObj } from '@src/utils/enum'
import { useApi } from '@src/hooks/useApi'
const { fetchPostApi } = useApi()
import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
const { devId } = storeToRefs(sysStore)
import { useMenuStore } from '@src/store/useMenu'
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
// import { useRouter } from 'vue-router'
// const router = useRouter()

console.log(9999, controlListObj, fetchPostApi, devId.value)
const props = defineProps({
  // 信号数据源
  signalsList: {
    type: Array
  },
  curAdapterId: {
    type: String
  }
})
console.log(9999, props.signalsList)

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
  showPicker.value = false
  console.log(111, props.signalsList)
}
const isEdit = ref(false)

// const formData = ref({})
// 保存确认框
const popoverRef = ref()
const findEditFields = () => {
  const result = []
  // copyTempSignal.forEach(old => {
  //   tempSignalForm.value.tempSignal.forEach(cur => {
  //     if (old.sig_id === cur.sig_id && !_public._equals(cur, old)) {
  //       result.push(cur)
  //     }
  //   })
  // })
  return result
}
// 保存
const handleSave = async () => {
  const arr = findEditFields()
  try {
    for (const val of arr) {
      if ((val.addr && val.addr_type !== 0) || (!val.addr && val.addr_type !== 3)) {
        // const tempId = pathNumList.value.filter(item => item.id === val.id)[0].id
        // await fetchPostApi({
        //   version: '1.0',
        //   method: 'set_signal_info',
        //   id: '21',
        //   params: {
        //     dev_id: devId.value,
        //     adapter_id: parseInt(tempId),
        //     signal_info: {
        //       sig_id: val.sig_id,
        //       addr_type: val.addr_type,
        //       addr: val.addr.toString(),
        //       addr_len: isNaN(+val.addr_len) ? 10 : +val.addr_len
        //     }
        //   }
        // })
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
      // const temp = []
      // props.signalsList.forEach(item => {
      //   temp.push({
      //     sig_id: item.sig_id,
      //     addr_type: item.addr_type,
      //     value: item.addr
      //   })
      // })
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
</style>
