use winreg::enums::*;
use winreg::RegKey;

use crate::tweak::schema::RegValue;

#[derive(Debug)]
pub enum RegValueError {
    UnsupportedType,
    FailedOpen,
}

pub fn now(path: &str, name: &str) -> Result<RegValue, RegValueError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let key = hkcu.open_subkey(std::path::Path::new(path)).unwrap();

    let res = key.get_raw_value(name);

    let Ok(value) = res else {
        return Err(RegValueError::FailedOpen);
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
                Some(&0) => &u16_chars[..u16_chars.len() -1],
                _ => &u16_chars[..],
            };

            Ok(RegValue::Sz(String::from_utf16_lossy(clean_chars)))
        }
        _ => { Err(RegValueError::UnsupportedType) }
    }
}