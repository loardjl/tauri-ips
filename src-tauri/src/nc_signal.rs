use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    sig_id: u32,
    sig_data_type: u8,
    nums: usize,
    buffer_len: usize,
    val: Vec<u8>,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultData {
    dev_id: u32,
    collector_id: u32,
    timestamp: u64,
    values: Vec<Value>,
}

#[derive(Debug)]
pub struct RealTimeData {
    dev_id: u32,
    collector_id: u32,
    metrics: HashMap<u32, Value>,
    data: HashMap<u32, Vec<Value>>,
}

impl RealTimeData {
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
pub fn nc_signal_val(
    realtime_data: &mut HashMap<u32, RealTimeData>,
    data_buff: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let result: ResultData = serde_json::from_slice(data_buff)?;

    // 获取 `collector_id`
    let collector_id = result.collector_id;

    // 初始化采集器的数据存储
    if !realtime_data.contains_key(&collector_id) {
        realtime_data.insert(
            collector_id,
            RealTimeData::new(result.dev_id, result.collector_id),
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
                    value.val = value.val.iter().take(value.buffer_len).cloned().collect();
                } else {
                    value.val.clear();
                }
            }
        }

        // 存储解析结果
        if [
            300, 301, 302, 303, 304, 305, 310, 57, 50, 51, 52, 327, 328, 329, 330, 801, 316, 317,
            318, 319, 320, 321,
        ]
        .contains(&value.sig_id)
        {
            value.timestamp = result.timestamp;
            data_entry.data.entry(value.sig_id).or_default().push(value);
        } else {
            data_entry.metrics.insert(value.sig_id, value);
        }
    }

    Ok(())
}
