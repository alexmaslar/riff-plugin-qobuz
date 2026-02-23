use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingQuality {
    Low,
    High,
    Lossless,
    HiRes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingArtist {
    pub provider_id: String,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingAlbum {
    pub provider_id: String,
    pub title: String,
    pub artist: StreamingArtist,
    pub year: Option<i32>,
    pub cover_url: Option<String>,
    pub track_count: u32,
    pub available_qualities: Vec<StreamingQuality>,
    pub album_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingTrack {
    pub provider_id: String,
    pub title: String,
    pub artist_name: String,
    pub album_title: String,
    pub track_number: u32,
    pub disc_number: u32,
    pub duration_secs: u32,
    pub available_qualities: Vec<StreamingQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingAlbumDetail {
    pub album: StreamingAlbum,
    pub tracks: Vec<StreamingTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSearchResults {
    pub albums: Vec<StreamingAlbum>,
    pub artists: Vec<StreamingArtist>,
    pub tracks: Vec<StreamingTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUrl {
    pub url: String,
    pub mime_type: String,
    pub quality: StreamingQuality,
}
