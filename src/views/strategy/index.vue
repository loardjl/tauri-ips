<template>
  <div class="strategy">
    <div class="filterItems">
      <span>工件型号</span>
      <span>(共1111件)：</span>
      <div class="selsect">
        <van-field
          v-model="program_num"
          is-link
          readonly
          placeholder="请选择"
          @click="showPicker = true"
        />
        <van-popup v-model:show="showPicker" round position="bottom">
          <van-picker
            option-height="54px"
            :columns="program_num_list"
            @cancel="showPicker = false"
            @confirm="onprogramConfirm"
          />
        </van-popup>
      </div>
    </div>
    <vxe-table
      v-if="tableData.length"
      border
      :data="tableData"
      align="center"
      :row-config="{ isHover: true }"
    >
      <vxe-column field="enable" title="启用状态">
        <template #default="{ row }">
          <van-switch v-model="row.enable" :active-value="1" :inactive-value="0" />
        </template>
      </vxe-column>
      <vxe-column field="tool_number" title="刀具号"></vxe-column>
      <vxe-column
        v-for="item in strategies"
        :key="item.strategy_id"
        :field="item.strategy_name"
        :title="item.strategy_name"
      >
        <template #default="{ row }">
          <div class="radio">
            <van-radio-group v-model="row.cur_strategy_id" shape="dot">
              <van-radio :name="item.strategy_id"></van-radio>
            </van-radio-group>
          </div>
        </template>
      </vxe-column>
    </vxe-table>
    <div class="empty" v-if="!tableData.length">
      <emptyPage text="暂无数据"></emptyPage>
    </div>
  </div>
</template>

<script setup lang="jsx">
import { onMounted, ref, getCurrentInstance, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useRouter } from 'vue-router'
import { useApi } from '@src/hooks/useApi'
import emptyPage from '@components/common/emptyPage/index.vue'
import popover from '@components/common/popover/inedx.vue'
const { fetchPostApi } = useApi()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const router = useRouter()
const strategies = JSON.parse(sessionStorage.getItem('strategies'))
const { proxy } = getCurrentInstance()
onMounted(() => {
  asideList.value = [
    {
      key: 'save',
      text: ' ',
      render: () => {
        return (
          <popover ref={popoverRef} contentText="是否保存修改?">
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
                        save()
                      }}
                    >
                      保存
                    </span>
                  </>
                )
              },
              referBtn: () => {
                return <div onClick={() => saveFun()}>保存</div>
              }
            }}
          </popover>
        )
      }
    },
    {
      key: 'previous',
      text: '上一页',
      disable: computed(() => (startPage.value <= 0 ? true : false)),
      cb: () => {
        prevFun()
      }
    },
    {
      key: 'next',
      text: '下一页',
      disable: computed(() => {
        return pageAll.value <= 0 || startPage.value === pageAll.value ? true : false
      }),
      cb: () => {
        nextFun()
      }
    },
    {
      key: 'policyManagement',
      text: '策略管理',
      auth: 'admin',
      cb: () => {
        router.push({
          path: '/strategy/policyManagement'
        })
      }
    },
    {
      key: 'learningStyle',
      text: '学习方式',
      auth: 'admin',
      cb: () => {
        if (program_num.value === '') {
          proxy.$alertMsg('clear', '', '请选择工件型号', { type: 'danger' })
          return
        }
        router.push({
          path: '/strategy/learningStyle',
          state: {
            program_num: JSON.stringify(program_num.value)
          }
        })
      }
    },
    {
      key: 'artifactManagement',
      text: '工件管理',
      cb: () => {
        router.push({
          path: '/strategy/artifactManagement'
        })
      }
    }
  ]
  workpieceProgram()
})
//获取工件型号
const workpieceProgram = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_all_workpiece_program',
        id: '2',
        params: {}
      },
      'ipsbatch'
    )
    const data = res
    const strategyprogramnum = sessionStorage.getItem('strategy_program_num')
    if (strategyprogramnum) {
      program_num.value = strategyprogramnum
      workpiece_strategy()
    } else {
      sessionStorage.setItem('strategy_program_num', program_num.value)
      program_num.value = data.result.cur_program_num
      workpiece_strategy()
    }
    program_num_list.value = data.result.program_num_list.map(item => {
      return {
        text: item,
        value: item
      }
    })
  } catch (e) {
    console.log(e)
  }
}
//下拉
const showPicker = ref(false)
const program_num = ref('')
const onprogramConfirm = ({ selectedOptions }) => {
  showPicker.value = false
  program_num.value = selectedOptions[0].text
  sessionStorage.setItem('strategy_program_num', program_num.value)
  startPage.value = 0
  total.value = 0
  pageAll.value = 0
  workpiece_strategy()
}
//工件序号列表
const program_num_list = ref([])

