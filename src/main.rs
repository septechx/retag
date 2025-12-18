use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use audiotags::{Album, Tag};

use crate::{config::Config, lua::LuaEngine};

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
        let mut tag = Tag::new().read_from_path(&file)?;
        let new_data = lua.run_callbacks((&tag).into())?;

        if let Some(new_data) = new_data {
            println!("retagging {}", file.display());

            tag.set_title(&new_data.title);
            tag.set_artist(&new_data.artist);
            tag.set_year(new_data.year);
            tag.set_genre(&new_data.genre);
            tag.set_comment(new_data.comment);
            tag.set_composer(new_data.composer);
            tag.set_album_artist(&new_data.album_artist);
            tag.set_disc_number(new_data.disc_number);
            tag.set_track_number(new_data.track_number);

            tag.set_album(Album {
                title: &new_data.album.title,
                artist: Some(&new_data.album.artist),
                cover: new_data.album.cover.as_ref().map(|c| audiotags::Picture {
                    data: c.data.as_ref(),
                    mime_type: c.mime.into(),
                }),
            });

            tag.write_to_path(file.to_str().unwrap())?;
        }
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
