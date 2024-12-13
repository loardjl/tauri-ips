<template>
  <div class="artifactManagement">
    <vxe-table
      v-if="tableData.length"
      border
      :data="tableData"
      align="center"
      :row-config="{ isHover: true }"
      ref="tableRef"
      max-height="100%"
    >
      <vxe-column type="checkbox" width="72"></vxe-column>
      <vxe-column field="program_name" title="程序号"></vxe-column>
      <vxe-column field="total_processing_time" title="累计加工时间"> </vxe-column>
      <vxe-column field="total_optimize_time" title="累计优化时间"> </vxe-column>
    </vxe-table>
    <div class="empty" v-if="!tableData.length">
      <emptyPage text="暂无数据"></emptyPage>
    </div>
  </div>
</template>

<script setup lang="jsx">
import { onMounted, ref, getCurrentInstance } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useApi } from '@src/hooks/useApi'
import emptyPage from '@components/common/emptyPage/index.vue'
import popover from '@components/common/popover/inedx.vue'
import { _public } from '@src/utils/common'
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
      key: 'accrued',
      text: '重新累计',
      render: () => {
        return (
          <popover ref={popoverRef} contentText="是否重新累计?">
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
                        accruedFun()
                      }}
                    >
                      确认
                    </span>
                  </>
                )
              },
              referBtn: () => {
                return <div onClick={() => accruedSelect()}>重新累计</div>
              }
            }}
          </popover>
        )
      }
    },
    {
      key: 'delete',
      text: '删除工件',
      render: () => {
        return (
          <popover ref={popoverDelRef} contentText="是否删除选中工件?">
            {{
              operate: () => {
                return (
                  <>
                    <span
                      onClick={() => {
                        popoverDelRef.value.cancelFun()
                      }}
                    >
                      取消
                    </span>
                    <span
                      onClick={() => {
                        deleteFun()
                      }}
                    >
                      确认
                    </span>
                  </>
                )
              },
              referBtn: () => {
                return <div onClick={() => deleteSelect()}>删除工件</div>
              }
            }}
          </popover>
        )
      }
    }
  ]
  getworkpiecemanager()
})
const tableData = ref([])

const getworkpiecemanager = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_workpiece_manager_history',
        id: '19',
        params: {}
      },
      'ipsbatch'
    )
    const data = res
    data.result.workpiece_histories.forEach(item => {
      item.total_optimize_time = _public.getTime(item.total_optimize_time, '{h}:{m}:{s}')
      item.total_processing_time = _public.getTime(item.total_processing_time, '{h}:{m}:{s}', false)
    })
    tableData.value = data.result.workpiece_histories
  } catch (e) {
    console.log(e)
  }
}

const tableRef = ref()

//重新累计
const popoverRef = ref(null)
const accruedSelect = () => {
  toolFun('请选择工件', popoverRef.value)
}
const accruedFun = async () => {
  const $table = tableRef.value
  if ($table) {
    const records = $table.getCheckboxRecords()
    try {
      const res = await fetchPostApi(
        {
          version: '1.0',
          method: 're_accumulate',
          id: '12',
          params: {
            program_num: records.map(item => item.program_name)
          }
        },
        'ipsbatch'
      )
      const data = res
      popoverRef.value.confirmFun()
      getworkpiecemanager()
      if (!data.result.status) proxy.$alertMsg('checked', '', '重新累计成功', { type: 'success' })
      else proxy.$alertMsg('clear', '', '重新累计失败', { type: 'danger' })
    } catch (e) {
      console.log(e)
    }
  }
}
const toolFun = (text, ref) => {
  const $table = tableRef.value
  const records = $table.getCheckboxRecords()
  if (records.length) {
    ref.handleClick()
  } else {
    proxy.$alertMsg('clear', '', text, { type: 'danger' })
  }
}
//删除
const popoverDelRef = ref(null)
const deleteSelect = () => {
  toolFun('请选择工件', popoverDelRef.value)
}
const deleteFun = async () => {
  const $table = tableRef.value
  if ($table) {
    const records = $table.getCheckboxRecords()
    try {
      const res = await fetchPostApi(
        {
          version: '1.0',
          method: 'delete_workpiece',
          id: '13',
          params: {
            program_num: records.map(item => item.program_name)
          }
        },
        'ipsbatch'
      )
      const data = res
      popoverDelRef.value.confirmFun()
      getworkpiecemanager()
      if (!data.result.status) proxy.$alertMsg('checked', '', '删除成功', { type: 'success' })
      else proxy.$alertMsg('clear', '', '删除失败', { type: 'danger' })
    } catch (e) {
      console.log(e)
    }
  }
}
</script>

<style lang="scss" scoped>
.artifactManagement {
  height: 100%;
  padding: 24px;
  background-color: rgb(241, 241, 241);
  .empty {
    height: 100%;
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
