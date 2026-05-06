use crate::tweak::schema::{
    CategoryMeta, Hive, Operation, RegValue, Risk, Tweak, TweakFile, ValueType,
};

#[derive(Debug)]
pub enum LoadError {
    DirNotFound(std::io::Error),
    NoTweaksFound,
}

pub fn load() -> Result<Vec<TweakFile>, LoadError> {
    let tweaks_dir = match std::fs::read_dir("tweaks") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("ディレクトリの読み取りに失敗しました: {e}");
            return Err(LoadError::DirNotFound(e));
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

    
    let mut seen_ids = std::collections::HashSet::new();
    for tweak_file in &tweaks_files {
        for tweak in &tweak_file.tweaks {
            if !seen_ids.insert(&tweak.id) {
                eprintln!("id重複: {}", tweak.id);
                panic!("tweakのid重複は禁止です。");
            }
        }
    }

    
    if !tweaks_files.is_empty() {
        return Ok(tweaks_files);
    }
    
    Err(LoadError::NoTweaksFound)
}
