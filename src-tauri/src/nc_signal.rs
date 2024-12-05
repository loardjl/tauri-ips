use crate::utils::{read_bytes, read_u16, read_u32, read_u64, read_u8};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NcSignalManager {
    pub realtime_data: HashMap<String, HashMap<u32, NcSignalVal>>,
}
impl NcSignalManager {
    pub fn new() -> Self {
        Self {
            realtime_data: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Value {
    pub sig_id: u32,
    pub sig_data_type: u8,
    pub nums: usize,
    pub buffer_len: usize,
    pub val: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalValue {
    pub sig_id: u32,
    pub sig_type: u32,
    pub sig_freq_type: u32,
    pub sig_data_type: u8,
    pub nums: u32,
    pub buffer_len: u32,
    pub val: Vec<u8>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultSignalValue {
    pub sig_id: u32,
    pub sig_type: u32,
    pub sig_freq_type: u32,
    pub sig_data_type: u8,
    pub nums: u32,
    pub buffer_len: u32,
    pub val: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultData {
    dev_id: u32,
    collector_id: u32,
    timestamp: u64,
    values: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NcSignalVal {
    pub dev_id: u32,
    pub collector_id: u32,
    pub metrics: HashMap<u32, ResultSignalValue>,
    pub data: HashMap<u32, Vec<ResultSignalValue>>,
}

impl NcSignalVal {
    pub fn new(dev_id: u32, collector_id: u32) -> Self {
        let mut data = HashMap::new();
        for key in [
            300, 301, 302, 303, 304, 305, 310, 57, 50, 51, 52, 327, 328, 329, 330, 801, 316, 317,
            318, 319, 320, 321,
        ] {
            data.insert(key, Vec::new());
        }
        Self {
            dev_id,
            collector_id,
            metrics: HashMap::new(),
            data,
        }
    }
}

pub struct NcSignal {
    pub slice_count: u16,
    pub slice_id: u16,
    pub slice_size: u16,
    pub dev_id: u32,
    pub timestamp: u64,
    pub collector_id: u32,
    pub sig_num: u32,
    pub values: Vec<SignalValue>,
}

impl NcSignal {
    pub fn parse_nc_signal(data: &[u8]) -> Result<NcSignal, &'static str> {
        let mut offset = 0;

        let slice_count = read_u16(data, &mut offset)?;
        let slice_id = read_u16(data, &mut offset)?;
        let slice_size = read_u16(data, &mut offset)?;
        let dev_id = read_u32(data, &mut offset)?;
        let timestamp = read_u64(data, &mut offset)?;
        let collector_id = read_u32(data, &mut offset)?;
        let sig_num = read_u32(data, &mut offset)?;

        let mut values = Vec::new();
        for _ in 0..sig_num {
            let sig_id = read_u32(data, &mut offset)?;
            let sig_type = read_u32(data, &mut offset)?;
            let sig_freq_type = read_u32(data, &mut offset)?;
            let sig_data_type = read_u8(data, &mut offset)?;
            let nums = read_u32(data, &mut offset)?;
            let buffer_len = read_u32(data, &mut offset)?;
            let val = read_bytes(data, &mut offset, buffer_len as usize)?;

            values.push(SignalValue {
                sig_id,
                sig_type,
                sig_freq_type,
                sig_data_type,
                nums,
                buffer_len,
                val: val.to_vec(),
            });
        }

        Ok(NcSignal {
            slice_count,
            slice_id,
            slice_size,
            dev_id,
            timestamp,
            collector_id,
            sig_num,
            values,
        })
    }
}

pub fn set_nc_signal_val(realtime_data: &mut HashMap<u32, NcSignalVal>, result: NcSignal) {
    let collector_id = result.collector_id;

    // 初始化采集器的数据存储
    if !realtime_data.contains_key(&collector_id) {
        realtime_data.insert(
            collector_id,
            NcSignalVal::new(result.dev_id, result.collector_id),
        );
    }

    let data_entry = realtime_data.get_mut(&collector_id).unwrap();

    // 遍历所有值进行处理
    for mut value in result.values {
        match value.sig_data_type {
            0 => {
                // 32位整型
                let ints: Vec<u32> = value
                    .val
                    .chunks(4)
                    .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
                    .collect();
                value.val = ints.iter().flat_map(|x| x.to_le_bytes()).collect();
            }
            1 => {
                // 单精度浮点型
                let floats: Vec<f32> = value
                    .val
                    .chunks(4)
                    .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                    .collect();
                value.val = floats
                    .iter()
                    .flat_map(|x| x.to_le_bytes())
                    .collect::<Vec<u8>>();
            }
            3 => {
                // 整型单值
                if !value.val.is_empty() {
                    value.val = vec![value.val[0]];
                }
            }
            _ => {
                // 字符串类型
                if value.buffer_len > 0 {
                    value.val = value
                        .val
                        .iter()
                        .take(value.buffer_len.try_into().unwrap())
                        .cloned()
                        .collect();
                } else {
                    value.val.clear();
                }
            }
        }
        let result_value = ResultSignalValue {
            sig_id: value.sig_id,
            sig_type: value.sig_type,
            sig_freq_type: value.sig_freq_type,
            sig_data_type: value.sig_data_type,
            nums: value.nums,
            buffer_len: value.buffer_len,
            val: value.val,
            timestamp: result.timestamp,
        };
        // 存储解析结果
        if [
            300, 301, 302, 303, 304, 305, 310, 57, 50, 51, 52, 327, 328, 329, 330, 801, 316, 317,
            318, 319, 320, 321,
        ]
        .contains(&value.sig_id)
        {
            data_entry
                .data
                .entry(result_value.sig_id)
                .or_default()
                .push(result_value);
        } else {
            data_entry.metrics.insert(value.sig_id, result_value);
        }
    }
}
