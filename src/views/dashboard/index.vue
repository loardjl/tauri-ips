<template>
  <div class="dsshboard">
    <div>
      <span>提效进程</span>
      <div class="background">
        <div class="all-time">
          <div class="text-time">
            <span class="text-left">总提效时间</span>
            <span
              class="text-optimize"
              :class="[
                realtimeInfo.strategy_status === 0
                  ? 'greens'
                  : realtimeInfo.strategy_status === 2
                  ? 'grey'
                  : 'yellow'
              ]"
              >{{
                realtimeInfo.strategy_status === 0
                  ? '效率优先'
                  : realtimeInfo.strategy_status === 2
                  ? '优化关闭'
                  : '过程等待'
              }}</span
            >
          </div>
          <div class="text-num">
            <div>
              <span class="text-timnum">{{
                parseInt(optimizeInfo.total_optimize_time / 1000 / 60 / 60 / 24)
              }}</span>
              <span class="text-timlogotype">d</span>
            </div>
            <div>
              <span class="text-timnum">{{
                parseInt((optimizeInfo.total_optimize_time / 1000 / 60 / 60) % 24)
              }}</span>
              <span class="text-timlogotype">h</span>
            </div>
            <div>
              <span class="text-timnum">{{
                parseInt((optimizeInfo.total_optimize_time / 1000 / 60) % 60)
              }}</span>
              <span class="text-timlogotype">min</span>
            </div>
          </div>
        </div>
        <div class="text-flex">
          <div class="text-alltime">
            <span class="text-grey">总加工时间</span>
            <span class="text-number">{{
              _public.getTime(optimizeInfo.total_processing_time, '{d} d {h} h {m} m')
            }}</span>
          </div>
          <div class="text-allnum">
            <span class="text-grey">总提效件数</span>
            <span class="text-number">{{ optimizeInfo.total_processing_count }}</span>
          </div>
        </div>
        <div class="text-btn">
          <div>
            <span>今日提效</span>
            <span>{{ _public.getTime(optimizeInfo.dayly_optimize_time, '{h} h {m} m') }}</span>
          </div>
          <div>
            <span>最近一周提效</span>
            <span>{{
              _public.getTime(optimizeInfo.weekly_optimize_time, '{d} d {h} h {m} m')
            }}</span>
          </div>
          <div>
            <span>最近一月提效</span>
            <span>{{
              _public.getTime(optimizeInfo.monthly_optimize_time, '{d} d {h} h {m} m')
            }}</span>
          </div>
        </div>
      </div>
    </div>
    <div>
      <span>提效动态</span>
      <div class="chart-tab">
        <div class="position-chart">
          <div class="chart" id="chart"></div>
          <div>0%</div>
          <div>200%</div>
        </div>
        <div class="magnification">
          <div>
            <span
              :style="realtimeInfo.strategy_status === 0 ? 'color: #42b4d2;' : ' color: #4e5969;'"
              >进给控制倍率</span
            >
            / <span>进给旋钮倍率</span>
          </div>
          <div>
            <span
              :style="realtimeInfo.strategy_status === 0 ? 'color: #42b4d2;' : ' color: #4e5969;'"
              >{{
                realtimeInfo.strategy_status === 0 ? realtimeInfo.strategy_feedback + '%' : '--'
              }}</span
            >&ensp;/
            <span>{{ realtimeInfo.nc_knob_feedback }}%</span>
          </div>
        </div>
      </div>
      <span>加工信息</span>
      <div class="processing">
        <div>
          <span>S</span>
          <span>{{ realtimeInfo.actual_rpm }}</span>
        </div>
        <div>
          <span>F</span>
          <span>{{ realtimeInfo.actual_feedback }}</span>
        </div>
      </div>
      <div class="tool-num">
        <span>刀具号</span>
        <span>{{ realtimeInfo.tool_number }}</span>
      </div>
      <div class="tool-num">
        <span>程序号</span>
        <span>{{ realtimeInfo.program_number }}</span>
      </div>
      <div class="tool-num">
        <span>工件序号</span>
        <span>{{ realtimeInfo.workpiece_id }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="jsx">
import { onMounted, inject, ref, computed, getCurrentInstance } from 'vue'
import { storeToRefs } from 'pinia'
import { useMenuStore } from '@src/store/useMenu'
import { useApi } from '@src/hooks/useApi'
import { useSignalController } from '@src/hooks/useSignalController'
import { _public } from '@src/utils/common'
import popover from '@components/common/popover/inedx.vue'
const { proxy } = getCurrentInstance()
const echarts = inject('echarts')
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const { fetchPostApi } = useApi()
const { worker } = useSignalController()
onMounted(() => {
  asideList.value = [
    {
      key: 'relearn',
      sort: 0,
      text: ' ',
      disable: computed(() => (realtimeInfo.value.program_number === '' ? true : false)),
      // cb: () => {
      //   relearnFun()
      // },
      auth: '*', // 权限, *或者不提供该字段 为所有权限
      render: () => {
        return (
          <popover ref={popoverRef} contentText="是否重新学习?" trigger="重新学习">
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
                        relearnFun()
                      }}
                    >
                      确定
                    </span>
                  </>
                )
              }
            }}
          </popover>
        )
      }
    },
    {
      key: 'strategy',
      // text: strategEnable.value ? '关闭提效' : '开启提效',
      text: computed(() => (strategEnable.value ? '关闭提效' : '开启提效')),
      cb: () => {
        strategFun()
      }
    }
  ]
  init()
  worker.dispatch('IpsRegister', () => {
    getstrategFun()
  })
  worker.send('IpsRegister', {})
  worker.dispatch('RealTimeData', ({ payload }) => {
    payload.Ok.strategy_feedback = +(payload.Ok.strategy_feedback * 100).toFixed(0)
    payload.Ok.nc_knob_feedback = +(payload.Ok.nc_knob_feedback * 100).toFixed(0)
    if (payload?.Ok?.strategy_status !== 0) {
      payload.Ok.strategy_feedback = 0
    }
    realtimeInfo.value = payload.Ok
    init()
    // console.log('RealTimeData', payload)
  })
  worker.dispatch('OptimizeInfo', ({ payload }) => {
    console.log('OptimizeInfo', payload)
    payload.Ok.total_optimize_time =
      payload.Ok.total_optimize_time < 0 ? 0 : payload.Ok.total_optimize_time
    payload.Ok.total_processing_time =
      payload.Ok.total_processing_time < 0 ? 0 : payload.Ok.total_processing_time
    payload.Ok.dayly_optimize_time =
      payload.Ok.dayly_optimize_time < 0 ? 0 : payload.Ok.dayly_optimize_time
    payload.Ok.weekly_optimize_time =
      payload.Ok.weekly_optimize_time < 0 ? 0 : payload.Ok.weekly_optimize_time
    payload.Ok.monthly_optimize_time =
      payload.Ok.monthly_optimize_time < 0 ? 0 : payload.Ok.monthly_optimize_time
    optimizeInfo.value = payload.Ok
  })
})
//提效数据
const optimizeInfo = ref({
  total_optimize_time: '',
  total_processing_time: '',
  total_processing_count: '0',
  dayly_optimize_time: 0,
  weekly_optimize_time: '',
  monthly_optimize_time: ''
})
//加工信息
const realtimeInfo = ref({
  strategy_status: -1,
  workpiece_id: '',
  program_number: '',
  tool_number: '',
  actual_rpm: '',
  actual_feedback: '',
  strategy_feedback: 0,
  nc_knob_feedback: 0
})

