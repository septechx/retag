use core::fmt;

use audiotags::AudioTag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum MimeType {
    Png,
    Jpeg,
    Tiff,
    Bmp,
    Gif,
}

impl From<MimeType> for audiotags::MimeType {
    fn from(mt: MimeType) -> Self {
        use audiotags::MimeType as MT;
        match mt {
            MimeType::Png => MT::Png,
            MimeType::Jpeg => MT::Jpeg,
            MimeType::Tiff => MT::Tiff,
            MimeType::Bmp => MT::Bmp,
            MimeType::Gif => MT::Gif,
        }
    }
}

impl From<audiotags::MimeType> for MimeType {
    fn from(mt: audiotags::MimeType) -> Self {
        use audiotags::MimeType as MT;
        match mt {
            MT::Png => MimeType::Png,
            MT::Jpeg => MimeType::Jpeg,
            MT::Tiff => MimeType::Tiff,
            MT::Bmp => MimeType::Bmp,
            MT::Gif => MimeType::Gif,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Picture {
    #[serde(with = "serde_bytes")]
    pub data: Box<[u8]>,
    pub mime: MimeType,
}

impl fmt::Debug for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Picture")
            .field("data", &"<binary>")
            .field("mime", &self.mime)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlbumData {
    pub title: String,
    pub artist: String,
    pub cover: Option<Picture>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackData {
    pub title: String,
    pub artist: String,
    pub album: AlbumData,
    pub year: i32,
    pub genre: String,
    pub comment: String,
    pub composer: String,
    pub album_artist: String,
    pub disc_number: u16,
    pub track_number: u16,
    pub duration: f64,
}

impl From<&Box<dyn AudioTag + Send + Sync>> for TrackData {
    fn from(tag: &Box<dyn AudioTag + Send + Sync>) -> Self {
        let album = tag.album();
        let (album_title, album_artist, cover) = if let Some(album) = album {
            (
                album.title.to_owned(),
                album.artist.unwrap_or_default().to_owned(),
                album.cover.map(|c| Picture {
                    data: c.data.into(),
                    mime: c.mime_type.into(),
                }),
            )
        } else {
            ("Unknown".to_owned(), String::new(), None)
        };

        Self {
            title: tag.title().unwrap_or_default().to_owned(),
            artist: tag.artist().unwrap_or_default().to_owned(),
            album: AlbumData {
                title: album_title,
                artist: album_artist,
                cover,
            },
            year: tag.year().unwrap_or_default(),
            genre: tag.genre().unwrap_or_default().to_owned(),
            comment: tag.comment().unwrap_or_default().to_owned(),
            composer: tag.composer().unwrap_or_default().to_owned(),
            album_artist: tag.album_artist().unwrap_or_default().to_owned(),
            disc_number: tag.disc_number().unwrap_or_default(),
            track_number: tag.track_number().unwrap_or_default(),
            duration: tag.duration().unwrap_or_default(),
        }
    }
}
