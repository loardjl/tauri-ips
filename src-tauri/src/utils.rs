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

pub fn read_u8(data: &[u8], offset: &mut usize) -> Result<u8, &'static str> {
    let bytes = read_bytes(data, offset, 1)?;
    Ok(u8::from_le_bytes(bytes.try_into().unwrap()))
}

pub fn read_u16(data: &[u8], offset: &mut usize) -> Result<u16, &'static str> {
    let bytes = read_bytes(data, offset, 2)?;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

pub fn read_u32(data: &[u8], offset: &mut usize) -> Result<u32, &'static str> {
    let bytes = read_bytes(data, offset, 4)?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}
pub fn read_i32(data: &[u8], offset: &mut usize) -> Result<i32, &'static str> {
    let bytes = read_bytes(data, offset, 4)?;
    Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
}
pub fn read_u64(data: &[u8], offset: &mut usize) -> Result<u64, &'static str> {
    let bytes = read_bytes(data, offset, 8)?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
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