const popoverRef = ref(null)
const relearnFun = async () => {
  if (realtimeInfo.value.program_number === '') return
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 're_study',
        id: '14',
        params: {
          program_num: realtimeInfo.value.program_number
        }
      },
      'ipsbatch'
    )
    const data = res
    popoverRef.value.confirmFun()
    if (!data.result.status) proxy.$alertMsg('checked', '', '重新学习成功', { type: 'success' })
    else proxy.$alertMsg('clear', '', '重新学习失败', { type: 'danger' })
  } catch (e) {
    console.log(e)
  }
}
//提效
const strategEnable = ref(0)
const strategFun = async () => {
  if (strategEnable.value === 0) strategEnable.value = 1
  else strategEnable.value = 0
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'optimize_switch',
        id: '15',
        params: {
          enable: strategEnable.value
        }
      },
      'ipsbatch'
    )
    const data = res
    let text = ''
    if (strategEnable.value) text = '开启提效'
    else text = '关闭提效'
    if (!data.result.status) proxy.$alertMsg('checked', '', text + '成功', { type: 'success' })
    else proxy.$alertMsg('clear', '', text + '失败', { type: 'danger' })
    getstrategFun()
  } catch (e) {
    console.log(e)
  }
}
const getstrategFun = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_optimize_switch',
        id: '21',
        params: {}
      },
      'ipsbatch'
    )
    const data = res
    console.log(res)
    strategEnable.value = data.result.enable
  } catch (e) {
    console.log(e)
  }
}
let myChart = null
const init = () => {
  const dom = document.getElementById('chart')
  if (!dom) return
  let option = null
  if (!myChart) {
    myChart = echarts.init(dom)
    option = {
      series: [
        {
          type: 'gauge',
          center: ['50%', '60%'],
          radius: '110%',
          startAngle: 180,
          endAngle: -0,
          min: 0,
          max: 200,
          itemStyle: {
            color: realtimeInfo.value.strategy_status === 0 ? '#42B4D2' : '#4A3AFF1A'
          },
          progress: {
            show: true,
            width: 15,
            roundCap: true, // 是否圆顶
            itemStyle: {}
          },
          pointer: {
            show: false
          },
          axisLine: {
            roundCap: true,
            lineStyle: {
              color: [[1, '#4A3AFF1A']],
              width: 15
            }
          },
          axisTick: {
            show: false
          },
          splitLine: {
            show: false
          },
          axisLabel: {
            show: false
          },
          detail: {
            show: false
          },
          data: [
            {
              value: realtimeInfo.value.strategy_feedback
            }
          ]
        },
        {
          type: 'gauge',
          center: ['50%', '60%'],
          radius: '70%',
          startAngle: 180,
          endAngle: -0,
          min: 0,
          max: 200,
          itemStyle: {
            color: '#A5A5A5'
          },
          progress: {
            roundCap: true,
            show: true,
            width: 10
          },
          pointer: {
            show: false
          },
          axisLine: {
            roundCap: true,
            lineStyle: {
              color: [[1, '#D6D6D6']]
            }
          },
          axisTick: {
            show: false
          },
          splitLine: {
            show: false
          },
          axisLabel: {
            show: false
          },
          detail: {
            show: false
          },
          data: [
            {
              value: realtimeInfo.value.nc_knob_feedback
            }
          ]
        }
      ]
    }
  } else {
    option = myChart.getOption()
    option.series[0].data[0].value = realtimeInfo.value.strategy_feedback
    option.series[0].itemStyle.color =
      realtimeInfo.value.strategy_status === 0 ? '#42B4D2' : '#4A3AFF1A'
    option.series[1].data[0].value = realtimeInfo.value.nc_knob_feedback
  }

  myChart.setOption(option)
}
</script>

