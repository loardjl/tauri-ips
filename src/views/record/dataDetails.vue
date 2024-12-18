<template>
  <div class="dataDetails">
    <div class="header">
      <div>
        <span>工件序号:</span>
        <span>{{ rowValue.workpiece_id }}</span>
      </div>
      <div>
        <span>优化前:</span>
        <span>{{ rowValue.standard_time }}</span>
      </div>
      <div>
        <span>优化后:</span>
        <span>{{ rowValue.actual_time }}</span>
      </div>
      <div>
        <span>节省时间(s)/比率(s):</span>
        <span
          >{{ (rowValue.optimize_time / 1000).toFixed(2) }} /
          {{ rowValue.optimize_ratio.toFixed(2) }}%</span
        >
      </div>
    </div>
    <div class="table">
      <vxe-table
        border
        :data="tableData"
        align="center"
        :row-config="{ isHover: true }"
        max-height="100%"
        v-if="tableData.length"
      >
        <vxe-column field="tool_number" title="刀具号"></vxe-column>
        <vxe-column field="strategy_name" title="优化策略"></vxe-column>
        <vxe-column field="standard_time" title="优化前"></vxe-column>
        <vxe-column field="actual_time" title="优化后">
          <template #default="{ row }">
            <span>{{ row.optimize_enable ? row.actual_time : '-' }}</span>
          </template>
        </vxe-column>
        <vxe-column field="optimize_time" title="节省时间(s)">
          <template #default="{ row }">
            <span>{{ row.optimize_enable ? row.optimize_time : '-' }}</span>
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
import { onMounted, ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useApi } from '@src/hooks/useApi'
import emptyPage from '@components/common/emptyPage/index.vue'
import { _public } from '@src/utils/common'
const { fetchPostApi } = useApi()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const rowValue = JSON.parse(history.state.rowValue)
const strategies = JSON.parse(sessionStorage.getItem('strategies'))
console.log(strategies)
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
        return pageAll.value <= 0 || startPage.value === pageAll.value || pageAll.value === 1
          ? true
          : false
      }),
      cb: () => {
        nextFun()
      }
    }
  ]
  process_history()
})

const process_history = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_workpiece_process_history',
        id: '18',
        params: {
          program_name: history.state.program_name,
          workpiece_id: rowValue.workpiece_id,
          start: startPage.value * count.value,
          count: count.value
        }
      },
      'ipsbatch'
    )
    const data = res
    data.result.workpiece_process_historys.forEach(item => {
      item.actual_time = _public.getTime(item.actual_time, '{h}:{m}:{s}', false)
      item.standard_time = _public.getTime(item.standard_time, '{h}:{m}:{s}', false)
      item.optimize_time = (item.optimize_time / 1000).toFixed(2)
    })
    tableData.value = data.result.workpiece_process_historys
    total.value = data.result.total
    pageAll.value = parseInt(data.result.total / count.value)
  } catch (e) {
    console.log(e)
  }
  tableData.value.forEach(item => {
    strategies.forEach(j => {
      if (j.strategy_id === item.cur_strategy_id) {
        item.strategy_name = j.strategy_name
      }
    })
  })
}

const tableData = ref([])

const startPage = ref(0) //当前页数
const total = ref(0) //总条数
const count = ref(7) //一页多少条
const pageAll = ref(0)
//上一页
const prevFun = () => {
  --startPage.value
  process_history()
}
//下一页
const nextFun = () => {
  ++startPage.value
  process_history()
}
</script>

<style lang="scss" scoped>
.dataDetails {
  height: 100%;
  background-color: rgb(241, 241, 241);
  padding: 24px;
  .empty {
    height: calc(100% - 96px);
  }
  .header {
    display: flex;
    margin-bottom: 24px;
    & > div {
      font-size: 22px;
      color: rgba(52, 52, 52, 0.5);
      margin-right: 18px;
      & > :last-child {
        margin-left: 10px;
      }
    }
  }
  .table {
    height: calc(100% - 50px);
  }
  .optimizeratio {
    display: flex;
    align-items: center;
    justify-content: center;
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
    background-color: rgb(241, 241, 241) !important;
  }
  :deep(.row--current) {
    background-color: rgb(241, 241, 241);
  }
}
</style>
