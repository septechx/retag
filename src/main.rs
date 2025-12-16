use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use audiotags::Tag;

use crate::{config::Config, lua::LuaEngine, trackdata::TrackData};

mod config;
mod lua;
mod trackdata;

fn main() -> Result<()> {
    let config = Config::load()?;
    if config.is_none() {
        Config::create()?;
        println!("Successfully initialized retag config");
        return Ok(());
    }
    let config = config.unwrap();

    let lua = LuaEngine::new()?;
    for script in config.scripts {
        lua.load_script(&script)?;
    }

    let files = find_files(&config.root)?;
    for file in files {
        let tag = Tag::new().read_from_path(&file)?;
        let data = TrackData::from(tag);
        lua.run_callbacks(&data)?;
    }

    Ok(())
}

fn find_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_files(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}
