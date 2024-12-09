<template>
  <div class="policyManagement">
    <div class="header">
      <div class="header-name">
        <span
          v-for="item in strategies"
          :key="item.strategy_id"
          :class="[item.strategy_id === strategiesid ? 'activeTitle' : '']"
          @click="strategiesFun(item.strategy_id)"
          >{{ item.strategy_name }}</span
        >
        <span></span>
      </div>
      <div class="header-switch">
        <div>
          <span>按旋钮倍率控制</span>
          <van-switch
            v-model="strategiesItem.knob_multiplier_switch"
            size="24px"
            :active-value="1"
            :inactive-value="0"
          />
        </div>
        <div>
          <span>无接触跳过</span>
          <van-switch
            v-model="strategiesItem.no_touch_skip_switch"
            size="24px"
            :active-value="1"
            :inactive-value="0"
          />
        </div>
        <!-- <div>
          <span>转速补偿</span>
          <integer v-model="strategiesItem.rpm_compensate_rate" title="转速补偿" />
          <van-switch
            size="24px"
            v-model="strategiesItem.rpm_compensate_rate_enable"
            :active-value="1"
            :inactive-value="0"
          />
        </div> -->
      </div>
    </div>
    <div class="tab-item">
      <span
        v-for="item in parameters"
        :key="item.id"
        :class="[item.id === strategyParameters ? 'tabActive' : '']"
        @click="tabFun(item.id)"
        >{{ item.name }}</span
      >
    </div>
    <div class="content">
      <div class="overload" v-show="strategyParameters === 1">
        <div>
          <span>启用状态</span>
          <van-switch
            v-model="strategiesItem.overload_protection_enable"
            size="24px"
            :active-value="1"
            :inactive-value="0"
          />
        </div>
        <van-form
          colon
          label-align="right"
          :validate-trigger="['onChange', 'onBlur']"
          ref="overload"
        >
          <integer
            label="进给倍率"
            name="overloadfeedrate"
            v-model="strategiesItem.overload_protection_feed_rate"
            title="进给倍率(%)"
            :rules="[
              {
                validator: overloadfeedrate
              }
            ]"
          />
          <decimal
            label="系数"
            name="overloadprotection"
            v-model="strategiesItem.overload_protection_learn_factor"
            title="系数(.)"
            :rules="[
              {
                validator: overloadprotection
              }
            ]"
          />
        </van-form>
      </div>
      <div class="optimize" v-show="strategyParameters === 2">
        <div class="opt-header">
          <div>
            <span>启用状态</span>
            <van-switch
              v-model="strategiesItem.optimize_ctrl_enable"
              size="24px"
              :active-value="1"
              :inactive-value="0"
            />
          </div>
          <van-form
            colon
            label-align="right"
            :validate-trigger="['onChange', 'onBlur']"
            ref="optimizectrHeader"
          >
            <integer
              label="进给倍率"
              name="optimizectrlfeed"
              v-model="strategiesItem.optimize_ctrl_feed_rate"
              title="进给倍率(%)"
              :rules="[
                {
                  validator: optimizectrlfeed
                }
              ]"
            />
            <decimal
              label="系数"
              name="optimizectrllearn"
              v-model="strategiesItem.optimize_ctrl_learn_factor"
              title="系数(.)"
              :rules="[
                {
                  validator: optimizectrllearn
                }
              ]"
            />
          </van-form>
        </div>
        <div class="opt-contet">
          <div class="opt-top">
            <span>上限增速</span>
            <van-switch
              v-model="strategiesItem.optimize_ctrl_max_feed_rate_enable"
              size="24px"
              :active-value="1"
              :inactive-value="0"
            />
          </div>
          <div class="opt-btn">
            <van-form
              colon
              label-align="right"
              :validate-trigger="['onChange', 'onBlur']"
              ref="optimizectrBtn"
            >
              <integer
                label="上限进给倍率"
                name="optimizectrlmax"
                v-model="strategiesItem.optimize_ctrl_max_feed_rate"
                title="上限进给倍率(%)"
                :rules="[
                  {
                    validator: optimizectrlmax
                  }
                ]"
                :disabled="!strategiesItem.optimize_ctrl_max_feed_rate_enable ? true : false"
              />
              <integer
                label="下限进给倍率"
                name="optimizectrlmin"
                v-model="strategiesItem.optimize_ctrl_min_feed_rate"
                title="下限进给倍率(%)"
                :rules="[
                  {
                    validator: optimizectrlmin
                  }
                ]"
                :disabled="!strategiesItem.optimize_ctrl_max_feed_rate_enable ? true : false"
              />
            </van-form>
            <!-- <div>
              <span>上限进给倍率:</span>
              <integer
                v-model="strategiesItem.optimize_ctrl_max_feed_rate"
                title="上限进给倍率(%)"
              />
            </div>
            <div>
              <span>下限进给倍率:</span>
              <integer
                v-model="strategiesItem.optimize_ctrl_min_feed_rate"
                title="下限进给倍率(%)"
              />
            </div> -->
          </div>
        </div>
      </div>
      <div class="contact" v-show="strategyParameters === 3">
        <div class="con-header">
          <div>
            <span>启用状态</span>
            <van-switch
              v-model="strategiesItem.touch_enable"
              size="24px"
              :active-value="1"
              :inactive-value="0"
            />
          </div>
          <van-form
            colon
            label-align="right"
            :validate-trigger="['onChange', 'onBlur']"
            ref="touchHeader"
          >
            <integer
              label="进给倍率"
              name="touchfeed"
              v-model="strategiesItem.touch_feed_rate"
              title="进给倍率(%)"
              :rules="[
                {
                  validator: touchfeed
                }
              ]"
            />
          </van-form>
          <!-- <div>
            <span>进给倍率:</span>
            <integer v-model="strategiesItem.touch_feed_rate" title="进给倍率(%)" />
          </div> -->
        </div>
        <div class="con-contet">
          <van-form
            colon
            label-align="right"
            :validate-trigger="['onChange', 'onBlur']"
            ref="touchBtn"
          >
            <div class="protection">
              <span>切入保护</span>
              <van-switch
                v-model="strategiesItem.touch_entry_protection_enable"
                size="24px"
                :active-value="1"
                :inactive-value="0"
              />
            </div>
            <integer
              label="在生效持续时间"
              name="touchfeed"
              v-model="strategiesItem.touch_revival_duration"
              title="在生效持续时间(s)"
              :rules="[{ required: true, message: '请输入在生效持续时间' }]"
              :disabled="!strategiesItem.touch_entry_protection_enable ? true : false"
            />
            <integer
              label="切入保护时间"
              name="touchfeed"
              v-model="strategiesItem.touch_entry_protection_time"
              title="切入保护时间(s)"
              :rules="[{ required: true, message: '请输入在切入保护时间' }]"
              :disabled="!strategiesItem.touch_entry_protection_enable ? true : false"
            />
            <integer
              label="接触保护倍率"
              name="touchprotection"
              v-model="strategiesItem.touch_protection_rate"
              title="接触保护倍率(%)"
              :rules="[
                {
                  validator: touchprotection
                }
              ]"
              :disabled="!strategiesItem.touch_entry_protection_enable ? true : false"
            />
          </van-form>
          <!-- <div>
            <span>在生效持续时间:</span>
            <integer v-model="strategiesItem.touch_revival_duration" title="在生效持续时间(s)" />
          </div>
          <div>
            <span>切入保护时间:</span>
            <decimal v-model="strategiesItem.touch_entry_protection_time" title="切入保护时间(s)" />
          </div>
          <div>
            <span>接触保护倍率:</span>
            <integer v-model="strategiesItem.touch_protection_rate" title="接触保护倍率(%)" />
          </div> -->
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="jsx">
import { onMounted, ref, getCurrentInstance } from 'vue'
import { storeToRefs } from 'pinia'
import integer from './components/integer.vue'
import decimal from './components/decimal.vue'
import { useMenuStore } from '@src/store/useMenu'
import { useApi } from '@src/hooks/useApi'
import popover from '@components/common/popover/inedx.vue'
const { proxy } = getCurrentInstance()
const { fetchPostApi } = useApi()
const menuStore = useMenuStore()
const { asideList } = storeToRefs(menuStore)
const strategies = ref([])
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
          <popover ref={popoverRef} contentText="是否保存修改?" trigger="保存">
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
                        setStrategyFun()
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
  workpieceStrategy()
})
const parameters = [
  {
    id: 1,
    name: '过载保护线'
  },
  {
    id: 2,
    name: '优化控制线'
  },
  {
    id: 3,
    name: '接触线'
  }
]
const strategiesItem = ref({
  overload_protection_feed_rate: 0
})
const strategiesid = ref(0)
const strategiesFun = val => {
  strategiesid.value = val
  strategyParameters.value = 1
  filterStrategiesItem()
}

