<template>
  <div class="record">
    <div class="filterItems">
      <span>工件型号</span>
      <span>(共{{ dataList.length }}件)：</span>
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
      <div class="timeDate">
        <van-cell title="选择日期区间" :value="date" @click="show = true" />
        <van-calendar v-model:show="show" type="range" @confirm="onConfirm" position="right" />
      </div>
    </div>
    <div class="content" v-if="dataList.length">
      <div class="contnet-item" v-for="item in dataList" :key="item">
        <div class="content-header">
          <div>
            <img src="@src/assets/icons/svg/path.svg" alt="" />
            <span>{{ item.tool_number }}</span>
          </div>
          <div>
            <!-- <span>刀具周期</span>
            <span>平均寿命</span> -->
          </div>
        </div>
        <div class="content-btn">
          <div v-for="j in item.strategy_historys" :key="j">
            <div class="con-btn-lef">
              <span :class="[item.cur_strategy_id === j.strategy_id ? 'active' : '']">{{
                j.strategy_name
              }}</span>
            </div>
            <div class="con-btn-rig">
              <div :style="j.styleName">
                <span class="percentage">{{
                  j.enable ? j.optimize_ratio.toFixed(2) + '%' : '--'
                }}</span>
                <!-- <span class="tool">{{ j.optimize_ratio }}</span>
                <span class="num">{{ j.optimize_ratio }}</span> -->
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="empty" v-if="!dataList.length">
      <emptyPage text="暂无数据"></emptyPage>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, getCurrentInstance, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useRouter } from 'vue-router'
