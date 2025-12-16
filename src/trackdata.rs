use core::fmt;

use audiotags::{Album, AudioTag};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum MimeType {
    Png,
    Jpeg,
    Tiff,
    Bmp,
    Gif,
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

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct Picture {
    #[serde(with = "serde_bytes")]
    data: Box<[u8]>,
    mime: MimeType,
}

impl fmt::Debug for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Picture")
            .field("data", &"<binary>")
            .field("mime", &self.mime)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct AlbumData {
    title: String,
    artist: String,
    cover: Option<Picture>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackData {
    title: String,
    artist: String,
    album: AlbumData,
    year: i32,
    genre: String,
    comment: String,
    composer: String,
    album_artist: String,
    disc_number: u16,
    track_number: u16,
    duration: f64,
}

impl From<Box<dyn AudioTag + Send + Sync>> for TrackData {
    fn from(tag: Box<dyn AudioTag + Send + Sync>) -> Self {
        Self {
            title: tag.title().unwrap_or_default().to_owned(),
            artist: tag.artist().unwrap_or_default().to_owned(),
            album: AlbumData {
                title: tag
                    .album()
                    .unwrap_or_else(|| Album::with_title("Unknown"))
                    .title
                    .to_owned(),
                artist: tag
                    .album()
                    .unwrap_or_else(|| Album::with_title("Unknown"))
                    .artist
                    .unwrap_or_default()
                    .to_owned(),
                cover: tag
                    .album()
                    .unwrap_or_else(|| Album::with_title("Unknown"))
                    .cover
                    .map(|c| Picture {
                        data: c.data.into(),
                        mime: c.mime_type.into(),
                    }),
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