const filterStrategiesItem = () => {
  const filter = strategies.value.filter(item => item.strategy_id === strategiesid.value)[0]
  if (filter) strategiesItem.value = JSON.parse(JSON.stringify(filter))
}

const strategyParameters = ref(1)
const tabFun = val => {
  strategyParameters.value = val
}

const workpieceStrategy = async () => {
  try {
    const res = await fetchPostApi(
      {
        version: '1.0',
        method: 'get_all_workpiece_strategy',
        id: '7',
        params: {}
      },
      'ipsbatch'
    )
    const data = res

    data.result.strategies.forEach(item => {
      item.optimize_ctrl_learn_factor = item.optimize_ctrl_learn_factor.toFixed(1)
      item.overload_protection_learn_factor = item.overload_protection_learn_factor.toFixed(1)
      item.overload_protection_feed_rate = (item.overload_protection_feed_rate * 100).toFixed(0)
      item.optimize_ctrl_feed_rate = (item.optimize_ctrl_feed_rate * 100).toFixed(0)
      item.optimize_ctrl_max_feed_rate = (item.optimize_ctrl_max_feed_rate * 100).toFixed(0)
      item.optimize_ctrl_min_feed_rate = (item.optimize_ctrl_min_feed_rate * 100).toFixed(0)
      item.touch_feed_rate = (item.touch_feed_rate * 100).toFixed(0)
      item.touch_protection_rate = (item.touch_protection_rate * 100).toFixed(0)
      item.touch_entry_protection_time = (item.touch_entry_protection_time / 1000).toFixed(1)
      item.touch_revival_duration = (item.touch_revival_duration / 1000).toFixed(0)
    })
    strategiesid.value = data.result.strategies[0].strategy_id
    strategies.value = data.result.strategies
    filterStrategiesItem()
    sessionStorage.setItem('strategies', JSON.stringify(strategies.value))
  } catch (e) {
    console.log(e)
  }
}
const popoverRef = ref(null)
const setStrategyFun = async () => {
  const promises = [
    overload.value.validate(),
    optimizectrHeader.value.validate(),
    optimizectrBtn.value.validate(),
    touchHeader.value.validate(),
    touchBtn.value.validate()
  ]
  Promise.all(promises)
    .then(async () => {
      strategiesItem.value.overload_protection_feed_rate =
        strategiesItem.value.overload_protection_feed_rate / 100
      strategiesItem.value.optimize_ctrl_feed_rate =
        strategiesItem.value.optimize_ctrl_feed_rate / 100
      strategiesItem.value.optimize_ctrl_max_feed_rate =
        strategiesItem.value.optimize_ctrl_max_feed_rate / 100
      strategiesItem.value.optimize_ctrl_min_feed_rate =
        strategiesItem.value.optimize_ctrl_min_feed_rate / 100
      strategiesItem.value.touch_feed_rate = strategiesItem.value.touch_feed_rate / 100
      strategiesItem.value.touch_protection_rate = strategiesItem.value.touch_protection_rate / 100
      strategiesItem.value.touch_entry_protection_time =
        strategiesItem.value.touch_entry_protection_time * 1000
      strategiesItem.value.touch_revival_duration =
        strategiesItem.value.touch_revival_duration * 1000
      try {
        const res = await fetchPostApi(
          {
            version: '1.0',
            method: 'set_strategy',
            id: '8',
            params: {
              strategy_parameters: strategiesItem.value
            }
          },
          'ipsbatch'
        )
        const data = res
        popoverRef.value.confirmFun()
        if (!data.result.status) proxy.$alertMsg('checked', '', '保存成功', { type: 'success' })
        else proxy.$alertMsg('clear', '', '保存失败', { type: 'danger' })
        workpieceStrategy()
      } catch (e) {
        console.log(e)
      }
    })
    .catch(() => {
      return false
    })
}
const overloadfeedrate = val => {
  const { optimize_ctrl_feed_rate, touch_feed_rate } = strategiesItem.value
  if (val > optimize_ctrl_feed_rate || val > touch_feed_rate) {
    return '过载保护线不能大于优化控制线和接触线'
  }
}
const overloadprotection = val => {
  const { optimize_ctrl_learn_factor } = strategiesItem.value
  if (val < optimize_ctrl_learn_factor) {
    return '不能小于优化控制线系数'
  }
}
const optimizectrlfeed = val => {
  const { overload_protection_feed_rate, touch_feed_rate } = strategiesItem.value
  if (val < overload_protection_feed_rate || val > touch_feed_rate) {
    return '优化控制线不能小于过载保护线和大于接触线'
  }
}
const optimizectrllearn = val => {
  const { overload_protection_learn_factor } = strategiesItem.value
  if (val > overload_protection_learn_factor) {
    return '不能大于过载保护线系数'
  }
}
const optimizectrlmax = val => {
  const { optimize_ctrl_feed_rate, optimize_ctrl_min_feed_rate } = strategiesItem.value
  if (val < optimize_ctrl_feed_rate) {
    return '不能小于进给倍率'
  } else if (val < optimize_ctrl_min_feed_rate) {
    return '不能小于下限进给倍率'
  }
}
const optimizectrlmin = val => {
  const { optimize_ctrl_feed_rate, optimize_ctrl_max_feed_rate } = strategiesItem.value
  if (val > optimize_ctrl_feed_rate) {
    return '不能大于进给倍率'
  } else if (val > optimize_ctrl_max_feed_rate) {
    return '不能大于上限进给倍率'
  }
}
const touchfeed = val => {
  const { optimize_ctrl_feed_rate, overload_protection_feed_rate } = strategiesItem.value
  if (val < optimize_ctrl_feed_rate || val < overload_protection_feed_rate) {
    return '接触线不能小于过载保护线和优化控制线'
  }
}
const touchprotection = val => {
  const { touch_feed_rate } = strategiesItem.value
  if (val > touch_feed_rate) {
    return '不能大于进给倍率'
  }
}

