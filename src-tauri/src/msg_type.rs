use bytemuck::{Pod, Zeroable};
use byteorder::{BigEndian, ReadBytesExt}; // 引入 byteorder
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{Cursor, Error};
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Pod, Zeroable)]
pub struct ProtocolHeader {
    pub header: u16, // 同步头
    pub version: u8, // 版本号
    pub order1: u16, // 命令1
    pub order2: u16, // 命令2
    pub state: u8,   // 状态
    pub reset: u32,  // 备用
    pub vor: u8,     // 校验码
    pub len: u16,    // 数据长度
}

pub fn parse_protocol_header(data: &[u8]) -> Result<ProtocolHeader, Error> {
    // 检查字节长度是否足够
    if data.len() < std::mem::size_of::<ProtocolHeader>() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Not enough bytes",
        ));
    }

    // 创建一个 Cursor 用于逐字节读取数据
    let mut rdr = Cursor::new(data);

    // 使用 ReadBytesExt 逐个字段读取并按小端字节序转换
    let header = ProtocolHeader {
        header: rdr.read_u16::<BigEndian>()?, // 同步头
        version: rdr.read_u8()?,              // 版本号
        order1: rdr.read_u16::<BigEndian>()?, // 命令1
        order2: rdr.read_u16::<BigEndian>()?, // 命令2
        state: rdr.read_u8()?,                // 状态
        reset: rdr.read_u32::<BigEndian>()?,  // 备用
        vor: rdr.read_u8()?,                  // 校验码
        len: rdr.read_u16::<BigEndian>()?,    // 数据长度
    };

    Ok(header)
}
#[derive(Debug, PartialEq, Clone)]
pub enum ProcessingMode {
    Fragmented,     // 表示需要分片处理，总分片数，结构体解析
    Direct,         // 表示直接发送，无需分片，结构体解析
    FragmentedJson, // 表示需要分片处理，总分片数，json格式
    DirectJson,     // 表示直接发送，无需分片，json格式
}

pub fn get_process_mode(order1: u16, order2: u16) -> ProcessingMode {
    let mut mode_map = HashMap::new();
    mode_map.insert((0x8013, 0x000A), ProcessingMode::FragmentedJson);
    mode_map.insert((0x8013, 0x0009), ProcessingMode::FragmentedJson);
    mode_map.insert((0x0016, 0x0102), ProcessingMode::Fragmented);
    mode_map.insert((0x0016, 0x0103), ProcessingMode::Fragmented);

    mode_map
        .get(&(order1, order2))
        .cloned()
        .unwrap_or(ProcessingMode::DirectJson)
}

pub fn read_bytes<'a>(
    data: &'a [u8],
    offset: &mut usize,
    size: usize,
) -> Result<&'a [u8], &'static str> {
    if *offset + size > data.len() {
        return Err("Buffer underflow while reading bytes");
    }
    let value = &data[*offset..*offset + size];
    *offset += size;
    Ok(value)
}

pub fn read_u32(data: &[u8], offset: &mut usize) -> Result<u32, &'static str> {
    let bytes = read_bytes(data, offset, 4)?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

pub fn read_f64(data: &[u8], offset: &mut usize) -> Result<f64, &'static str> {
    let bytes = read_bytes(data, offset, 8)?;
    Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
}

pub fn read_string(data: &[u8], offset: &mut usize, len: usize) -> Result<String, &'static str> {
    let bytes = read_bytes(data, offset, len)?;
    std::str::from_utf8(bytes)
        .map(|s| s.to_string())
        .map_err(|_| "Failed to parse UTF-8 string")
}

#[repr(C)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RealTimeData {
    pub workpiece_id: u32,      // 工件 ID
    pub program_number: String, // 程序号
    pub tool_number: String,    // 刀具号
    pub actual_rpm: f64,        // 实际转速
    pub actual_feedback: f64,   // 实际进给
    pub strategy_feedback: f64, // 策略中输出的进给控制倍率
    pub nc_knob_feedback: f64,  // NC采到的进给旋钮倍率
}
impl RealTimeData {
    // 解析 RealTimeData
    pub fn parse_real_time_data(data: &[u8]) -> Result<RealTimeData, &'static str> {
        let mut offset = 0;

        let workpiece_id = read_u32(data, &mut offset)?;
        let program_len = read_u32(data, &mut offset)? as usize;
        let program_number = read_string(data, &mut offset, program_len)?;

        let tool_len = read_u32(data, &mut offset)? as usize;
        let tool_number = read_string(data, &mut offset, tool_len)?;

        let actual_rpm = read_f64(data, &mut offset)?;
        let actual_feedback = read_f64(data, &mut offset)?;
        let strategy_feedback = read_f64(data, &mut offset)?;
        let nc_knob_feedback = read_f64(data, &mut offset)?;

        Ok(RealTimeData {
            workpiece_id,
            program_number,
            tool_number,
            actual_rpm,
            actual_feedback,
            strategy_feedback,
            nc_knob_feedback,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OptimizeInfo {
    pub strategy_status: u32, //当前策略状态：0-绿色的 效率优化 1-黄色的 过程等待 2-灰色的 优化关闭
    pub total_optimize_time: u32, //总提效时间
    pub total_processing_time: u32, //总加工时间
    pub total_processing_count: u32, //总加工件数
    pub dayly_optimize_time: u32, //当日提效时间
    pub weekly_optimize_time: u32, //最近一周提效时间
    pub monthly_optimize_time: u32, //最近一个月提效时间
}

impl OptimizeInfo {
    pub fn to_string(data: &[u8]) -> Result<OptimizeInfo, &'static str> {
        let mut offset = 0;

        let strategy_status = read_u32(data, &mut offset)?;
        let total_optimize_time = read_u32(data, &mut offset)?;
        let total_processing_time = read_u32(data, &mut offset)?;
        let total_processing_count = read_u32(data, &mut offset)?;
        let dayly_optimize_time = read_u32(data, &mut offset)?;
        let weekly_optimize_time = read_u32(data, &mut offset)?;
        let monthly_optimize_time = read_u32(data, &mut offset)?;

        Ok(OptimizeInfo {
            strategy_status,
            total_optimize_time,
            total_processing_time,
            total_processing_count,
            dayly_optimize_time,
            weekly_optimize_time,
            monthly_optimize_time,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MsgType {
    RealTimeData,
    OptimizeInfo,
    Unknown, // 未知类型
}

impl MsgType {
    pub fn to_string(&self) -> String {
        match self {
            MsgType::RealTimeData => "RealTimeData".to_string(),
            MsgType::OptimizeInfo => "OptimizeInfo".to_string(),
            MsgType::Unknown => "Unknown".to_string(),
        }
    }
}

pub fn get_msg(order1: u16, order2: u16) -> MsgType {
    let mut mode_map = HashMap::new();
    mode_map.insert((0x0016, 0x0102), MsgType::RealTimeData);
    mode_map.insert((0x0016, 0x0103), MsgType::OptimizeInfo);
    mode_map
        .get(&(order1, order2))
        .cloned()
        .unwrap_or(MsgType::Unknown)
}