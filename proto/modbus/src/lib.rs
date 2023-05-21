// ---------------------------------------------------------------------
// Modbus data
// ---------------------------------------------------------------------
// Copyright (C) 2021-2023 Gufo Labs
// See LICENSE for details
// ---------------------------------------------------------------------

use common::{AgentError, AgentResult, Value};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Deserialize, Serialize, Debug, Clone, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RegisterType {
    Holding,
    Input,
    Coil,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Hash)]
pub enum ModbusFormat {
    // 16 bit
    #[serde(rename = "i16_be")]
    I16Be,
    #[serde(rename = "u16_be")]
    U16Be,
    // 32 bit
    #[serde(rename = "i32_be")]
    I32Be,
    #[serde(rename = "i32_le")]
    I32Le,
    #[serde(rename = "i32_bs")]
    I32Bs,
    #[serde(rename = "i32_ls")]
    I32Ls,
    #[serde(rename = "u32_be")]
    U32Be,
    #[serde(rename = "u32_le")]
    U32Le,
    #[serde(rename = "u32_bs")]
    U32Bs,
    #[serde(rename = "u32_ls")]
    U32Ls,
    // Float
    #[serde(rename = "f32_be")]
    F32Be,
    #[serde(rename = "f32_le")]
    F32Le,
    #[serde(rename = "f32_bs")]
    F32Bs,
    #[serde(rename = "f32_ls")]
    F32Ls,
}

impl ModbusFormat {
    pub fn modbus_try_from(self, v: Vec<u16>) -> AgentResult<Value> {
        match self {
            ModbusFormat::I16Be => read_i16be(v),
            ModbusFormat::U16Be => read_u16be(v),
            ModbusFormat::I32Be => read_i32be(v),
            ModbusFormat::I32Le => read_i32le(v),
            ModbusFormat::I32Bs => read_i32bs(v),
            ModbusFormat::I32Ls => read_i32ls(v),
            ModbusFormat::U32Be => read_u32be(v),
            ModbusFormat::U32Le => read_u32le(v),
            ModbusFormat::U32Bs => read_u32bs(v),
            ModbusFormat::U32Ls => read_u32ls(v),
            ModbusFormat::F32Be => read_f32be(v),
            ModbusFormat::F32Le => read_f32le(v),
            ModbusFormat::F32Bs => read_f32bs(v),
            ModbusFormat::F32Ls => read_f32ls(v),
        }
    }
    pub fn min_count(self) -> u16 {
        match self {
            ModbusFormat::I16Be => 1,
            ModbusFormat::U16Be => 1,
            ModbusFormat::I32Be => 2,
            ModbusFormat::I32Le => 2,
            ModbusFormat::I32Bs => 2,
            ModbusFormat::I32Ls => 2,
            ModbusFormat::U32Be => 2,
            ModbusFormat::U32Le => 2,
            ModbusFormat::U32Bs => 2,
            ModbusFormat::U32Ls => 2,
            ModbusFormat::F32Be => 2,
            ModbusFormat::F32Le => 2,
            ModbusFormat::F32Bs => 2,
            ModbusFormat::F32Ls => 2,
        }
    }
}
// Decode vector functions
#[inline]
fn from_u32be(v: Vec<u16>) -> u64 {
    ((v[0] as u64) << 16) + v[1] as u64
}
#[inline]
fn from_u32le(v: Vec<u16>) -> u64 {
    let o1 = (v[1] & 0xff) as u64;
    let o2 = ((v[1] >> 8) & 0xff) as u64;
    let o3 = (v[0] & 0xff) as u64;
    let o4 = ((v[0] >> 8) & 0xff) as u64;
    (o1 << 24) + (o2 << 16) + (o3 << 8) + o4
}
#[inline]
fn from_u32bs(v: Vec<u16>) -> u64 {
    let o1 = (v[0] & 0xff) as u64;
    let o2 = ((v[0] >> 8) & 0xff) as u64;
    let o3 = (v[1] & 0xff) as u64;
    let o4 = ((v[1] >> 8) & 0xff) as u64;
    (o1 << 24) + (o2 << 16) + (o3 << 8) + o4
}
#[inline]
fn from_u32ls(v: Vec<u16>) -> u64 {
    let o1 = ((v[1] >> 8) & 0xff) as u64;
    let o2 = (v[1] & 0xff) as u64;
    let o3 = ((v[0] >> 8) & 0xff) as u64;
    let o4 = (v[0] & 0xff) as u64;
    (o1 << 24) + (o2 << 16) + (o3 << 8) + o4
}
#[inline]
fn from_signed32(v: u64) -> i64 {
    if v >= 0x80000000 {
        v as i64 - 0x100000000
    } else {
        v as i64
    }
}
// Layouts
// 16-bit
fn read_i16be(v: Vec<u16>) -> AgentResult<Value> {
    if v.is_empty() {
        return Err(AgentError::ParseError("empty data".to_string()));
    }
    let x = v[0];
    Ok(Value::GaugeI(if x >= 0x8000 {
        x as i64 - 0x10000
    } else {
        x as i64
    }))
}
fn read_u16be(v: Vec<u16>) -> AgentResult<Value> {
    if v.is_empty() {
        return Err(AgentError::ParseError("empty data".to_string()));
    }
    Ok(Value::Gauge(v[0] as u64))
}
// 32 bit unsigned
fn read_u32be(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::Gauge(from_u32be(v)))
}
fn read_u32le(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::Gauge(from_u32le(v)))
}
fn read_u32bs(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::Gauge(from_u32bs(v)))
}
fn read_u32ls(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::Gauge(from_u32ls(v)))
}
// 32 bit signed
fn read_i32be(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeI(from_signed32(from_u32be(v))))
}
fn read_i32le(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeI(from_signed32(from_u32le(v))))
}
fn read_i32bs(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeI(from_signed32(from_u32bs(v))))
}
fn read_i32ls(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeI(from_signed32(from_u32ls(v))))
}
// Float
fn read_f32be(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeF(f32::from_bits(from_u32be(v) as u32)))
}
fn read_f32le(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeF(f32::from_bits(from_u32le(v) as u32)))
}
fn read_f32bs(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeF(f32::from_bits(from_u32bs(v) as u32)))
}
fn read_f32ls(v: Vec<u16>) -> AgentResult<Value> {
    if v.len() < 2 {
        return Err(AgentError::ParseError("short data".to_string()));
    }
    Ok(Value::GaugeF(f32::from_bits(from_u32ls(v) as u32)))
}

