<template>
  <div class="learningStyle">
    <div class="header">
      <div>
        <span>学习方式:</span>
        <div class="selsect">
          <van-field
            v-model="learning"
            is-link
            readonly
            placeholder="请选择"
            @click="showPicker = true"
          />
          <van-popup v-model:show="showPicker" round position="bottom">
            <van-picker
              option-height="54px"
              :columns="study_method_list"
              @cancel="showPicker = false"
              @confirm="onprogramConfirm"
              v-model="selectedValues"
            />
          </van-popup>
        </div>
      </div>
      <div>
        <span>学习次数:</span>
        <div class="num">
          <numberInput v-model="studymethodlistItem.study_count" :min="1" :max="99" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="jsx">
import { onMounted, ref, getCurrentInstance } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import numberInput from '@src/components/common/numberInput/index.vue'
import { useApi } from '@src/hooks/useApi'
import popover from '@components/common/popover/inedx.vue'
const { fetchPostApi } = useApi()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const { proxy } = getCurrentInstance()
onMounted(() => {
  asideList.value = [
    {
      key: 'relearn',
      text: '返回',
      cb: () => {
        history.back(-1)
      }
    },
    {
      key: 'save',
      text: ' ',
      render: () => {
        return (
          <popover ref={popoverRef} contentText="是否保存学习方式?" trigger="保存">
            {{
              operate: () => {
                return (
                  <>
                    <span
                      onClick={() => {
                        popoverRef.value.cancelFun()
                        getstudymethod()
                      }}
                    >
                      取消
                    </span>
                    <span
                      onClick={() => {
                        save()
                      }}
                    >
                      保存
                    </span>
                  </>
                )
              }
            }}
          </popover>
        )
      }
    }
  ]
  getstudymethod()
})

const showPicker = ref(false)
const learning = ref('')
const selectedValues = ref([])
const study_id = ref(0)
const onprogramConfirm = ({ selectedOptions }) => {
  showPicker.value = false
  learning.value = selectedOptions[0].text
  selectedValues.value = [selectedOptions[0].study_id]
}
const study_method_list = ref([
  {
    text: '学习次数',
    value: 0
  }
])
const studymethodlistItem = ref({})
const getstudymethod = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_study_method',
        id: '9',
        params: {
          program_num: JSON.parse(history.state.program_num)
        }
      },
      'ipsbatch'
    )
    const data = res
    selectedValues.value = [study_id.value]
    studymethodlistItem.value = data.result.study_method_list.filter(
      item => item.study_id === study_id.value
    )[0]
    learning.value = study_method_list.value[0].text
  } catch (e) {
    console.log(e)
  }
}
const popoverRef = ref(null)
const save = async () => {
  studymethodlistItem.value.study_count = +studymethodlistItem.value.study_count
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'set_study_method',
        id: '10',
        params: JSON.parse(JSON.stringify(studymethodlistItem.value))
      },
      'ipsbatch'
    )
    const data = res
    popoverRef.value.confirmFun()
    if (!data.result.status) proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
    else proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
  } catch (e) {
    console.log(e)
  }
}
</script>

<style lang="scss" scoped>
.learningStyle {
  background-color: rgb(241, 241, 241);
  height: 588px;
  padding: 24px 0px 0px 24px;
  & > .header {
    display: flex;
    height: 64px;
    align-items: center;
    & > div {
      display: flex;
      align-items: center;
      margin-right: 40px;
      span {
        font-size: 22px;
        margin-right: 8px;
      }
      .selsect {
        :deep(.van-cell) {
          width: 220px;
          height: 64px;
          display: flex;
          align-items: center;
          .van-field__control {
            font-size: 22px;
          }
        }
        :deep(.van-popup--bottom.van-popup--round) {
          border-radius: 4px;
        }
        :deep(.van-picker__cancel) {
          font-size: 22px;
          width: 100px;
        }
        :deep(.van-picker__confirm) {
          font-size: 22px;
          width: 100px;
        }
        :deep(.van-picker-column__item) {
          font-size: 22px;
        }
      }
      .num {
        width: 289px;
      }
    }
  }
}
</style>
