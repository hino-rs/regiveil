use crate::system::{notify_explorer, notify_system};
use std::io;
use winreg::enums::*;
use winreg::RegKey;
use winreg::HKEY;

pub enum RegData {
    Dword(u32),
    String(String),
}

pub struct RegistryTweak {
    pub hive: HKEY,
    pub sub_key: &'static str,
    pub value_name: &'static str,
    pub data: RegData,
}

impl RegistryTweak {
    pub fn apply(&self) -> io::Result<()> {
        let hive_key = RegKey::predef(self.hive);
        let (key, _) = hive_key.create_subkey(self.sub_key)?;

        match &self.data {
            RegData::Dword(val) => key.set_value(self.value_name, val)?,
            RegData::String(val) => key.set_value(self.value_name, val)?,
        }
        Ok(())
    }
}

pub struct Windows11Tweaks;

impl Windows11Tweaks {
    pub fn show_hidden_files() -> RegistryTweak {
        RegistryTweak {
            hive: HKEY_CURRENT_USER,
            sub_key: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
            value_name: "Hidden",
            data: RegData::Dword(1),
        }
    }

    pub fn show_file_extensions() -> RegistryTweak {
        RegistryTweak {
            hive: HKEY_CURRENT_USER,
            sub_key: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced",
            value_name: "HideFileExt",
            data: RegData::Dword(0),
        }
    }
}

// apply関数もここに持たせると、UI側がスッキリします
pub fn apply_tweaks(tweaks: Vec<RegistryTweak>) {
    println!("適用させます...");
    let mut applied_count = 0;
    for (i, tweak) in tweaks.iter().enumerate() {
        match tweak.apply() {
            Ok(_) => {
                println!("[{}] 成功: {} を更新しました", i + 1, tweak.value_name);
                applied_count += 1;
            }
            Err(e) => eprintln!("[{}] 失敗: {} ({})", i + 1, tweak.value_name, e),
        }
    }

    if applied_count > 0 {
        println!("設定の反映処理を実行します...");
        notify_explorer();
        notify_system();
    }
}
