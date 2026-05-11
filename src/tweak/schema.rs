use serde::Deserialize;
use winreg::{HKEY, enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS}};

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

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub hive: Hive,
    pub path: String,
    pub name: String,
    pub value_type: ValueType,
    pub value_enabled: RegValue,
    pub value_disabled: RegValue,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Hive {
    Hkcu,
    Hklm,
    Hkcr,
    Hku,
}

impl Hive {
    pub fn to_hkey(&self) -> HKEY {
        match self {
            Self::Hkcu => HKEY_CURRENT_USER,
            Self::Hklm => HKEY_LOCAL_MACHINE,
            Self::Hkcr => HKEY_CLASSES_ROOT,
            Self::Hku  => HKEY_USERS,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValueType {
    Dword,
    Qword,
    Sz,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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