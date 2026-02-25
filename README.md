# Riff Plugin: Qobuz

Streaming plugin for [Riff](https://github.com/alexmaslar/riff) that provides hi-res audio from Qobuz via the SquidWTF proxy.

## Features

- Search tracks, albums, and artists on Qobuz
- Stream up to Hi-Res 192kHz FLAC quality
- Browse artist discographies
- Configurable country code for regional catalogs

## Settings

| Key | Default | Description |
|-----|---------|-------------|
| `quality` | `6` (Lossless) | `27` = Hi-Res 192kHz, `7` = Hi-Res 96kHz, `6` = Lossless 44.1kHz, `5` = High MP3 320kbps |
| `country` | `US` | Two-letter country code for catalog availability |

## Building

```bash
rustup target add wasm32-unknown-unknown  # first time only
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/riff_plugin_qobuz.wasm plugin.wasm
```

## Development

Add a dev plugin entry to your Riff `config.yaml`:

```yaml
dev_plugins:
  - path: /path/to/riff-plugin-qobuz
```

Rebuild and reload without restarting the server:

```bash
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/riff_plugin_qobuz.wasm plugin.wasm

curl -X POST -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/plugins/qobuz/reload
```

## Exported Functions

| Function | Input | Output |
|----------|-------|--------|
| `riff_search` | `{ query, limit }` | `StreamingSearchResults` (tracks, albums, artists) |
| `riff_get_album` | `{ id }` | `StreamingAlbumDetail` (album + tracks) |
| `riff_get_artist_albums` | `{ id }` | `Vec<StreamingAlbum>` |
| `riff_get_stream_url` | `{ id, quality }` | `StreamUrl` (url, mime_type, quality) |
| `riff_health_check` | `""` | `"healthy"` |

## How It Works

The plugin proxies Qobuz API requests through [SquidWTF](https://squid.wtf), passing a `Token-Country` header for regional catalog access. Quality levels map to Qobuz's numeric format IDs — levels 6, 7, and 27 return FLAC, while level 5 returns MP3.

## License

MIT