//获取工件型号刀具策略
const workpiece_strategy = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_workpiece_strategy',
        id: '6',
        params: {
          program_num: program_num.value,
          start: startPage.value * count.value,
          count: count.value
        }
      },
      'ipsbatch'
    )
    const data = res
    tableData.value = data.result.workpiece_strategies
    total.value = data.result.totoal
    pageAll.value = Math.ceil(data.result.total / count.value) - 1
  } catch (e) {
    console.log(e)
  }
}
const tableData = ref([])
//保存确认
const popoverRef = ref(null)
const save = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'set_workpiece_strategy',
        id: '20',
        params: {
          program_name: program_num.value,
          workpiece_strategies: tableData.value
        }
      },
      'ipsbatch'
    )
    const data = res
    console.log(data)
    popoverRef.value.confirmFun()
    if (!data.result.status) proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
    else proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
    workpiece_strategy()
  } catch (e) {
    console.log(e)
  }
}

const startPage = ref(0) //当前页数
const total = ref(0) //总条数
const count = ref(5) //一页多少条
const pageAll = ref(0)
//上一页
const prevFun = () => {
  --startPage.value
  workpiece_strategy()
}
//下一页
const nextFun = () => {
  ++startPage.value
  workpiece_strategy()
}

const saveFun = () => {
  if (!tableData.value.length) {
    proxy.$alertMsg('clear', '', '请选择工件', { type: 'danger' })
    return
  } else popoverRef.value.handleClick()
}
</script>

<style lang="scss" scoped>
.strategy {
  height: 100%;
  .filterItems {
    display: flex;
    align-items: center;
    background-color: #f1f1f1;
    padding: 8px 0 8px 24px;
    margin-bottom: 16px;
    & > :first-child {
      color: #343434;
      font-size: 22px;
    }
    & > :nth-child(2) {
      color: #34343480;
      font-size: 22px;
      margin-left: 5px;
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
    .timeDate {
      margin-left: 24px;
      :deep(.van-cell) {
        width: 374px;
        height: 64px;
        display: flex;
        align-items: center;
        border: 1px solid rgb(220, 223, 230);
        border-radius: 0.3125vw;
        .van-cell__title {
          font-size: 22px;
        }
        .van-cell__value {
          font-size: 22px;
        }
      }
      :deep(.van-popup--right.van-popup--round) {
        border-radius: 4px;
      }
    }
  }
  .empty {
    height: calc(100% - 128px);
  }
  :deep(.vxe-header--row) {
    background-color: rgb(244, 248, 254);
    height: 54px;
    color: rgb(51, 51, 51);
  }
  :deep(.vxe-body--row) {
    height: 65px;
    color: rgb(102, 102, 102);
    .vxe-body--column {
      height: 65px;
      .vxe-cell {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        .radio {
          display: flex;
          justify-content: center;
          height: 100%;
          width: 100%;
          .van-radio-group {
            width: 100%;
            height: 100%;
            & > div {
              width: 100%;
              height: 100%;
              display: flex;
              justify-content: center;
            }
            // height: 50px;
            // width: 50px;
          }
        }
      }
      // :deep(.vxe-cell) {
      //   height: 60px;

      // }
    }
  }
  :deep(.vxe-table--body-wrapper) {
    min-height: 0px !important;
  }
  :deep(.row--hover) {
    background-color: rgb(241, 241, 241) !important;
  }
  :deep(.row--current) {
    background-color: rgb(241, 241, 241);
  }
}
</style>
