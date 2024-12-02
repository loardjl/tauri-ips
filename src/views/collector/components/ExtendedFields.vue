<template>
  <div class="ch1-wrap">
    <div v-if="isEdit">
      <van-form colon label-align="right" label-width="168">
        <div class="conbine-input flex-start" v-for="item in extendedList" :key="item.name">
          <van-field
            v-model="extendedList.username"
            is-link
            readonly
            label="控制开关地址"
            placeholder="请选择"
            @click="showPicker = true"
          />

          <van-field v-model="extendedList.name" name="" label="" placeholder="请输入" />
        </div>
      </van-form>
    </div>
    <div class="list-con flex-start" v-else>
      <div class="item" v-for="item in extendedList" :key="item.name">
        <div class="label">{{ item.name }}：</div>
        <div class="value">{{ item.value }}</div>
      </div>
    </div>
    <van-popup v-model:show="showPicker" round position="bottom">
      <van-picker
        option-height="54px"
        :columns="pickerList"
        @cancel="showPicker = false"
        @confirm="onConfirm"
      />
    </van-popup>
  </div>
</template>

<script setup lang="jsx">
import { ref, watch, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import popover from '@components/common/popover/inedx.vue'

// import { useRouter } from 'vue-router'

// const router = useRouter()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)

// const { worker } = useSignalController()
const extendedList = ref([
  { id: 1, name: '字段1', value: 1 },
  { id: 1, name: '字段2', value: 2 }
])
const isEdit = ref(false)

const showPicker = ref(false)
const pickerList = ref([{ text: '杭州', value: 'Hangzhou' }])
const onConfirm = () => {
  console.log(999)
}

const popoverRef = ref()
// 保存
const handleSave = () => {
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
            isEdit.value = false
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
  :deep(.van-cell) {
    width: auto !important;
    .van-cell__value {
      width: 257px;
    }
  }
}
</style>
