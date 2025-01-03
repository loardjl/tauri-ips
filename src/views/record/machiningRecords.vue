<template>
  <div class="machiningRecords">
    <div class="table">
      <vxe-table
        border
        :data="tableData"
        align="center"
        :row-config="{ isHover: true, isCurrent: true }"
        @cell-click="cellClickEvent"
        max-height="100%"
        v-if="tableData.length"
      >
        <vxe-column field="workpiece_id" title="工件序号"></vxe-column>
        <vxe-column field="process_start_time" title="加工开始时间" width="260">
          <template #default="{ row }">
            <span>{{ dayjs(row.process_start_time).format('YYYY-MM-DD HH:mm:ss') }}</span>
          </template>
        </vxe-column>
        <vxe-column field="standard_time" title="优化前"></vxe-column>
        <vxe-column field="actual_time" title="优化后">
          <template #default="{ row }">
            <span>{{ row.optimize_enable ? row.actual_time : '-' }}</span>
          </template>
        </vxe-column>
        <vxe-column field="optimize_ratio" title="节省时间(s)/比率" width="260">
          <template #default="{ row }">
            <div class="optimizeratio" v-if="row.optimize_enable">
              <span :class="[row.optimize_ratio > 100 ? 'green' : '']">
                {{ (row.optimize_time / 1000).toFixed(2) }}</span
              >/<span>{{ row.optimize_ratio.toFixed(2) }}%</span>
              <img v-show="row.optimize_ratio > 100" src="@src/assets/icons/svg/arrow.svg" alt="" />
            </div>
            <span v-else>-</span>
          </template>
        </vxe-column>
      </vxe-table>
    </div>
    <div class="empty" v-if="!tableData.length">
      <emptyPage text="暂无数据"></emptyPage>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, getCurrentInstance, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useApi } from '@src/hooks/useApi'
import { useRouter } from 'vue-router'
import emptyPage from '@components/common/emptyPage/index.vue'
import { _public } from '@src/utils/common'
import dayjs from 'dayjs'
const router = useRouter()
const menuStore = useMenuStore()
const { fetchPostApi } = useApi()
const { asideList } = storeToRefs(menuStore)
const parameter = history.state.parameter
console.log(parameter)
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
      key: 'datadetail',
      text: '数据详情',
      cb: () => {
        if (!Object.keys(rowValue.value).length) {
          proxy.$alertMsg('clear', '', '请选择加工记录', { type: 'danger' })
          return
        }
        router.push({
          path: '/record/machiningRecords/dataDetails',
          state: {
            rowValue: JSON.stringify(rowValue.value),
            program_name: parameter.program_name,
            process_start_time: rowValue.value.total_process_start_time
          }
        })
        sessionStorage.setItem('machiningPage', JSON.stringify(startPage.value))
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
    }
  ]
  startPage.value = JSON.parse(sessionStorage.getItem('machiningPage'))
  workpiece_history()
})
const workpiece_history = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_workpiece_record_history',
        id: '17',
        params: {
          program_name: parameter.program_name,
          start_time: parameter.start_time || -1,
          end_time: parameter.end_time || -1,
          start: startPage.value * count.value,
          count: count.value
        }
      },
      'ipsbatch'
    )
    const data = res
    data.result.process_historys.forEach(item => {
      item.actual_time = _public.getTime(item.actual_time, '{h}:{m}:{s}', false)
      item.standard_time = _public.getTime(item.standard_time, '{h}:{m}:{s}', false)
    })
    tableData.value = data.result.process_historys
    total.value = data.result.total
    pageAll.value = Math.ceil(data.result.total / count.value) - 1
  } catch (e) {
    console.log(e)
  }
}

const tableData = ref([])
//选中某一行参数
const rowValue = ref({})
const cellClickEvent = ({ row }) => {
  rowValue.value = row
}

const startPage = ref(0) //当前页数
const total = ref(0) //总条数
const count = ref(8) //一页多少条
const pageAll = ref(0)
//上一页
const prevFun = () => {
  --startPage.value
  workpiece_history()
}
//下一页
const nextFun = () => {
  ++startPage.value
  workpiece_history()
}
</script>

<style lang="scss" scoped>
.machiningRecords {
  height: 100%;
  background-color: rgb(241, 241, 241);
  padding: 24px;
  .empty {
    height: 100%;
  }
  .table {
    height: 100%;
  }
  .optimizeratio {
    display: flex;
    align-items: center;
    justify-content: center;
    .green {
      color: rgb(43, 193, 85);
    }
  }
  :deep(.vxe-header--row) {
    background-color: rgb(244, 248, 254);
    height: 54px;
    color: rgb(51, 51, 51);
  }
  :deep(.vxe-body--row) {
    height: 65px;
    color: rgb(102, 102, 102);
  }
  :deep(.vxe-table--body-wrapper) {
    min-height: 0px !important;
  }
  :deep(.row--hover) {
    background-color: transparent !important;
  }
  :deep(.row--current) {
    background-color: rgb(241, 241, 241) !important;
  }
}
</style>
