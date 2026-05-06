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