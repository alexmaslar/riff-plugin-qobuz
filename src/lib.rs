use extism_pdk::*;
use serde::Deserialize;

mod client;
mod models;
mod types;

use client::QobuzClient;
use types::*;

#[derive(Deserialize)]
struct SearchInput {
    query: String,
    limit: u32,
}

#[derive(Deserialize)]
struct IdInput {
    id: String,
}

#[derive(Deserialize)]
struct StreamUrlInput {
    id: String,
    quality: StreamingQuality,
}

fn get_or_init_client() -> Result<QobuzClient, Error> {
    let country = config::get("country")?.unwrap_or_else(|| "US".to_string());
    Ok(QobuzClient::new(&country))
}

#[plugin_fn]
pub fn riff_search(Json(input): Json<SearchInput>) -> FnResult<Json<StreamingSearchResults>> {
    let client = get_or_init_client()?;
    let resp = client.search(&input.query, input.limit)?;

    Ok(Json(StreamingSearchResults {
        albums: resp
            .albums
            .unwrap_or_default()
            .items
            .iter()
            .map(convert_album)
            .collect(),
        tracks: resp
            .tracks
            .unwrap_or_default()
            .items
            .iter()
            .map(convert_track)
            .collect(),
        artists: resp
            .artists
            .unwrap_or_default()
            .items
            .iter()
            .map(convert_artist)
            .collect(),
    }))
}

#[plugin_fn]
pub fn riff_get_album(Json(input): Json<IdInput>) -> FnResult<Json<StreamingAlbumDetail>> {
    let client = get_or_init_client()?;
    let album_data = client.get_album(&input.id)?;

    let album = convert_album(&album_data);
    let tracks: Vec<StreamingTrack> = album_data
        .tracks
        .unwrap_or_default()
        .items
        .iter()
        .map(convert_track)
        .collect();

    Ok(Json(StreamingAlbumDetail { album, tracks }))
}

#[plugin_fn]
pub fn riff_get_artist_albums(Json(input): Json<IdInput>) -> FnResult<Json<Vec<StreamingAlbum>>> {
    let client = get_or_init_client()?;
    let id: u64 = input
        .id
        .parse()
        .map_err(|e| Error::msg(format!("invalid artist id: {e}")))?;
    let resp = client.get_artist(id)?;
    let albums: Vec<StreamingAlbum> = resp
        .albums
        .unwrap_or_default()
        .items
        .iter()
        .map(convert_album)
        .collect();
    Ok(Json(albums))
}

#[plugin_fn]
pub fn riff_get_stream_url(Json(input): Json<StreamUrlInput>) -> FnResult<Json<StreamUrl>> {
    let client = get_or_init_client()?;
    let id: u64 = input
        .id
        .parse()
        .map_err(|e| Error::msg(format!("invalid track id: {e}")))?;
    let qobuz_quality = quality_to_qobuz(input.quality);
    let url = client.get_stream_url(id, qobuz_quality)?;

    // Qobuz returns FLAC for quality 6/7/27, MP3 for quality 5
    let mime_type = if qobuz_quality == "5" {
        "audio/mpeg"
    } else {
        "audio/flac"
    };

    Ok(Json(StreamUrl {
        url,
        mime_type: mime_type.to_string(),
        quality: input.quality,
    }))
}

#[plugin_fn]
pub fn riff_health_check(_input: String) -> FnResult<String> {
    let client = get_or_init_client()?;
    client.search("test", 1)?;
    Ok("healthy".to_string())
}

// --- Converter functions ---

fn quality_to_qobuz(q: StreamingQuality) -> &'static str {
    match q {
        StreamingQuality::HiRes => "27",
        StreamingQuality::Lossless => "6",
        StreamingQuality::High => "5",
        StreamingQuality::Low => "5",
    }
}

fn all_qualities() -> Vec<StreamingQuality> {
    vec![
        StreamingQuality::HiRes,
        StreamingQuality::Lossless,
        StreamingQuality::High,
    ]
}

fn convert_artist(a: &models::QobuzArtist) -> StreamingArtist {
    StreamingArtist {
        provider_id: a.id.map(|id| id.to_string()).unwrap_or_default(),
        name: a.name.0.clone(),
        image_url: a
            .image
            .as_ref()
            .and_then(|img| img.large.clone().or_else(|| img.small.clone())),
    }
}

fn convert_album(a: &models::QobuzAlbum) -> StreamingAlbum {
    let year = a.release_date_original.as_ref().and_then(|d| {
        d.split('-')
            .next()
            .and_then(|y| y.parse::<i32>().ok())
    });
    let artist = a
        .artist
        .as_ref()
        .map(convert_artist)
        .unwrap_or_else(|| StreamingArtist {
            provider_id: String::new(),
            name: "Unknown Artist".to_string(),
            image_url: None,
        });
    StreamingAlbum {
        provider_id: a.id.0.clone(),
        title: a.title.clone(),
        artist,
        year,
        cover_url: a
            .image
            .as_ref()
            .and_then(|img| img.large.clone().or_else(|| img.small.clone())),
        track_count: a.tracks_count.unwrap_or(0),
        available_qualities: all_qualities(),
        album_type: None,
    }
}

fn convert_track(t: &models::QobuzTrack) -> StreamingTrack {
    StreamingTrack {
        provider_id: t.id.to_string(),
        title: t.title.clone(),
        artist_name: t
            .performer
            .as_ref()
            .map(|p| p.name.0.clone())
            .unwrap_or_else(|| "Unknown Artist".to_string()),
        album_title: t
            .album
            .as_ref()
            .and_then(|a| a.title.clone())
            .unwrap_or_default(),
        track_number: t.track_number.unwrap_or(1),
        disc_number: 1,
        duration_secs: t.duration.unwrap_or(0),
        available_qualities: all_qualities(),
    }
}