#[cfg(test)]
mod tests {
    use super::{ModbusFormat, Value};
    use assert_approx_eq::assert_approx_eq;
    use std::iter::FromIterator;

    // Convert vec of u8 to modbus-style vec of registers
    fn into_vec16(data: &[u8]) -> Vec<u16> {
        Vec::from_iter(
            data.chunks(2)
                .map(|chunk| ((chunk[0] as u16) << 8) + (chunk[1] as u16)),
        )
    }

    #[test]
    fn test_into_vec16_1() {
        let data = [1u8, 2u8];
        let exp = [0x0102u16];
        assert_eq!(into_vec16(&data), Vec::from(&exp[..]));
    }

    #[test]
    fn test_into_vec16_2() {
        let data = [1u8, 2u8, 3u8, 4u8];
        let exp = [0x0102u16, 0x0304u16];
        assert_eq!(into_vec16(&data), Vec::from(&exp[..]));
    }

    #[test]
    fn test_i16_be_1() {
        let data = [1u8, 2u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I16Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(258));
    }

    #[test]
    fn test_i16_be_2() {
        let data = [0xffu8, 1u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I16Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(-255));
    }

    #[test]
    fn test_u16_be_1() {
        let data = [1u8, 2u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U16Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(258));
    }

    #[test]
    fn test_u16_be_2() {
        let data = [0xffu8, 1u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U16Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(65281));
    }
    #[test]
    fn test_i32_be_1() {
        let data = [1u8, 2u8, 3u8, 4u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(16909060));
    }

