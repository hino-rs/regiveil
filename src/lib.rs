use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TweakFile {
    pub meta: CategoryMeta,
    pub tweaks: Vec<Tweak>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryMeta {
    pub label: String,
    pub description: String,
    pub icon: String,
    pub order: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tweak {
    pub id: String,
    pub schema_version: u8,
    pub label: String,
    pub description: String,
    pub tags: Vec<String>,
    pub risk: Risk,
    pub requires_reboot: bool,
    pub requires_admin: bool,
    pub docs_url: Option<String>,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub hive: Hive,
    pub path: String,
    pub name: String,
    pub value_type: ValueType,
    pub value_enabled: RegValue,
    pub value_disabled: RegValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Hive {
    Hkcu,
    Hklm,
    Hkcr,
    Hku,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValueType {
    Dword,
    Qword,
    Sz,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RegValue {
    Integer(i64),
    Sz(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Risk {
    Low,
    Medium,
    High,
}
pub fn load() -> Option<Vec<TweakFile>> {
    let tweaks_dir = match std::fs::read_dir("tweaks") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("ディレクトリの読み取りに失敗しました: {e}");
            return None;
        }
    };

    let mut tweaks_files: Vec<TweakFile> = Vec::new();
    
    for entry in tweaks_dir {
        let Ok(entry) = entry else {
            eprintln!("エントリーの取得に失敗しました。");
            continue;
        };
        
        let path = entry.path();

        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }

        println!("{:?} を読み取ります...", path.file_name().unwrap());

        let string = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{:?} の読み込みに失敗しました: {}", path, e);
                continue;
            }
        };

        match toml::from_str::<TweakFile>(&string) {
            Ok(tweak_file) => {
                tweaks_files.push(tweak_file);
            }
            Err(e) => {
                eprintln!("{path:?} のパースに失敗しました: {}", e.message());
                continue;
            }
        }
    }

    if tweaks_files.len() > 0 {
        return Some(tweaks_files);
    }
    
    None
}
