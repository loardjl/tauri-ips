<template>
  <div class="collector-wrap" v-if="tableData.length" @click="removeSelect">
    <div class="collector-list">
      <div
        class="collector-item"
        v-for="item in tableData"
        :key="item.id"
        :class="selectedId === item.id ? 'active' : ''"
        @click.stop="selectedCollector(item)"
      >
        <div class="flex-between">
          <div class="left flex-start">
            <img src="@src/assets/icons/svg/connect2.svg" alt="" v-if="item.connect_status" />
            <img src="@src/assets/icons/svg/connect1.svg" alt="" v-else />
            <div class="ml9 line1 name">{{ item.name }}</div>
          </div>
          <div class="right flex-center">{{ filterCollectorType(item.collector_type_id) }}</div>
        </div>
        <div class="ip mt16">{{ item.ip }}</div>
      </div>
    </div>
  </div>

  <emptyPage text="暂无数据" v-else>
    <VanButton type="primary" @click="router.push('/collector/add')">去添加</VanButton>
  </emptyPage>
</template>

<script setup lang="jsx">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useSignalController } from '@src/hooks/useSignalController'
import popover from '@components/common/popover/inedx.vue'
import emptyPage from '@components/common/emptyPage/index.vue'
import { useApi } from '@src/hooks/useApi'
const { fetchPostApi } = useApi()
import { useRouter } from 'vue-router'
const router = useRouter()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
import { useSysStore } from '@src/store/useSys'
const sysStore = useSysStore()
const { devId, adapterList } = storeToRefs(sysStore)
const { worker } = useSignalController()
console.log(worker)
const selectedId = ref(0)
const selectedItem = ref(null)
const removeSelect = () => {
  selectedItem.value = null
  selectedId.value = 0
}
const prevFun = () => {
  console.log('prevFun')
}
const nextFun = () => {
  console.log('nextFun')
}
const selectedCollector = item => {
  selectedId.value = item.id
  selectedItem.value = item
}
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
  getMachineList()
}

const filterCollectorType = id => {
  const item = collectorList.value.find(item => item.type_id === id)
  return item?.display_name || ''
}

// 获取机床列表
// const selectedMachine = ref()
const machineTabList = ref([])
const getMachineList = async () => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_dev_list',
    id: '9',
    params: {}
  })
  machineTabList.value = res.result.dev_list.filter(d => d.dev_type === 'ARM')
  if (machineTabList.value.length) {
    devId.value = machineTabList.value[0].dev_id
    await getList()
  } else {
    // 没有机床默认添加一个，作为采集配置的模版机床用于后续采集器的添加
    const params = {
      mch_id: 'ARM',
      dev_name: 'ARM',
      dev_brand: '',
      dev_type: 'ARM',
      dev_extra_param: '1',
      channel_info_list: [
        {
          nc_channel_info: {
            channel_num: 1
          },
          // NC轴信息
          nc_axis_list: [
            {
              axis_type: 0,
              axis_num: 1,
              name: '主轴'
            }
          ]
        }
      ]
    }
    const res = await fetchPostApi({
      version: '1.0',
      method: 'add_dev',
      id: '10',
      params
    })
    if (res.result) {
      devId.value = res.result.dev_id
    } else {
      // ElMessage.error(t('collector.addMachineFail'))
    }
  }
}
// 获取表格数据
const tableData = ref([]) // 筛选后页面需要展示
const allTableData = ref([]) // 实际上列表返回的数据
let tempTableData = []
let tableIndex = 0
let firstNcData = null
const getList = async () => {
  await unsubscribe()
  firstNcData = null
  const res = await fetchPostApi({
    version: '1.0',
    method: 'enum_adapters',
    id: '8',
    params: {
      dev_id: devId.value
    }
  })
  allTableData.value = []
  tableData.value = []
  tableIndex = 0
  tempTableData = res.result.adapter_list
  tempTableData.length && (await getAdapterStatus(res.result.adapter_list[0]))
  adapterList.value = tableData.value
}

// 获取采集器状态 -- 递归方法
const getAdapterStatus = async item => {
  const res = await fetchPostApi({
    version: '1.0',
    method: 'get_adapter_status',
    id: '46',
    params: {
      dev_id: devId.value,
      adapter_id: item.id
    }
  })

  const tempData = {
    connect_status: res.result.connect_status,
    connect_status_name: res.result.connect_status ? '已连接' : '未连接',
    collector_type_name: collectorList.value.filter(
      val => val.type_id === item.collector_type_id
    )[0].name
  }

  if (firstNcData && item.collector_type_id === 1) {
    tableData.value.find(d => d.id === firstNcData.id).ids.push(item.id)
  }

  // 过滤多通道数据
  // 需要在页面展示的数据
  // 代表是 nc, 筛选只展示第一条NC采集器的数据
  if ((!firstNcData && item.collector_type_id === 1) || item.collector_type_id !== 1) {
    if (item.collector_type_id === 1) {
      firstNcData = item
    }
    tableData.value.push({
      ...item,
      // path_num: item.collector_type_id === 1 ? pathNumList.value.length : 0,
      // TODO
      path_num: 0,
      ...tempData,
      ids: [item.id]
    })
  }

  // 接口返回的总的数据
  allTableData.value.push({
    ...item,
    ...tempData
  })

  if (tableIndex !== tempTableData.length - 1) {
    tableIndex++
    await getAdapterStatus(tempTableData[tableIndex])
  }
}

