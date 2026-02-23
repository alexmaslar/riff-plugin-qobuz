use extism_pdk::*;

use crate::models::*;

const BASE_URL: &str = "https://qobuz.squid.wtf";

pub struct QobuzClient {
    country: String,
}

impl QobuzClient {
    pub fn new(country: &str) -> Self {
        Self {
            country: country.to_string(),
        }
    }

    fn get(&self, path: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{BASE_URL}{path}");
        let req = HttpRequest::new(&url)
            .with_method("GET")
            .with_header("Token-Country", &self.country);

        let resp = http::request::<()>(&req, None)?;
        let status = resp.status_code();
        if status < 200 || status >= 300 {
            return Err(Error::msg(format!(
                "qobuz API returned status {status} for {path}"
            )));
        }
        Ok(resp.body())
    }

    pub fn search(&self, query: &str, limit: u32) -> Result<QobuzSearchResponse, Error> {
        let encoded = urlencoding::encode(query);
        let path = format!("/api/get-music?q={encoded}&limit={limit}");
        let body = self.get(&path)?;
        serde_json::from_slice(&body)
            .map_err(|e| Error::msg(format!("parsing qobuz search: {e}")))
    }

    pub fn get_album(&self, album_id: &str) -> Result<QobuzAlbum, Error> {
        let encoded = urlencoding::encode(album_id);
        let path = format!("/api/get-album?album_id={encoded}");
        let body = self.get(&path)?;
        serde_json::from_slice(&body)
            .map_err(|e| Error::msg(format!("parsing qobuz album: {e}")))
    }

    pub fn get_artist(&self, artist_id: u64) -> Result<QobuzArtistResponse, Error> {
        let path = format!("/api/get-artist?artist_id={artist_id}");
        let body = self.get(&path)?;
        serde_json::from_slice(&body)
            .map_err(|e| Error::msg(format!("parsing qobuz artist: {e}")))
    }

    pub fn get_stream_url(&self, track_id: u64, quality: &str) -> Result<String, Error> {
        let path = format!("/api/download-music?track_id={track_id}&quality={quality}");
        let body = self.get(&path)?;
        let download: QobuzDownloadResponse = serde_json::from_slice(&body)
            .map_err(|e| Error::msg(format!("parsing qobuz download: {e}")))?;
        download
            .url
            .ok_or_else(|| Error::msg("no URL in qobuz download response"))
    }
}

/// Minimal percent-encoding for URL query parameters.
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
        result
    }
}
