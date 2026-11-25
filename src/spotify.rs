use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use url::{Url, form_urlencoded};

pub const DEFAULT_CLIENT_ID: &str = "91edaf1b1e8647169b426bd99d1ac9c2";
pub const REDIRECT_URI: &str = "http://127.0.0.1:8888/callback";
const SCOPES: &str = "user-read-private user-library-read user-library-modify playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    #[serde(default = "default_client_id")]
    pub client_id: String,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: u64,
    #[serde(default)]
    pub soloist_api_key: String,
    #[serde(default)]
    pub soloist_bin: String,
}

fn default_client_id() -> String {
    DEFAULT_CLIENT_ID.to_string()
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            client_id: default_client_id(),
            access_token: None,
            refresh_token: None,
            expires_at: 0,
            soloist_api_key: String::new(),
            soloist_bin: String::new(),
        }
    }
}

impl LocalConfig {
    pub fn path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from))
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .or_else(|| std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("spotirust").join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::path();
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let body = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, body).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
        Ok(())
    }

    pub fn is_logged_in(&self) -> bool {
        self.access_token.is_some() || self.refresh_token.is_some()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub is_active: bool,
    pub volume_percent: u8,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemoteTrack {
    pub spotify_id: String,
    pub uri: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u32,
    pub explicit: bool,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemotePlaylist {
    pub spotify_id: String,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemotePlayer {
    pub is_playing: bool,
    pub position_secs: f32,
    pub shuffle: bool,
    pub repeat: String,
    pub volume: f32,
    pub device_name: Option<String>,
    pub track: Option<RemoteTrack>,
}

pub struct SoloistManager {
    child: Option<Child>,
}

impl Default for SoloistManager {
    fn default() -> Self {
        Self { child: None }
    }
}

impl Drop for SoloistManager {
    fn drop(&mut self) {
        self.stop();
    }
}

impl SoloistManager {
    pub fn find_binary(configured: &str) -> Option<PathBuf> {
        let bin_name = if cfg!(target_os = "windows") {
            "soloist.exe"
        } else {
            "soloist"
        };
        let trimmed = configured.trim();
        if !trimmed.is_empty() {
            let p = PathBuf::from(trimmed);
            if p.is_file() {
                return Some(p);
            }
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let beside = dir.join(bin_name);
                if beside.is_file() {
                    return Some(beside);
                }
            }
        }
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let local = PathBuf::from(home).join(".local/bin").join(bin_name);
            if local.is_file() {
                return Some(local);
            }
        }
        if let Some(local_app) = std::env::var_os("LOCALAPPDATA") {
            let p = PathBuf::from(local_app).join("soloist").join(bin_name);
            if p.is_file() {
                return Some(p);
            }
        }
        let usr_local = PathBuf::from("/usr/local/bin").join(bin_name);
        if usr_local.is_file() {
            return Some(usr_local);
        }
        if let Some(paths) = std::env::var_os("PATH") {
            for dir in std::env::split_paths(&paths) {
                let candidate = dir.join(bin_name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
        None
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(child) = self.child.as_mut() {
            match child.try_wait() {
                Ok(None) => true,
                _ => {
                    self.child = None;
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn start(&mut self, api_key: &str, configured_bin: &str) -> Result<(), String> {
        if self.is_running() {
            return Ok(());
        }
        let key = api_key.trim();
        if key.is_empty() {
            return Err("Soloist API Key belum diisi.".into());
        }
        let bin = Self::find_binary(configured_bin).ok_or_else(|| {
            if cfg!(target_os = "windows") {
                "Binary `soloist.exe` belum ditemukan. Taruh di folder yang sama dengan spotirust.exe atau isi path-nya di Settings.".to_string()
            } else {
                "Binary `soloist` belum ditemukan. Taruh di ~/.local/bin/soloist atau /usr/local/bin/soloist.".to_string()
            }
        })?;
        let mut cmd = Command::new(bin);
        cmd.args(["--device-name", "SpotiRust", "--api-key", key])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
        let child = cmd
            .spawn()
            .map_err(|e| format!("Gagal menjalankan soloist: {e}"))?;
        self.child = Some(child);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub struct SpotifyClient {
    http: reqwest::Client,
}

impl Default for SpotifyClient {
    fn default() -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }
}

fn open_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let res = Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", url])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    #[cfg(target_os = "macos")]
    let res = Command::new("open")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    #[cfg(all(unix, not(target_os = "macos")))]
    let res = Command::new("xdg-open")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    res.map(|_| ())
        .map_err(|e| format!("Gagal membuka browser untuk login Spotify: {e}"))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn random_urlsafe(byte_len: usize) -> Result<String, String> {
    let mut buf = vec![0u8; byte_len];
    getrandom::fill(&mut buf).map_err(|e| format!("RNG error: {e}"))?;
    Ok(URL_SAFE_NO_PAD.encode(buf))
}

fn parse_track(val: &Value) -> Option<RemoteTrack> {
    let id = val.get("id")?.as_str()?.to_string();
    let title = val.get("name")?.as_str()?.to_string();
    let uri = val
        .get("uri")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| format!("spotify:track:{id}"));
    let artist = val
        .get("artists")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|a| a.get("name").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown Artist".into());
    let album_obj = val.get("album");
    let album = album_obj
        .and_then(|a| a.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("Single")
        .to_string();
    let image_url = album_obj
        .and_then(|a| a.get("images"))
        .and_then(Value::as_array)
        .and_then(|imgs| imgs.get(1).or_else(|| imgs.first()))
        .and_then(|img| img.get("url"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let duration_secs = (val.get("duration_ms").and_then(Value::as_u64).unwrap_or(0) / 1000) as u32;
    let explicit = val.get("explicit").and_then(Value::as_bool).unwrap_or(false);

    Some(RemoteTrack {
        spotify_id: id,
        uri,
        title,
        artist,
        album,
        duration_secs: duration_secs.max(1),
        explicit,
        image_url,
    })
}

impl SpotifyClient {
    pub async fn login_pkce(&self, cfg: &mut LocalConfig) -> Result<(), String> {
        let client_id = cfg.client_id.trim().to_string();
        if client_id.is_empty() {
            return Err("Client ID tidak boleh kosong.".into());
        }

        let verifier = random_urlsafe(48)?;
        let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        let state = random_urlsafe(16)?;

        let listener = TcpListener::bind("127.0.0.1:8888")
            .await
            .map_err(|e| format!("Port 8888 sedang dipakai ({e}). Tutup proses lain di port 8888 lalu coba lagi."))?;

        let mut auth_url = Url::parse("https://accounts.spotify.com/authorize").unwrap();
        auth_url
            .query_pairs_mut()
            .append_pair("client_id", &client_id)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", REDIRECT_URI)
            .append_pair("code_challenge_method", "S256")
            .append_pair("code_challenge", &challenge)
            .append_pair("state", &state)
            .append_pair("scope", SCOPES)
            .append_pair("show_dialog", "true");

        open_browser(auth_url.as_str())?;

        let (mut stream, _) = tokio::time::timeout(Duration::from_secs(180), listener.accept())
            .await
            .map_err(|_| "Timeout menunggu login Spotify di browser (3 menit).".to_string())?
            .map_err(|e| format!("Callback socket error: {e}"))?;

        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await.map_err(|e| e.to_string())?;
        let req_str = String::from_utf8_lossy(&buf[..n]);
        let first_line = req_str.lines().next().unwrap_or("");
        let path = first_line.split_whitespace().nth(1).unwrap_or("/");
        let parsed = Url::parse(&format!("http://127.0.0.1:8888{path}"))
            .map_err(|_| "Invalid OAuth callback URL".to_string())?;

        let mut code = None;
        let mut recv_state = None;
        let mut oauth_err = None;
        for (k, v) in parsed.query_pairs() {
            match k.as_ref() {
                "code" => code = Some(v.into_owned()),
                "state" => recv_state = Some(v.into_owned()),
                "error" => oauth_err = Some(v.into_owned()),
                _ => {}
            }
        }

        let (status_line, body_html) = if oauth_err.is_some() || code.is_none() || recv_state.as_deref() != Some(&state) {
            (
                "HTTP/1.1 400 Bad Request",
                "<html><body style='background:#121212;color:#fff;font-family:sans-serif;display:grid;place-items:center;height:100vh'><div><h2>Login Spotify dibatalkan / gagal</h2><p>Silakan tutup tab ini dan coba lagi di SpotiRust.</p></div></body></html>",
            )
        } else {
            (
                "HTTP/1.1 200 OK",
                "<html><body style='background:#121212;color:#fff;font-family:sans-serif;display:grid;place-items:center;height:100vh'><div style='text-align:center'><h2 style='color:#1db954'>Terhubung ke SpotiRust!</h2><p>Kamu bisa tutup tab ini dan kembali ke aplikasi SpotiRust.</p></div></body></html>",
            )
        };
        let resp = format!(
            "{status_line}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body_html}",
            body_html.len()
        );
        let _ = stream.write_all(resp.as_bytes()).await;

        if let Some(err) = oauth_err {
            return Err(format!("Spotify OAuth error: {err}"));
        }
        if recv_state.as_deref() != Some(&state) {
            return Err("OAuth state mismatch.".into());
        }
        let code = code.ok_or_else(|| "Authorization code tidak ditemukan.".to_string())?;

        let form_body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &client_id)
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", &code)
            .append_pair("redirect_uri", REDIRECT_URI)
            .append_pair("code_verifier", &verifier)
            .finish();

        self.exchange_token(cfg, form_body).await
    }

    pub async fn ensure_token(&self, cfg: &mut LocalConfig) -> Result<String, String> {
        if let Some(tok) = cfg.access_token.clone() {
            if now_secs() + 60 < cfg.expires_at {
                return Ok(tok);
            }
        }
        let refresh = cfg
            .refresh_token
            .clone()
            .ok_or_else(|| "Belum login ke Spotify.".to_string())?;
        let form_body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", cfg.client_id.trim())
            .append_pair("grant_type", "refresh_token")
            .append_pair("refresh_token", &refresh)
            .finish();
        self.exchange_token(cfg, form_body).await?;
        cfg.access_token
            .clone()
            .ok_or_else(|| "Gagal me-refresh access token.".to_string())
    }

    async fn exchange_token(&self, cfg: &mut LocalConfig, form_body: String) -> Result<(), String> {
        let res = self
            .http
            .post("https://accounts.spotify.com/api/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .map_err(|e| format!("Gagal menghubungi Spotify Accounts: {e}"))?;

        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(format!("Token exchange gagal ({status}): {text}"));
        }
        let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let access = json
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or_else(|| "Response token tidak mengandung access_token.".to_string())?
            .to_string();
        let expires_in = json.get("expires_in").and_then(Value::as_u64).unwrap_or(3600);
        if let Some(refresh) = json.get("refresh_token").and_then(Value::as_str) {
            cfg.refresh_token = Some(refresh.to_string());
        }
        cfg.access_token = Some(access);
        cfg.expires_at = now_secs() + expires_in;
        cfg.save()?;
        Ok(())
    }

    async fn api_request(
        &self,
        token: &str,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<(u16, Value), String> {
        let mut req = self.http.request(method.clone(), url).bearer_auth(token);
        if let Some(b) = body {
            req = req.json(&b);
        } else if matches!(method, Method::POST | Method::PUT | Method::DELETE) {
            req = req.header("Content-Length", "0");
        }
        let res = req.send().await.map_err(|e| format!("Network error: {e}"))?;
        let status = res.status().as_u16();
        let text = res.text().await.unwrap_or_default();
        if text.trim().is_empty() {
            return Ok((status, Value::Null));
        }
        let parsed = serde_json::from_str(&text).unwrap_or_else(|_| json!({ "raw": text }));
        Ok((status, parsed))
    }

    pub async fn fetch_profile(&self, token: &str) -> Result<UserProfile, String> {
        let (status, val) = self
            .api_request(token, Method::GET, "https://api.spotify.com/v1/me", None)
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let display_name = val
            .get("display_name")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .or_else(|| val.get("id").and_then(Value::as_str))
            .unwrap_or("Spotify User")
            .to_string();
        let avatar_url = val
            .get("images")
            .and_then(Value::as_array)
            .and_then(|arr| arr.first())
            .and_then(|img| img.get("url"))
            .and_then(Value::as_str)
            .map(str::to_string);
        Ok(UserProfile {
            display_name,
            avatar_url,
        })
    }

    pub async fn fetch_playlists(&self, token: &str) -> Result<Vec<RemotePlaylist>, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::GET,
                "https://api.spotify.com/v1/me/playlists?limit=50",
                None,
            )
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let mut out = Vec::new();
        if let Some(items) = val.get("items").and_then(Value::as_array) {
            for p in items {
                let Some(id) = p.get("id").and_then(Value::as_str) else {
                    continue;
                };
                let name = p
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("Untitled Playlist")
                    .to_string();
                let description = p
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                let creator = p
                    .get("owner")
                    .and_then(|o| o.get("display_name").or_else(|| o.get("id")))
                    .and_then(Value::as_str)
                    .unwrap_or("Spotify")
                    .to_string();
                let image_url = p
                    .get("images")
                    .and_then(Value::as_array)
                    .and_then(|imgs| imgs.first())
                    .and_then(|img| img.get("url"))
                    .and_then(Value::as_str)
                    .map(str::to_string);
                out.push(RemotePlaylist {
                    spotify_id: id.to_string(),
                    name,
                    description,
                    creator,
                    image_url,
                });
            }
        }
        Ok(out)
    }

    pub async fn fetch_playlist_tracks(
        &self,
        token: &str,
        playlist_id: &str,
    ) -> Result<Vec<RemoteTrack>, String> {
        let items_url = format!("https://api.spotify.com/v1/playlists/{playlist_id}/items?limit=50");
        let (mut status, mut val) = self
            .api_request(token, Method::GET, &items_url, None)
            .await?;
        if status == 404 || status == 400 {
            let legacy_url =
                format!("https://api.spotify.com/v1/playlists/{playlist_id}/tracks?limit=50");
            (status, val) = self
                .api_request(token, Method::GET, &legacy_url, None)
                .await?;
        }
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let mut tracks = Vec::new();
        if let Some(items) = val.get("items").and_then(Value::as_array) {
            for entry in items {
                let inner = entry.get("item").or_else(|| entry.get("track")).unwrap_or(entry);
                if let Some(t) = parse_track(inner) {
                    tracks.push(t);
                }
            }
        }
        Ok(tracks)
    }

    pub async fn fetch_liked_tracks(&self, token: &str) -> Result<Vec<RemoteTrack>, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::GET,
                "https://api.spotify.com/v1/me/tracks?limit=50",
                None,
            )
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let mut tracks = Vec::new();
        if let Some(items) = val.get("items").and_then(Value::as_array) {
            for entry in items {
                let inner = entry.get("track").or_else(|| entry.get("item")).unwrap_or(entry);
                if let Some(t) = parse_track(inner) {
                    tracks.push(t);
                }
            }
        }
        Ok(tracks)
    }

    pub async fn fetch_recently_played(&self, token: &str) -> Result<Vec<RemoteTrack>, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::GET,
                "https://api.spotify.com/v1/me/player/recently-played?limit=20",
                None,
            )
            .await?;
        if status >= 400 {
            return Ok(Vec::new());
        }
        let mut tracks = Vec::new();
        if let Some(items) = val.get("items").and_then(Value::as_array) {
            for entry in items {
                let inner = entry.get("track").unwrap_or(entry);
                if let Some(t) = parse_track(inner) {
                    tracks.push(t);
                }
            }
        }
        Ok(tracks)
    }

    pub async fn search_tracks(&self, token: &str, query: &str) -> Result<Vec<RemoteTrack>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }
        let mut url = Url::parse("https://api.spotify.com/v1/search").unwrap();
        url.query_pairs_mut()
            .append_pair("q", q)
            .append_pair("type", "track")
            .append_pair("limit", "10");
        let (status, val) = self
            .api_request(token, Method::GET, url.as_str(), None)
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let mut tracks = Vec::new();
        if let Some(items) = val
            .get("tracks")
            .and_then(|t| t.get("items"))
            .and_then(Value::as_array)
        {
            for item in items {
                if let Some(t) = parse_track(item) {
                    tracks.push(t);
                }
            }
        }
        Ok(tracks)
    }

    pub async fn fetch_player(&self, token: &str) -> Result<Option<RemotePlayer>, String> {
        let (status, val) = self
            .api_request(token, Method::GET, "https://api.spotify.com/v1/me/player", None)
            .await?;
        if status == 204 {
            return Ok(None);
        }
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let is_playing = val.get("is_playing").and_then(Value::as_bool).unwrap_or(false);
        let position_secs =
            val.get("progress_ms").and_then(Value::as_u64).unwrap_or(0) as f32 / 1000.0;
        let shuffle = val
            .get("shuffle_state")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let repeat = val
            .get("repeat_state")
            .and_then(Value::as_str)
            .unwrap_or("off")
            .to_string();
        let device = val.get("device");
        let volume = device
            .and_then(|d| d.get("volume_percent"))
            .and_then(Value::as_u64)
            .unwrap_or(72) as f32
            / 100.0;
        let device_name = device
            .and_then(|d| d.get("name"))
            .and_then(Value::as_str)
            .map(str::to_string);
        let track = val.get("item").and_then(parse_track);

        Ok(Some(RemotePlayer {
            is_playing,
            position_secs,
            shuffle,
            repeat,
            volume,
            device_name,
            track,
        }))
    }

    pub async fn fetch_queue(&self, token: &str) -> Result<Vec<RemoteTrack>, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::GET,
                "https://api.spotify.com/v1/me/player/queue",
                None,
            )
            .await?;
        if status >= 400 {
            return Ok(Vec::new());
        }
        let mut tracks = Vec::new();
        if let Some(items) = val.get("queue").and_then(Value::as_array) {
            for item in items.iter().take(20) {
                if let Some(t) = parse_track(item) {
                    tracks.push(t);
                }
            }
        }
        Ok(tracks)
    }

    pub async fn fetch_devices(&self, token: &str) -> Result<Vec<DeviceInfo>, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::GET,
                "https://api.spotify.com/v1/me/player/devices",
                None,
            )
            .await?;
        if status >= 400 {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        if let Some(arr) = val.get("devices").and_then(Value::as_array) {
            for d in arr {
                let Some(id) = d.get("id").and_then(Value::as_str) else {
                    continue;
                };
                out.push(DeviceInfo {
                    id: id.to_string(),
                    name: d
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("Spotify Device")
                        .to_string(),
                    device_type: d
                        .get("type")
                        .and_then(Value::as_str)
                        .unwrap_or("Computer")
                        .to_string(),
                    is_active: d.get("is_active").and_then(Value::as_bool).unwrap_or(false),
                    volume_percent: d
                        .get("volume_percent")
                        .and_then(Value::as_u64)
                        .unwrap_or(70) as u8,
                });
            }
        }
        Ok(out)
    }

    pub async fn pick_preferred_device(&self, token: &str) -> Option<String> {
        let devices = self.fetch_devices(token).await.ok()?;
        devices
            .iter()
            .find(|d| d.is_active)
            .or_else(|| devices.iter().find(|d| d.name.eq_ignore_ascii_case("SpotiRust")))
            .or_else(|| devices.first())
            .map(|d| d.id.clone())
    }

    pub async fn play_uris(
        &self,
        token: &str,
        uris: Vec<String>,
        offset_uri: Option<String>,
    ) -> Result<(), String> {
        let mut body = json!({ "uris": uris });
        if let Some(uri) = offset_uri {
            body["offset"] = json!({ "uri": uri });
        }
        let (status, val) = self
            .api_request(
                token,
                Method::PUT,
                "https://api.spotify.com/v1/me/player/play",
                Some(body.clone()),
            )
            .await?;
        if status == 404 {
            if let Some(dev_id) = self.pick_preferred_device(token).await {
                let url = format!(
                    "https://api.spotify.com/v1/me/player/play?device_id={}",
                    url::form_urlencoded::byte_serialize(dev_id.as_bytes()).collect::<String>()
                );
                let (s2, v2) = self
                    .api_request(token, Method::PUT, &url, Some(body))
                    .await?;
                if s2 >= 400 {
                    return Err(extract_api_error(s2, &v2));
                }
                return Ok(());
            }
            return Err("Tidak ada device Spotify aktif. Jalankan Soloist atau buka Spotify di HP/Web dulu.".into());
        }
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn resume(&self, token: &str) -> Result<(), String> {
        let (status, val) = self
            .api_request(token, Method::PUT, "https://api.spotify.com/v1/me/player/play", None)
            .await?;
        if status == 404 {
            if let Some(dev_id) = self.pick_preferred_device(token).await {
                let url = format!("https://api.spotify.com/v1/me/player/play?device_id={dev_id}");
                let (s2, v2) = self.api_request(token, Method::PUT, &url, None).await?;
                if s2 >= 400 {
                    return Err(extract_api_error(s2, &v2));
                }
                return Ok(());
            }
            return Err("Tidak ada device Spotify aktif.".into());
        }
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn pause(&self, token: &str) -> Result<(), String> {
        let (status, val) = self
            .api_request(token, Method::PUT, "https://api.spotify.com/v1/me/player/pause", None)
            .await?;
        if status >= 400 && status != 403 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn next(&self, token: &str) -> Result<(), String> {
        let (status, val) = self
            .api_request(token, Method::POST, "https://api.spotify.com/v1/me/player/next", None)
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn previous(&self, token: &str) -> Result<(), String> {
        let (status, val) = self
            .api_request(
                token,
                Method::POST,
                "https://api.spotify.com/v1/me/player/previous",
                None,
            )
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn seek(&self, token: &str, position_ms: u64) -> Result<(), String> {
        let url = format!("https://api.spotify.com/v1/me/player/seek?position_ms={position_ms}");
        let (status, val) = self.api_request(token, Method::PUT, &url, None).await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn set_volume(&self, token: &str, percent: u8) -> Result<(), String> {
        let pct = percent.min(100);
        let url = format!("https://api.spotify.com/v1/me/player/volume?volume_percent={pct}");
        let (status, val) = self.api_request(token, Method::PUT, &url, None).await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn set_shuffle(&self, token: &str, state: bool) -> Result<(), String> {
        let url = format!("https://api.spotify.com/v1/me/player/shuffle?state={state}");
        let (status, val) = self.api_request(token, Method::PUT, &url, None).await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn set_repeat(&self, token: &str, state: &str) -> Result<(), String> {
        let url = format!("https://api.spotify.com/v1/me/player/repeat?state={state}");
        let (status, val) = self.api_request(token, Method::PUT, &url, None).await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn add_to_queue(&self, token: &str, uri: &str) -> Result<(), String> {
        let mut url = Url::parse("https://api.spotify.com/v1/me/player/queue").unwrap();
        url.query_pairs_mut().append_pair("uri", uri);
        let (status, val) = self
            .api_request(token, Method::POST, url.as_str(), None)
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn transfer_playback(&self, token: &str, device_id: &str) -> Result<(), String> {
        let (status, val) = self
            .api_request(
                token,
                Method::PUT,
                "https://api.spotify.com/v1/me/player",
                Some(json!({ "device_ids": [device_id], "play": true })),
            )
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn set_track_liked(
        &self,
        token: &str,
        uri: &str,
        spotify_id: &str,
        liked: bool,
    ) -> Result<(), String> {
        let method = if liked { Method::PUT } else { Method::DELETE };
        let mut lib_url = Url::parse("https://api.spotify.com/v1/me/library").unwrap();
        lib_url.query_pairs_mut().append_pair("uris", uri);
        let (status, _) = self
            .api_request(token, method.clone(), lib_url.as_str(), None)
            .await?;
        if status < 400 {
            return Ok(());
        }
        let legacy = format!("https://api.spotify.com/v1/me/tracks?ids={spotify_id}");
        let (s2, v2) = self.api_request(token, method, &legacy, None).await?;
        if s2 >= 400 {
            return Err(extract_api_error(s2, &v2));
        }
        Ok(())
    }

    pub async fn create_playlist(&self, token: &str, name: &str) -> Result<RemotePlaylist, String> {
        let (status, val) = self
            .api_request(
                token,
                Method::POST,
                "https://api.spotify.com/v1/me/playlists",
                Some(json!({
                    "name": name,
                    "description": "Created with SpotiRust",
                    "public": false
                })),
            )
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        let id = val
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| "Playlist ID tidak ditemukan.".to_string())?
            .to_string();
        Ok(RemotePlaylist {
            spotify_id: id,
            name: val
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or(name)
                .to_string(),
            description: "Created with SpotiRust".into(),
            creator: "You".into(),
            image_url: None,
        })
    }

    pub async fn rename_playlist(
        &self,
        token: &str,
        playlist_id: &str,
        name: &str,
    ) -> Result<(), String> {
        let url = format!("https://api.spotify.com/v1/playlists/{playlist_id}");
        let (status, val) = self
            .api_request(token, Method::PUT, &url, Some(json!({ "name": name })))
            .await?;
        if status >= 400 {
            return Err(extract_api_error(status, &val));
        }
        Ok(())
    }

    pub async fn add_track_to_playlist(
        &self,
        token: &str,
        playlist_id: &str,
        uri: &str,
    ) -> Result<(), String> {
        let body = json!({ "uris": [uri] });
        let url = format!("https://api.spotify.com/v1/playlists/{playlist_id}/items");
        let (status, _) = self
            .api_request(token, Method::POST, &url, Some(body.clone()))
            .await?;
        if status < 400 {
            return Ok(());
        }
        let legacy = format!("https://api.spotify.com/v1/playlists/{playlist_id}/tracks");
        let (s2, v2) = self
            .api_request(token, Method::POST, &legacy, Some(body))
            .await?;
        if s2 >= 400 {
            return Err(extract_api_error(s2, &v2));
        }
        Ok(())
    }

    pub async fn remove_track_from_playlist(
        &self,
        token: &str,
        playlist_id: &str,
        uri: &str,
    ) -> Result<(), String> {
        let body = json!({ "tracks": [{ "uri": uri }] });
        let url = format!("https://api.spotify.com/v1/playlists/{playlist_id}/items");
        let (status, _) = self
            .api_request(token, Method::DELETE, &url, Some(body.clone()))
            .await?;
        if status < 400 {
            return Ok(());
        }
        let legacy = format!("https://api.spotify.com/v1/playlists/{playlist_id}/tracks");
        let (s2, v2) = self
            .api_request(token, Method::DELETE, &legacy, Some(body))
            .await?;
        if s2 >= 400 {
            return Err(extract_api_error(s2, &v2));
        }
        Ok(())
    }
}

fn extract_api_error(status: u16, val: &Value) -> String {
    if let Some(msg) = val
        .get("error")
        .and_then(|e| e.get("message"))
        .and_then(Value::as_str)
    {
        return format!("Spotify ({status}): {msg}");
    }
    if let Some(raw) = val.get("raw").and_then(Value::as_str) {
        if raw.trim_start().starts_with('<') {
            return format!("Spotify ({status}): Request rejected by Spotify");
        }
        return format!("Spotify ({status}): {raw}");
    }
    format!("Spotify ({status}): API error")
}