const overload = ref(null)
const optimizectrHeader = ref(null)
const optimizectrBtn = ref(null)
const touchHeader = ref(null)
const touchBtn = ref(null)
</script>

<style lang="scss" scoped>
.policyManagement {
  .header {
    .header-name {
      background-color: rgb(241, 241, 241);
      height: 86px;
      margin-bottom: 2px;
      display: flex;
      align-items: center;
      justify-content: space-around;
      padding: 0 16px;
      border-radius: 4px;
      & > span {
        display: inline-block;
        width: 182px;
        height: 54px;
        font-size: 22px;
        line-height: 54px;
        text-align: center;
        cursor: pointer;
      }
      .activeTitle {
        background-color: rgb(66, 180, 210);
        color: white;
      }
    }
    .header-switch {
      background-color: rgb(241, 241, 241);
      border-radius: 4px;
      display: flex;
      height: 96px;
      padding: 0 24px;
      align-items: center;
      & > div {
        display: flex;
        align-items: center;
        margin-right: 24px;
        & > span {
          margin-right: 8px;
          font-size: 24px;
        }
        :deep(.van-cell) {
          width: 100px;
          margin-right: 8px;
        }
      }
    }
  }
  .tab-item {
    margin-top: 24px;
    span {
      width: 143px;
      height: 72px;
      display: inline-block;
      background-color: rgb(225, 225, 225);
      border: 1px solid rgb(241, 241, 241);
      font-size: 24px;
      text-align: center;
      line-height: 72px;
      cursor: pointer;
    }
    .tabActive {
      background-color: rgb(241, 241, 241);
    }
  }
  .content {
    background-color: rgb(241, 241, 241);
    height: 308px;
    padding: 16px 0 0 24px;

    .overload {
      display: flex;
      height: 96px;
      align-items: center;
      padding-left: 24px;
      & > div {
        display: flex;
        align-items: center;
        font-size: 22px;
        margin-right: 40px;
        span {
          margin-right: 8px;
        }
      }
      :deep(.van-form) {
        display: flex;
        font-size: 24px;
        .van-cell {
          position: relative;
          padding: 0;
          border: 0;
          font-size: 24px;
          background: transparent;
          overflow: visible;
          margin-right: 24px;
        }
        .van-cell__title {
          width: auto;
          font-size: 24px;
        }
        :deep(.van-field__label) {
          line-height: 64px;
        }
        input {
          width: 250px;
          padding: 0 12px;
          height: 64px;
          line-height: 64px;
          background: #fff;
          border-radius: 4px;
          border: 1px solid #dcdfe6;
          &:disabled {
            background: rgb(224, 224, 224);
          }
        }
      }
      :deep(.van-field .van-field__value) {
        position: relative;
        left: 0;
        top: 0;

        .van-field__error-message {
          // top: 0;
          position: absolute;
          font-size: 16px;
        }
      }
    }
    .optimize {
      .opt-header {
        display: flex;
        height: 96px;
        align-items: center;
        border-bottom: 2px solid rgb(225, 225, 225);
        padding-left: 24px;
        & > div {
          height: 64px;
          display: flex;
          align-items: center;
          font-size: 22px;
          margin-right: 40px;
          span {
            margin-right: 8px;
          }
        }
        :deep(.van-form) {
          display: flex;
          font-size: 24px;
          .van-cell {
            position: relative;
            padding: 0;
            border: 0;
            font-size: 24px;
            background: transparent;
            overflow: visible;
            margin-right: 20px;
          }
          .van-cell__title {
            width: auto;
            font-size: 24px;
          }
          :deep(.van-field__label) {
            line-height: 64px;
          }
          input {
            width: 250px;
            padding: 0 12px;
            height: 64px;
            line-height: 64px;
            background: #fff;
            border-radius: 4px;
            border: 1px solid #dcdfe6;
            &:disabled {
              background: rgb(224, 224, 224);
            }
          }
        }
        :deep(.van-field .van-field__value) {
          position: relative;
          left: 0;
          top: 0;

          .van-field__error-message {
            // top: 0;
            position: absolute;
            font-size: 16px;
          }
        }
      }
      .opt-contet {
        padding-left: 24px;
        font-size: 22px;
        .opt-top {
          display: flex;
          margin-top: 37px;
          span {
            margin-right: 8px;
          }
        }
        .opt-btn {
          display: flex;
          align-items: center;
          margin-top: 37px;
          & > div {
            display: flex;
            align-items: center;
            margin-right: 40px;
            span {
              margin-right: 8px;
            }
          }
          :deep(.van-form) {
            display: flex;
            font-size: 24px;
            .van-cell {
              position: relative;
              padding: 0;
              border: 0;
              font-size: 24px;
              background: transparent;
              overflow: visible;
              margin-right: 20px;
            }
            .van-cell__title {
              width: auto;
              font-size: 24px;
            }
            :deep(.van-field__label) {
              line-height: 64px;
            }
            input {
              width: 250px;
              padding: 0 12px;
              height: 64px;
              line-height: 64px;
              background: #fff;
              border-radius: 4px;
              border: 1px solid #dcdfe6;
              &:disabled {
                background: rgb(224, 224, 224);
              }
            }
          }
          :deep(.van-field .van-field__value) {
            position: relative;
            left: 0;
            top: 0;

            .van-field__error-message {
              // top: 0;
              position: absolute;
              font-size: 16px;
            }
          }
        }
      }
    }
    .contact {
      .con-header {
        display: flex;
        height: 96px;
        align-items: center;
        border-bottom: 2px solid rgb(225, 225, 225);
        padding-left: 24px;
        // :deep(.van-cell) {
        //   width: 250px;
        //   margin-right: 8px;
        // }
        & > div {
          height: 64px;
          display: flex;
          align-items: center;
          font-size: 22px;
          margin-right: 40px;
          span {
            margin-right: 8px;
          }
        }
        :deep(.van-form) {
          display: flex;
          font-size: 24px;
          .van-cell {
            position: relative;
            padding: 0;
            border: 0;
            font-size: 24px;
            background: transparent;
            overflow: visible;
            margin-right: 20px;
          }
          .van-cell__title {
            width: auto;
            font-size: 24px;
          }
          :deep(.van-field__label) {
            line-height: 64px;
          }
          input {
            width: 250px;
            padding: 0 12px;
            height: 64px;
            line-height: 64px;
            background: #fff;
            border-radius: 4px;
            border: 1px solid #dcdfe6;
            &:disabled {
              background: rgb(224, 224, 224);
            }
          }
        }
        :deep(.van-field .van-field__value) {
          position: relative;
          left: 0;
          top: 0;

          .van-field__error-message {
            // top: 0;
            position: absolute;
            font-size: 16px;
          }
        }
      }
      .con-contet {
        padding-left: 24px;
        font-size: 22px;
        display: flex;
        flex-wrap: wrap;
        margin-top: 16px;
        & > div {
          display: flex;
          align-items: center;
          margin-right: 40px;
          span {
            margin-right: 8px;
          }
        }
        // & > :nth-child(2) {
        //   :deep(.van-cell) {
        //     width: 180px;
        //     margin-right: 8px;
        //   }
        // }
        // & > :nth-child(3) {
        //   :deep(.van-cell) {
        //     width: 180px;
        //     margin-right: 8px;
        //   }
        // }
        // & > :last-child {
        //   margin-top: 16px;
        //   :deep(.van-cell) {
        //     width: 203px;
        //     margin-right: 8px;
        //   }
        // }
        :deep(.van-form) {
          display: flex;
          font-size: 24px;
          flex-wrap: wrap;
          align-items: center;
          .protection {
            display: flex;
            align-items: center;
            margin-right: 40px;
            span {
              margin-right: 8px;
            }
          }
          .van-cell {
            position: relative;
            padding: 0;
            border: 0;
            font-size: 24px;
            background: transparent;
            overflow: visible;
            margin-right: 20px;
          }
          .van-cell__title {
            width: auto;
            font-size: 24px;
          }
          :deep(.van-field__label) {
            line-height: 64px;
          }
          input {
            width: 180px;
            padding: 0 12px;
            height: 64px;
            line-height: 64px;
            background: #fff;
            border-radius: 4px;
            border: 1px solid #dcdfe6;
            &:disabled {
              background: rgb(224, 224, 224);
            }
          }
        }
        :deep(.van-field .van-field__value) {
          position: relative;
          left: 0;
          top: 0;

          .van-field__error-message {
            // top: 0;
            position: absolute;
            font-size: 16px;
          }
        }
      }
    }
  }
}
</style>