import { useApi } from '@src/hooks/useApi'
import dayjs from 'dayjs'
import emptyPage from '@components/common/emptyPage/index.vue'
const { fetchPostApi } = useApi()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const router = useRouter()
const { proxy } = getCurrentInstance()
onMounted(() => {
  asideList.value = [
    {
      key: 'relearn',
      text: '加工记录',
      cb: () => {
        if (program_num.value === '') {
          proxy.$alertMsg('clear', '', '请选择工件型号', { type: 'danger' })
          return
        }
        router.push({
          path: '/record/machiningRecords',
          state: {
            parameter: {
              program_name: program_num.value,
              start_time: startTime.value,
              end_time: endTime.value
            }
          }
        })
        sessionStorage.removeItem('machiningPage')
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
  workpieceProgram()
  const dateTime = JSON.parse(sessionStorage.getItem('record_program_dateTiem'))
  if (dateTime) {
    // date.value = dateTime.date
    // endTime.value = dateTime.endTime
    // startTime.value = dateTime.startTime
  }
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
    const recordprogramnum = sessionStorage.getItem('record_program_num')
    if (recordprogramnum) {
      program_num.value = recordprogramnum
      toolhistoryFun()
    } else {
      program_num.value = data.result.cur_program_num
      sessionStorage.setItem('record_program_num', program_num.value)
      toolhistoryFun()
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

//获取所有策略配置
const strategies = JSON.parse(sessionStorage.getItem('strategies'))

//下拉
const showPicker = ref(false)
const program_num = ref('')
const onprogramConfirm = ({ selectedOptions }) => {
  showPicker.value = false
  program_num.value = selectedOptions[0].text
  sessionStorage.setItem('record_program_num', program_num.value)
  startPage.value = 0
  total.value = 0
  pageAll.value = 0
  toolhistoryFun()
}
//工件序号列表
const program_num_list = ref([])

//日期
const date = ref('')
const show = ref(false)
const startTime = ref()
const endTime = ref()
const formatDate = date => `${date.getMonth() + 1}/${date.getDate()}`
const onConfirm = values => {
  const [start, end] = values
  show.value = false
  startTime.value = dayjs(start).valueOf()
  endTime.value = dayjs(end).valueOf()
  date.value = `${formatDate(start)} - ${formatDate(end)}`
  const obj = {
    startTime: startTime.value,
    endTime: endTime.value,
    date: date.value
  }
  sessionStorage.setItem('record_program_dateTiem', JSON.stringify(obj))
  toolhistoryFun()
}
const startPage = ref(0) //当前页数
const total = ref(0) //总条数
const cont = ref(4) //一页多少条
const pageAll = ref(0)
//获取历史数据
const toolhistoryFun = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_workpiece_tool_history',
        id: '16',
        params: {
          program_name: program_num.value,
          start_time: startTime.value ? startTime.value : -1,
          end_time: endTime.value ? endTime.value : -1,
          start: startPage.value * cont.value,
          count: cont.value
        }
      },
      'ipsbatch'
    )
    const data = res
    strategies.forEach(item => {
      data.result.workpiece_tool_historys.forEach(j => {
        j.strategy_historys.forEach(i => {
          if (item.strategy_id === i.strategy_id) {
            i.strategy_name = item.strategy_name
          }
        })
        findMaxes(j.strategy_historys)
      })
    })
    dataList.value = data.result.workpiece_tool_historys
    total.value = data.result.total
    pageAll.value = parseInt(data.result.total / cont.value)
  } catch (e) {
    console.log(e)
  }
}

const findMaxes = arr => {
  const sortedArr = [...arr].sort((a, b) => b.optimize_ratio - a.optimize_ratio)
  sortedArr.forEach((item, index) => {
    if (item.enable) {
      item.styleName = `background: linear-gradient(to right, rgb(80, 104, 190) 0%, rgba(45, 91, 255, 0) ${
        100 - index * 20
      }%);`
    } else {
      item.styleName = `background: linear-gradient(to right, rgb(187, 188, 185) 0%, rgba(187, 188, 185, 0.1) 100%);`
    }

    // if (index === 0) item.className = 'purpleramp100'
    // if (index === 1) item.className = 'purpleramp60'
    // if (index === 2) item.className = 'purpleramp30'
    // if (index === 3) item.className = 'purpleramp10'
  })
  arr.forEach(item => {
    sortedArr.forEach(j => {
      if (item.strategy_id === j.strategy_id) {
        item = j
      }
    })
  })
}
//上一页
const prevFun = () => {
  --startPage.value
  toolhistoryFun()
}
//下一页
const nextFun = () => {
  ++startPage.value
  toolhistoryFun()
}

const dataList = ref([])
</script>

<style lang="scss" scoped>
.record {
  height: 100%;
  .empty {
    height: calc(100% - 96px);
  }
  .filterItems {
    display: flex;
    align-items: center;
    background-color: #f1f1f1;
    padding: 8px 0 8px 24px;
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
      }
      :deep(.van-picker__confirm) {
        font-size: 22px;
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
  .content {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    .contnet-item {
      margin-top: 16px;
      width: 492px;
      height: 242px;
      background-color: #f1f1f1;
      box-shadow: 0px 2px 20px 0px rgba(0, 0, 0, 0.1);
      padding: 10px 16px 0px 16px;
      .content-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        & > :first-child {
          font-size: 30px;
          font-weight: 600;
          font-family: 阿里巴巴普惠体 2;
          span {
            margin-left: 10px;
          }
        }
        & > :last-child {
          font-size: 18px;
          color: rgba(0, 0, 0, 0.5);
          span {
            margin-left: 16px;
          }
        }
      }
      .content-btn {
        margin-top: 4px;
        & > div {
          display: grid;
          grid-template-columns: 95px 357px;
        }
        .con-btn-lef {
          font-size: 20px;
          color: rgba(0, 0, 0, 0.5);
          span {
            // padding: 2px 15px;
            text-align: center;
            margin-top: 12px;
            display: block;
            background-color: white;
          }
          .active {
            background-color: rgb(80, 104, 190);
            color: white;
          }
        }
        .con-btn-rig {
          font-size: 20px;
          margin-left: 8px;
          & > div {
            margin-top: 12px;
            height: 32px;
            display: flex;
            align-items: center;
            .percentage {
              margin-left: 8px;
              font-size: 24px;
              font-weight: 700;
              color: rgb(255, 255, 255);
              width: 150px;
            }
            .tool {
              width: 80px;
              margin-left: 28px;
              text-align: right;
            }
            .num {
              width: 80px;
              text-align: right;
            }
          }
          .purpleramp100 {
            background: linear-gradient(to right, rgb(80, 104, 190) 0%, rgba(45, 91, 255, 0) 100%);
          }
          .purpleramp60 {
            background: linear-gradient(to right, rgb(80, 104, 190) 0%, rgba(45, 91, 255, 0) 70%);
          }
          .purpleramp30 {
            background: linear-gradient(to right, rgb(80, 104, 190) 0%, rgba(45, 91, 255, 0) 50%);
          }
          .purpleramp10 {
            background: linear-gradient(to right, rgb(80, 104, 190) 0%, rgba(45, 91, 255, 0) 30%);
          }
          .greyramp {
            background: linear-gradient(
              to right,
              rgb(187, 188, 185) 0%,
              rgba(187, 188, 185, 0.1) 100%
            );
          }
        }
      }
    }
  }
}
</style>
