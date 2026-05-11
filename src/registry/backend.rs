use std::cmp::PartialEq;
use std::collections::BTreeMap;
use winreg::RegKey;
use winreg::enums::*;

use crate::tweak::schema::TweakFile;
use crate::tweak::schema::ValueType;
use crate::tweak::schema::{Operation, RegValue, Tweak};

#[derive(Debug)]
pub enum RegveilError {
    UnsupportedType,
    FailedOpen,
    Io(std::io::Error),
}

impl From<std::io::Error> for RegveilError {
    fn from(e: std::io::Error) -> Self {
        RegveilError::Io(e)
    }
}

pub fn is_default(now_value: RegValue, op: &Operation) -> bool {
    now_value == op.value_enabled
}

pub fn now(op: &Operation) -> Result<RegValue, RegveilError> {
    let hive_key = RegKey::predef(op.hive.to_hkey());

    let key = hive_key.open_subkey(std::path::Path::new(&op.path)).unwrap();

    let res = key.get_raw_value(&op.name);

    let Ok(value) = res else {
        return Err(RegveilError::FailedOpen);
    };

    let bytes: &[u8] = &value.bytes;

    match value.vtype {
        RegType::REG_DWORD if bytes.len() == 4 => {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(bytes);
            Ok(RegValue::Integer(i32::from_ne_bytes(buf) as i64))
        }

        RegType::REG_QWORD if bytes.len() == 8 => {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(bytes);
            Ok(RegValue::Integer(i64::from_ne_bytes(buf)))
        }
        RegType::REG_SZ => {
            let u16_chars: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
                .collect();

            let clean_chars = match u16_chars.last() {
                Some(&0) => &u16_chars[..u16_chars.len() - 1],
                _ => &u16_chars[..],
            };

            Ok(RegValue::Sz(String::from_utf16_lossy(clean_chars)))
        }
        _ => Err(RegveilError::UnsupportedType),
    }
}

pub fn setup(tweak_files: &[TweakFile]) -> BTreeMap<String, bool> {
    tweak_files
        .iter()
        .flat_map(|tf| &tf.tweaks)
        .map(|tweak| {
            let is_default = tweak.operations.first()
                .and_then(|op| {
                    now(&op).ok().map(|v| v == op.value_enabled)
                })
                .unwrap_or(false);
            (tweak.id.clone(), is_default)
        })
        .collect()
}

pub fn write(op: &Operation, value: &RegValue) -> Result<(), RegveilError> {
    let hive_key = RegKey::predef(op.hive.to_hkey());
    let (key, _) = hive_key.create_subkey(&op.path)?;

    match (value, &op.value_type) {
        (RegValue::Integer(n), ValueType::Dword) => {
            key.set_value(&op.name, &(*n as u32))?;
        }
        (RegValue::Integer(n), ValueType::Qword) => {
            key.set_value(&op.name, &(*n as u64))?;
        }
        (RegValue::Sz(s), ValueType::Sz) => {
            key.set_value(&op.name, s)?;
        }
        _ => return Err(RegveilError::UnsupportedType),
    }
    Ok(())
}