const popoverRef = ref()
// 连接/断连 改变连接状态
const handleConnect = async () => {
  console.log(999, selectedItem.value)
  const { id, connect_status, ids } = selectedItem.value
  let tempMethod = 'adapter_connect'
  let tempId = '52'
  if (connect_status) {
    // 需要断连
    tempMethod = 'adapter_disconnect'
    tempId = '53'
  }
  for (const i of ids) {
    await fetchPostApi({
      version: '1.0',
      method: tempMethod,
      id: tempId,
      params: {
        dev_id: devId.value,
        adapter_id: i
      }
    })
  }
  popoverRef.value.cancelFun()
  setTimeout(() => {
    getList()
  }, 200)

  // 改变列表连接状态
  // for (const val of tableData.value) {
  //   if (val.id === id) {
  //     val.connect_status = !connect_status
  //     val.connect_status_name = !connect_status ? '已连接' : '未连接'
  //     break
  //   }
  // }
}
const popDeleteRef = ref()
// 删除
const handleDelete = async () => {
  const row = selectedItem.value
  if (row.collector_type_id === 1) {
    // 代表是 NC，需要把所有的NC采集器一并删除
    for (const val of allTableData.value) {
      if (val.collector_type_id === 1) {
        await fetchPostApi({
          version: '1.0',
          method: 'delete_adapter',
          id: '16',
          params: {
            dev_id: devId.value,
            adapter_id: val.id
          }
        })
        // getMachineChannelList()
      }
    }
  } else {
    await fetchPostApi({
      version: '1.0',
      method: 'delete_adapter',
      id: '16',
      params: {
        dev_id: devId.value,
        adapter_id: row.id
      }
    })
  }
  popDeleteRef.value.cancelFun()
  removeSelect()
  getList()
  // type为0表示是ips的信号；type为1表示是过载的信号
  // await delSignalNode({ type: 0, deleteData: [{ adapter_id: row.id }] }).then(res => {
  //   console.log(res)
  // })
  // await delSignalNode({ type: 1, deleteData: [{ adapter_id: row.id }] }).then(res => {
  //   console.log(res)
  // })
}
onMounted(() => {
  getCollectorList()
  asideList.value = [
    {
      key: 'add',
      text: '新增',
      disable: computed(() => selectedId.value > 0),
      cb: () => {
        router.push('/collector/add')
      },
      auth: '*' // 权限, *或者不提供该字段 为所有权限
    },
    {
      key: 'view',
      text: '查看',
      disable: computed(() => selectedId.value <= 0),
      cb: () => {
        // router.push('/collector/view')
        router.push({
          path: '/collector/view',
          query: {
            adapter_id: tempTableData.filter(d => d.collector_type_id === 1).map(val => val.id)
          }
        })
      },
      auth: '*' // 权限, *或者不提供该字段 为所有权限
    },
    {
      key: 'disconnect',
      text: '断连',
      disable: computed(() => selectedId.value <= 0),
      cb: () => {},
      render: () => {
        return (
          <popover
            ref={popoverRef}
            contentText={`是否${selectedItem.value?.connect_status ? '断连' : '连接'}?`}
            trigger={selectedItem.value?.connect_status ? '断连' : '连接'}
          >
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
                          handleConnect()
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
    },
    {
      key: 'delete',
      text: '删除',
      disable: computed(() => selectedId.value <= 0),
      cb: () => {},
      render: () => {
        return (
          <popover ref={popDeleteRef} contentText="是否删除?" trigger="删除">
            {{
              operate: () => {
                return (
                  <>
                    <span
                      onClick={() => {
                        popDeleteRef.value.cancelFun()
                      }}
                    >
                      取消
                    </span>
                    <span
                      onClick={() => {
                        {
                          handleDelete()
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
    },
    {
      key: 'previous',
      text: '上一页',
      disable: true,
      cb: () => {
        prevFun()
      },
      auth: '*' // 权限, *或者不提供该字段 为所有权限
    },
    {
      key: 'next',
      text: '下一页',
      disable: true,
      cb: () => {
        nextFun()
      },
      auth: '*' // 权限, *或者不提供该字段 为所有权限
    }
  ]
})
// 取消订阅
const unsubscribe = async () => {
  // for (const val of spreadExpendList) {
  //   // 取消订阅
  //   await fetchPostApi({
  //     version: '1.0',
  //     method: 'unsubscribe_single_signal',
  //     id: '33',
  //     params: {
  //       dev_id: devId.value,
  //       adapter_id: val,
  //       token: sessionStorage.getItem('token')
  //     }
  //   })
  //   // 退订实时数据
  //   worker.send('stopPushChartData', {})
  // }
  // spreadExpendList = []
}

onBeforeUnmount(async () => {
  unsubscribe()
})
</script>

<style lang="scss" scoped>
.collector-wrap {
  width: 100%;
  height: 100%;
  padding: 24px;
  background: #f1f1f1;
}
.collector-list {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px 16px;
  .collector-item {
    height: auto;
    padding: 16px;
    border-radius: 4px;
    background: #e4e6e7;
    cursor: pointer;
    .left {
      font-size: 24px;
      font-weight: 600;
      color: rgba(52, 52, 52, 0.3);
    }
    .name {
      width: 180px;
    }
    .right {
      width: 64px;
      height: 64px;
      font-size: 28px;
      border-radius: 4px;
      font-weight: 600;
      color: #fff;
      background: #668ffe;
    }
    .ip {
      font-size: 22px;
      color: #c3c3c3;
    }
    &.active {
      background: #fff;
      .left {
        color: #343434;
      }
    }
  }
}
</style>
