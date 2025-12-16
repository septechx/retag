use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub scripts: Vec<PathBuf>,
    pub root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scripts: vec![PathBuf::from("./script.lua")],
            root: PathBuf::from("./music/"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MinVersion {
    version: u32,
}

impl Config {
    pub fn load() -> Result<Option<Self>> {
        if !PathBuf::from("./retag.yml").exists() {
            return Ok(None);
        }

        let config = fs::read_to_string("./retag.yml")?;
        let version: MinVersion = serde_yml::from_str(&config)?;

        if version.version != 1 {
            return Err(anyhow::anyhow!("Unsupported config version"));
        }

        Ok(Some(serde_yml::from_str(&config)?))
    }

    pub fn create() -> Result<()> {
        fs::write("./retag.yml", include_str!("../template/config.yml"))?;
        fs::write("./script.lua", include_str!("../template/script.lua"))?;
        fs::write("./.luarc.json", include_str!("../template/.luarc.json"))?;
        fs::create_dir_all("./lua/api")?;
        fs::write("./lua/api/retag.lua", include_str!("../lua/api/retag.lua"))?;
        Ok(())
    }
}