    #[test]
    fn test_i32_be_2() {
        let data = [0xffu8, 0xfeu8, 0xfdu8, 0xfcu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(-66052));
    }
    #[test]
    fn test_i32_le_1() {
        let data = [4u8, 3u8, 2u8, 1u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Le.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(16909060));
    }

    #[test]
    fn test_i32_le_2() {
        let data = [0xfcu8, 0xfdu8, 0xfeu8, 0xffu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Le.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(-66052));
    }
    #[test]
    fn test_i32_bs_1() {
        let data = [2u8, 1u8, 4u8, 3u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Bs.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(16909060));
    }

    #[test]
    fn test_i32_bs_2() {
        let data = [0xfeu8, 0xffu8, 0xfcu8, 0xfdu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Bs.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(-66052));
    }
    #[test]
    fn test_i32_ls_1() {
        let data = [3u8, 4u8, 1u8, 2u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Ls.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(16909060));
    }

    #[test]
    fn test_i32_ls_2() {
        let data = [0xfdu8, 0xfcu8, 0xffu8, 0xfeu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::I32Ls.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::GaugeI(-66052));
    }
    #[test]
    fn test_u32_be_1() {
        let data = [1u8, 2u8, 3u8, 4u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(16909060));
    }

    #[test]
    fn test_u32_be_2() {
        let data = [0xffu8, 0xfeu8, 0xfdu8, 0xfcu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Be.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(4294901244));
    }
    #[test]
    fn test_u32_le_1() {
        let data = [4u8, 3u8, 2u8, 1u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Le.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(16909060));
    }

    #[test]
    fn test_u32_le_2() {
        let data = [0xfcu8, 0xfdu8, 0xfeu8, 0xffu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Le.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(4294901244));
    }
    #[test]
    fn test_u32_bs_1() {
        let data = [2u8, 1u8, 4u8, 3u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Bs.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(16909060));
    }

    #[test]
    fn test_u32_bs_2() {
        let data = [0xfeu8, 0xffu8, 0xfcu8, 0xfdu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Bs.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(4294901244));
    }
    #[test]
    fn test_u32_ls_1() {
        let data = [3u8, 4u8, 1u8, 2u8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Ls.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(16909060));
    }

    #[test]
    fn test_u32_ls_2() {
        let data = [0xfdu8, 0xfcu8, 0xffu8, 0xfeu8];
        let msg = into_vec16(&data);
        let result = ModbusFormat::U32Ls.modbus_try_from(msg).unwrap();
        assert_eq!(result, Value::Gauge(4294901244));
    }
    #[test]
    fn test_f32_be_1() {
        let data = [0x3fu8, 0x80u8, 0x37u8, 0x4du8];
        let msg = into_vec16(&data);
        match ModbusFormat::F32Be.modbus_try_from(msg).unwrap() {
            Value::GaugeF(v) => assert_approx_eq!(v, 1.0016876, 1e-6),
            _ => panic!("invalid type"),
        }
    }
    #[test]
    fn test_f32_le_1() {
        let data = [0x4du8, 0x37u8, 0x80u8, 0x3fu8];
        let msg = into_vec16(&data);
        match ModbusFormat::F32Le.modbus_try_from(msg).unwrap() {
            Value::GaugeF(v) => assert_approx_eq!(v, 1.0016876, 1e-6),
            _ => panic!("invalid type"),
        }
    }
    #[test]
    fn test_f32_bs_1() {
        let data = [0x80u8, 0x3fu8, 0x4du8, 0x37u8];
        let msg = into_vec16(&data);
        match ModbusFormat::F32Bs.modbus_try_from(msg).unwrap() {
            Value::GaugeF(v) => assert_approx_eq!(v, 1.0016876, 1e-6),
            _ => panic!("invalid type"),
        }
    }
    #[test]
    fn test_f32_ls_1() {
        let data = [0x37u8, 0x4du8, 0x3fu8, 0x80u8];
        let msg = into_vec16(&data);
        match ModbusFormat::F32Ls.modbus_try_from(msg).unwrap() {
            Value::GaugeF(v) => assert_approx_eq!(v, 1.0016876, 1e-6),
            _ => panic!("invalid type"),
        }
    }
}