<style lang="scss" scoped>
$text-clor: #787777;
@mixin boxShadow() {
  background-color: #ffffff;
  box-shadow: -4px 0px #282d43;
  border-radius: 4px;
}
@mixin backgroundwhite() {
  background-color: #f1f1f1;
  border-radius: 4px;
}
.dsshboard {
  display: grid;
  grid-template-columns: 524px 468px;
  grid-column-gap: 16px;
  & > div {
    & > span {
      font-size: 24px;
      color: $text-clor;
    }
    .background {
      margin-top: 16px;
      background-color: #f1f1f1;
      padding: 27px 24px 28px;
      border-radius: 4px;
      .all-time {
        @include boxShadow;
        padding: 20px 9px 27px 19px;
        .text-time {
          display: flex;
          justify-content: space-between;
          align-items: center;
          .text-left {
            font-size: 20px;
            color: $text-clor;
          }
          .text-optimize {
            display: block;
            padding: 2px 8px;
            position: relative;
            margin-right: 7px;
          }
          .text-optimize::before {
            position: absolute;
            content: '';
            left: -20%;
            top: 30%;
            width: 8px;
            height: 8px;
            border-radius: 50%;
          }
          .greens {
            background-color: rgba(42, 227, 4, 0.2);
            color: rgb(43, 193, 85);
          }
          .greens::before {
            background-color: rgb(43, 193, 85);
          }
          .yellow {
            background-color: rgba(235, 106, 2, 0.2);
            color: rgb(235, 106, 2);
          }
          .yellow::before {
            background-color: rgba(235, 106, 2);
          }
          .grey {
            background-color: rgba(165, 165, 165, 0.2);
            color: rgba(165, 165, 165, 0.3);
          }
          .grey::before {
            background-color: rgba(165, 165, 165, 0.3);
          }
        }
        .text-num {
          margin-top: 23px;
          display: flex;
          justify-content: space-between;
          & > div {
            display: flex;
            align-items: flex-end;
            .text-timnum {
              display: inline-block;
              width: 84px;
              height: 83px;
              border-radius: 4px;
              border: 2px solid rgb(198, 198, 198);
              background: linear-gradient(rgb(255, 255, 255), rgb(217, 217, 217));
              text-align: center;
              line-height: 83px;
              font-weight: 500;
              font-size: 60px;
              font-family: DIN;
              color: rgb(40, 45, 67);
            }
            .text-timlogotype {
              font-size: 28px;
              color: rgb(120, 119, 119);
              margin-left: 8px;
              font-family: 思源黑体;
            }
          }
        }
      }
      .text-flex {
        display: flex;
        justify-content: space-between;
        height: 112px;
        margin-top: 16px;
        & > div {
          @include boxShadow;
          padding: 19.5px 0px 19.5px 16px;
          span {
            display: block;
          }
          .text-grey {
            color: $text-clor;
            font-size: 20px;
          }
          .text-number {
            margin-top: 12px;
            font-weight: 600;
            font-size: 24px;
            color: rgb(0, 0, 0);
            font-family: 阿里巴巴普惠体 2;
          }
        }
        .text-alltime {
          width: 259px;
        }
        .text-allnum {
          width: 201px;
        }
      }
      .text-btn {
        div {
          display: flex;
          justify-content: space-between;
          color: $text-clor;
          font-size: 20px;
          font-family: 阿里巴巴普惠体 2;
        }
        & > :first-child {
          margin-top: 48px;
        }
        & > :nth-child(2) {
          margin-top: 29px;
        }
        & > :last-child {
          margin-top: 16px;
        }
      }
    }
    .chart-tab {
      margin-top: 16px;
      height: 155px;
      background-color: #f1f1f1;
      border-radius: 4px;
      margin-bottom: 16px;
      display: flex;
      justify-content: space-between;
      padding: 20px 26px 0 0;
      .position-chart {
        position: relative;
        & > :nth-child(2) {
          position: absolute;
          left: 23px;
          bottom: 10px;
          font-size: 18px;
          color: #4e5969;
        }
        & > :nth-child(3) {
          position: absolute;
          left: 143px;
          bottom: 10px;
          font-size: 18px;
          color: #4e5969;
        }
      }
      .chart {
        width: 200px;
        height: 155px;
      }
      .magnification {
        color: #4e5969;
        margin-top: 27px;
        & > :first-child {
          font-size: 18px;
          & > :first-child {
            color: #42b4d2;
          }
        }
        & > :last-child {
          font-size: 24px;
          display: flex;
          align-items: flex-start;
          margin-left: 34px;
          & > :first-child {
            color: #42b4d2;
            font-size: 44px;
            font-weight: 900;
            height: 52px;
            line-height: 42px;
          }
        }
      }
    }
    .processing {
      margin-top: 16px;
      display: flex;
      justify-content: space-between;
      & > div {
        width: 226px;
        @include backgroundwhite;
        font-size: 48px;
        display: flex;
        justify-content: space-between;
        padding: 19px 20px;
        & > :first-child {
          color: $text-clor;
          font-family: 阿里巴巴普惠体 2;
        }
        & > :last-child {
          font-family: DIN;
        }
      }
    }
    .tool-num {
      @include backgroundwhite;
      display: flex;
      justify-content: space-between;
      margin-top: 16px;
      padding: 0 20px;
      align-items: center;
      height: 56px;
      & > :first-child {
        color: $text-clor;
        font-size: 20px;
        font-family: 阿里巴巴普惠体 2;
      }
      & > :last-child {
        font-family: DIN;
        font-size: 48px;
      }
    }
  }
}
</style>
