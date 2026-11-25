use std::collections::{HashMap, HashSet};
use std::time::Instant;

use serde::Serialize;

use crate::spotify::{
    DEFAULT_CLIENT_ID, DeviceInfo, RemotePlayer, RemotePlaylist, RemoteTrack, UserProfile,
};

/// Playback repeat mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatMode {
    Off,
    All,
    One,
}

impl RepeatMode {
    pub fn cycle(self) -> Self {
        match self {
            Self::Off => Self::All,
            Self::All => Self::One,
            Self::One => Self::Off,
        }
    }

    pub fn as_spotify_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::All => "context",
            Self::One => "track",
        }
    }

    pub fn from_spotify_str(s: &str) -> Self {
        match s {
            "context" => Self::All,
            "track" => Self::One,
            _ => Self::Off,
        }
    }
}

/// Timestamped lyric line for synced lyrics view.
#[derive(Debug, Clone, Serialize)]
pub struct LyricLine {
    pub time_secs: f32,
    pub text: String,
}

/// Track metadata and cover art / palette.
#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub id: usize,
    pub spotify_id: Option<String>,
    pub uri: Option<String>,
    pub image_url: Option<String>,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u32,
    pub explicit: bool,
    pub liked: bool,
    pub primary_rgb: [u8; 3],
    pub accent_rgb: [u8; 3],
}

/// Playlist grouping tracks by local index.
#[derive(Debug, Clone, Serialize)]
pub struct Playlist {
    pub spotify_id: Option<String>,
    pub image_url: Option<String>,
    pub loaded: bool,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub track_ids: Vec<usize>,
    pub primary_rgb: [u8; 3],
    pub accent_rgb: [u8; 3],
}

/// Artist entry for circular library rows & about card.
#[derive(Debug, Clone, Serialize)]
pub struct Artist {
    pub name: String,
    pub monthly_listeners: String,
    pub image_url: Option<String>,
    pub primary_rgb: [u8; 3],
    pub accent_rgb: [u8; 3],
}

/// Live Spotify & Soloist connection status exposed to frontend.
#[derive(Debug, Clone, Serialize)]
pub struct SpotifyStatus {
    pub connected: bool,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub client_id: String,
    pub active_device: Option<String>,
    pub devices: Vec<DeviceInfo>,
    pub soloist_running: bool,
    pub soloist_has_key: bool,
    pub soloist_bin_found: bool,
    pub soloist_bin: String,
    pub last_error: Option<String>,
}

impl Default for SpotifyStatus {
    fn default() -> Self {
        Self {
            connected: false,
            user_name: "kif".into(),
            user_avatar: None,
            client_id: DEFAULT_CLIENT_ID.into(),
            active_device: None,
            devices: Vec::new(),
            soloist_running: false,
            soloist_has_key: false,
            soloist_bin_found: false,
            soloist_bin: String::new(),
            last_error: None,
        }
    }
}

/// Serializable snapshot pushed to the Tauri WebView frontend.
#[derive(Debug, Clone, Serialize)]
pub struct StateSnapshot {
    pub tracks: Vec<Track>,
    pub playlists: Vec<Playlist>,
    pub artists: Vec<Artist>,
    pub active_context_name: String,
    pub queue: Vec<usize>,
    pub queue_pos: usize,
    pub is_playing: bool,
    pub position_secs: f32,
    pub shuffle: bool,
    pub repeat: RepeatMode,
    pub volume: f32,
    pub muted: bool,
    pub lyrics: Vec<LyricLine>,
    pub search_result_ids: Vec<usize>,
    pub spotify: SpotifyStatus,
}

/// Central Rust-side application state (supports both Demo Mode and Live Spotify Mode).
pub struct AppState {
    pub tracks: Vec<Track>,
    pub playlists: Vec<Playlist>,
    pub artists: Vec<Artist>,
    pub active_context_name: String,
    pub queue: Vec<usize>,
    pub queue_pos: usize,
    pub is_playing: bool,
    pub position_secs: f32,
    pub shuffle: bool,
    pub repeat: RepeatMode,
    pub volume: f32,
    pub muted: bool,
    pub search_result_ids: Vec<usize>,
    pub spotify: SpotifyStatus,
    spotify_track_index: HashMap<String, usize>,
    last_tick: Instant,
}

impl Default for AppState {
    fn default() -> Self {
        let tracks = dummy_tracks();
        let playlists = dummy_playlists();
        let artists = dummy_artists();
        let queue = playlists[0].track_ids.clone();
        let active_context_name = playlists[0].name.clone();
        Self {
            tracks,
            playlists,
            artists,
            active_context_name,
            queue,
            queue_pos: 0,
            is_playing: true,
            position_secs: 42.0,
            shuffle: false,
            repeat: RepeatMode::Off,
            volume: 0.72,
            muted: false,
            search_result_ids: Vec::new(),
            spotify: SpotifyStatus::default(),
            spotify_track_index: HashMap::new(),
            last_tick: Instant::now(),
        }
    }
}

fn palette_for(seed: &str) -> ([u8; 3], [u8; 3]) {
    const PALETTES: &[([u8; 3], [u8; 3])] = &[
        ([29, 185, 84], [15, 68, 34]),
        ([142, 68, 173], [44, 21, 74]),
        ([41, 128, 185], [18, 49, 77]),
        ([230, 126, 34], [94, 43, 12]),
        ([22, 160, 133], [12, 66, 56]),
        ([192, 57, 43], [79, 20, 16]),
        ([185, 29, 115], [74, 14, 49]),
        ([72, 52, 212], [30, 22, 92]),
        ([0, 168, 255], [8, 63, 97]),
    ];
    let hash = seed
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_mul(31).wrapping_add(b as usize));
    PALETTES[hash % PALETTES.len()]
}

impl AppState {
    pub fn snapshot(&mut self) -> StateSnapshot {
        self.tick();
        let lyrics = self.current_track().map(track_lyrics).unwrap_or_default();
        StateSnapshot {
            tracks: self.tracks.clone(),
            playlists: self.playlists.clone(),
            artists: self.artists.clone(),
            active_context_name: self.active_context_name.clone(),
            queue: self.queue.clone(),
            queue_pos: self.queue_pos,
            is_playing: self.is_playing,
            position_secs: self.position_secs,
            shuffle: self.shuffle,
            repeat: self.repeat,
            volume: self.volume,
            muted: self.muted,
            lyrics,
            search_result_ids: self.search_result_ids.clone(),
            spotify: self.spotify.clone(),
        }
    }

    pub fn reset_to_dummy(&mut self) {
        let keep_client_id = self.spotify.client_id.clone();
        let keep_soloist_key = self.spotify.soloist_has_key;
        let keep_soloist_bin_found = self.spotify.soloist_bin_found;
        let keep_soloist_bin = self.spotify.soloist_bin.clone();
        *self = Self::default();
        self.spotify.client_id = keep_client_id;
        self.spotify.soloist_has_key = keep_soloist_key;
        self.spotify.soloist_bin_found = keep_soloist_bin_found;
        self.spotify.soloist_bin = keep_soloist_bin;
    }

    pub fn upsert_remote_track(&mut self, rt: RemoteTrack, mark_liked: Option<bool>) -> usize {
        if let Some(&existing_id) = self.spotify_track_index.get(&rt.spotify_id) {
            if let Some(t) = self.tracks.get_mut(existing_id) {
                t.title = rt.title;
                t.artist = rt.artist;
                t.album = rt.album;
                t.duration_secs = rt.duration_secs;
                t.explicit = rt.explicit;
                if rt.image_url.is_some() {
                    t.image_url = rt.image_url;
                }
                if let Some(liked) = mark_liked {
                    t.liked = liked;
                }
            }
            return existing_id;
        }
        let id = self.tracks.len();
        let (primary_rgb, accent_rgb) = palette_for(&rt.title);
        self.spotify_track_index.insert(rt.spotify_id.clone(), id);
        self.tracks.push(Track {
            id,
            spotify_id: Some(rt.spotify_id),
            uri: Some(rt.uri),
            image_url: rt.image_url,
            title: rt.title,
            artist: rt.artist,
            album: rt.album,
            duration_secs: rt.duration_secs,
            explicit: rt.explicit,
            liked: mark_liked.unwrap_or(false),
            primary_rgb,
            accent_rgb,
        });
        id
    }

    pub fn load_spotify_library(
        &mut self,
        profile: UserProfile,
        playlists: Vec<RemotePlaylist>,
        liked: Vec<RemoteTrack>,
        recent: Vec<RemoteTrack>,
    ) {
        self.tracks.clear();
        self.spotify_track_index.clear();
        self.playlists.clear();
        self.artists.clear();
        self.search_result_ids.clear();
        self.queue.clear();
        self.queue_pos = 0;

        self.spotify.connected = true;
        self.spotify.user_name = profile.display_name;
        self.spotify.user_avatar = profile.avatar_url;
        self.spotify.last_error = None;

        let mut liked_ids = Vec::new();
        for rt in liked {
            liked_ids.push(self.upsert_remote_track(rt, Some(true)));
        }
        let mut recent_ids = Vec::new();
        for rt in recent {
            let id = self.upsert_remote_track(rt, None);
            if !recent_ids.contains(&id) {
                recent_ids.push(id);
            }
        }

        for rp in playlists {
            let (primary_rgb, accent_rgb) = palette_for(&rp.name);
            self.playlists.push(Playlist {
                spotify_id: Some(rp.spotify_id),
                image_url: rp.image_url,
                loaded: false,
                name: rp.name,
                description: rp.description,
                creator: rp.creator,
                track_ids: Vec::new(),
                primary_rgb,
                accent_rgb,
            });
        }

        // Build unique artists list from user's tracks
        let mut seen_artists = HashSet::new();
        for t in &self.tracks {
            let primary_artist = t.artist.split(',').next().unwrap_or(&t.artist).trim();
            if !primary_artist.is_empty() && seen_artists.insert(primary_artist.to_string()) {
                let (primary_rgb, accent_rgb) = palette_for(primary_artist);
                self.artists.push(Artist {
                    name: primary_artist.to_string(),
                    monthly_listeners: "Spotify Artist".into(),
                    image_url: t.image_url.clone(),
                    primary_rgb,
                    accent_rgb,
                });
                if self.artists.len() >= 12 {
                    break;
                }
            }
        }

        if !recent_ids.is_empty() {
            self.queue = recent_ids;
            self.active_context_name = "Recently Played".into();
        } else if !liked_ids.is_empty() {
            self.queue = liked_ids;
            self.active_context_name = "Liked Songs".into();
        }
        self.is_playing = false;
        self.position_secs = 0.0;
        self.last_tick = Instant::now();
    }

    pub fn set_playlist_tracks(&mut self, playlist_idx: usize, tracks: Vec<RemoteTrack>) {
        let mut ids = Vec::with_capacity(tracks.len());
        for rt in tracks {
            ids.push(self.upsert_remote_track(rt, None));
        }
        if let Some(p) = self.playlists.get_mut(playlist_idx) {
            p.track_ids = ids;
            p.loaded = true;
        }
    }

    pub fn set_search_results(&mut self, tracks: Vec<RemoteTrack>) {
        let mut ids = Vec::with_capacity(tracks.len());
        for rt in tracks {
            ids.push(self.upsert_remote_track(rt, None));
        }
        self.search_result_ids = ids;
    }

    pub fn apply_remote_player(
        &mut self,
        player: Option<RemotePlayer>,
        remote_queue: Vec<RemoteTrack>,
        devices: Vec<DeviceInfo>,
    ) {
        self.spotify.devices = devices;
        if let Some(p) = player {
            self.is_playing = p.is_playing;
            self.position_secs = p.position_secs;
            self.shuffle = p.shuffle;
            self.repeat = RepeatMode::from_spotify_str(&p.repeat);
            self.volume = p.volume.clamp(0.0, 1.0);
            self.muted = self.volume <= 0.001;
            self.spotify.active_device = p.device_name;
            self.last_tick = Instant::now();

            if let Some(cur) = p.track {
                let cur_id = self.upsert_remote_track(cur, None);
                let mut new_queue = vec![cur_id];
                for qt in remote_queue {
                    new_queue.push(self.upsert_remote_track(qt, None));
                }
                self.queue = new_queue;
                self.queue_pos = 0;
            }
        } else {
            self.is_playing = false;
            self.spotify.active_device = self
                .spotify
                .devices
                .iter()
                .find(|d| d.is_active)
                .map(|d| d.name.clone());
            self.last_tick = Instant::now();
        }
    }

    pub fn current_track(&self) -> Option<&Track> {
        let id = *self.queue.get(self.queue_pos)?;
        self.tracks.get(id)
    }

    pub fn is_track_active(&self, track_id: usize) -> bool {
        self.queue.get(self.queue_pos) == Some(&track_id)
    }

    pub fn play_in_context(&mut self, context_name: String, context_ids: &[usize], track_id: usize) {
        if context_ids.is_empty() {
            return;
        }
        self.active_context_name = context_name;
        if self.is_track_active(track_id) && self.queue == context_ids {
            self.toggle_play();
            return;
        }
        self.queue = context_ids.to_vec();
        self.queue_pos = self.queue.iter().position(|&id| id == track_id).unwrap_or(0);
        self.position_secs = 0.0;
        self.is_playing = true;
        self.last_tick = Instant::now();
    }

    pub fn toggle_play(&mut self) {
        self.tick();
        self.is_playing = !self.is_playing;
        self.last_tick = Instant::now();
    }

    pub fn next_track(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        if self.repeat == RepeatMode::One {
            self.position_secs = 0.0;
            self.is_playing = true;
            self.last_tick = Instant::now();
            return;
        }
        if self.shuffle && self.queue.len() > 1 {
            let step = (self.queue_pos + 3) % self.queue.len();
            self.queue_pos = if step == self.queue_pos {
                (self.queue_pos + 1) % self.queue.len()
            } else {
                step
            };
        } else if self.queue_pos + 1 < self.queue.len() {
            self.queue_pos += 1;
        } else if self.repeat == RepeatMode::All {
            self.queue_pos = 0;
        } else {
            self.is_playing = false;
            self.position_secs = 0.0;
            self.last_tick = Instant::now();
            return;
        }
        self.position_secs = 0.0;
        self.is_playing = true;
        self.last_tick = Instant::now();
    }

    pub fn prev_track(&mut self) {
        self.tick();
        if self.queue.is_empty() {
            return;
        }
        if self.position_secs > 3.0 {
            self.position_secs = 0.0;
            self.last_tick = Instant::now();
            return;
        }
        if self.queue_pos > 0 {
            self.queue_pos -= 1;
        } else if self.repeat == RepeatMode::All {
            self.queue_pos = self.queue.len() - 1;
        }
        self.position_secs = 0.0;
        self.is_playing = true;
        self.last_tick = Instant::now();
    }

    pub fn seek_ratio(&mut self, ratio: f32) {
        if let Some(track) = self.current_track() {
            let duration = track.duration_secs as f32;
            self.position_secs = (ratio.clamp(0.0, 1.0) * duration).min(duration);
            self.last_tick = Instant::now();
        }
    }

    pub fn seek_secs(&mut self, secs: f32) {
        if let Some(track) = self.current_track() {
            let duration = track.duration_secs as f32;
            self.position_secs = secs.clamp(0.0, duration);
            self.is_playing = true;
            self.last_tick = Instant::now();
        }
    }

    pub fn jump_queue(&mut self, idx: usize) {
        if idx < self.queue.len() {
            self.queue_pos = idx;
            self.position_secs = 0.0;
            self.is_playing = true;
            self.last_tick = Instant::now();
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32();
        self.last_tick = now;

        if !self.is_playing {
            return;
        }
        let Some(duration) = self.current_track().map(|t| t.duration_secs as f32) else {
            return;
        };
        self.position_secs += dt;
        if self.position_secs >= duration {
            if self.spotify.connected {
                self.position_secs = duration;
            } else {
                self.next_track();
            }
        }
    }

    pub fn toggle_like(&mut self, track_id: usize) {
        if let Some(track) = self.tracks.get_mut(track_id) {
            track.liked = !track.liked;
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.muted = self.volume <= 0.001;
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn create_playlist(&mut self, name: String) -> usize {
        let idx = self.playlists.len();
        let title = if name.trim().is_empty() {
            format!("My Playlist #{}", idx + 1)
        } else {
            name.trim().to_string()
        };
        let (primary_rgb, accent_rgb) = palette_for(&title);
        self.playlists.push(Playlist {
            spotify_id: None,
            image_url: None,
            loaded: true,
            name: title,
            description: "Custom playlist created in SpotiRust.".into(),
            creator: self.spotify.user_name.clone(),
            track_ids: if self.spotify.connected {
                Vec::new()
            } else {
                vec![0, 1, 4]
            },
            primary_rgb,
            accent_rgb,
        });
        idx
    }

    pub fn rename_playlist(&mut self, playlist_idx: usize, name: String) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        if let Some(p) = self.playlists.get_mut(playlist_idx) {
            p.name = trimmed.to_string();
        }
    }

    pub fn delete_playlist(&mut self, playlist_idx: usize) {
        if playlist_idx < self.playlists.len() {
            self.playlists.remove(playlist_idx);
        }
    }

    pub fn add_track_to_playlist(&mut self, playlist_idx: usize, track_id: usize) {
        if track_id >= self.tracks.len() {
            return;
        }
        if let Some(p) = self.playlists.get_mut(playlist_idx) {
            if !p.track_ids.contains(&track_id) {
                p.track_ids.push(track_id);
            }
        }
    }

    pub fn remove_track_from_playlist(&mut self, playlist_idx: usize, track_id: usize) {
        if let Some(p) = self.playlists.get_mut(playlist_idx) {
            p.track_ids.retain(|&id| id != track_id);
        }
    }

    pub fn add_to_queue(&mut self, track_id: usize) {
        if track_id < self.tracks.len() {
            self.queue.push(track_id);
        }
    }

    pub fn remove_from_queue(&mut self, idx: usize) {
        if idx > self.queue_pos && idx < self.queue.len() {
            self.queue.remove(idx);
        }
    }

    pub fn clear_queue(&mut self) {
        if !self.queue.is_empty() {
            self.queue.truncate(self.queue_pos + 1);
        }
    }
}

pub fn track_lyrics(track: &Track) -> Vec<LyricLine> {
    if track.spotify_id.is_some() {
        return vec![
            LyricLine {
                time_secs: 0.0,
                text: format!("{} — {}", track.title, track.artist),
            },
            LyricLine {
                time_secs: 8.0,
                text: "Live lyrics are not provided by the official Spotify Web API.".into(),
            },
            LyricLine {
                time_secs: 16.0,
                text: format!("Album: {}", track.album),
            },
        ];
    }

    let raw: Vec<&str> = match track.id {
        0 => vec![
            "Neon reflections on a silent screen,",
            "Midnight compile in a sea of green.",
            "Every lifetime checked, every edge aligned,",
            "Chasing zero cost through the quiet night.",
            "Borrow checker holds the line tonight,",
            "Underneath the glow of the monitor light.",
            "No allocations wasted, nothing left to fear,",
            "Only pure rhythm when the build turns clear.",
            "Step by step the waveform starts to rise,",
            "Midnight Rust beneath electric skies.",
            "Hold the tempo steady till the break of dawn,",
            "Cargo run is green and the night rolls on.",
        ],
        1 => vec![
            "Wayland windows floating in the dark,",
            "Cursor blinking like a tiny spark.",
            "Coffee cooling by the mechanical keys,",
            "Synthwave drifting through the cooling breeze.",
            "Neon terminal, glowing at 2 AM,",
            "Packets dancing through the local LAN.",
            "Turn the volume up till the bassline hits,",
            "Every pixel rendered in sixty frames crisp.",
            "Don't look back at the clock on the wall,",
            "Neon terminal answers every call.",
        ],
        6 => vec![
            "Langit jingga turun di ujung jalan,",
            "Angin sore bawa pulang kenangan.",
            "Di balik jendela kopi hangat menanti,",
            "Senja di Dago pelan menyapa hati.",
            "Langkah kaki menyusuri rindang pohon,",
            "Waktu berjalan tanpa pernah memohon.",
            "Biarkan melodi ini menemani malam,",
            "Tenang dan hangat di dalam diam.",
            "Sampai jumpa esok di pagi yang baru,",
            "Senja di Dago selalu kurindu.",
        ],
        _ => vec![
            "Turn the lights down, let the speakers breathe,",
            "Every beat is a story underneath.",
            "Lost inside the static and the midnight hum,",
            "Waiting for the chorus when the drop will come.",
            "City lights flicker through the windowpane,",
            "Rhythm washing over like the summer rain.",
            "Keep the needle spinning till the morning light,",
            "Nothing else matters in the sound tonight.",
            "Echoes fade slowly as the track winds down,",
            "Floating on a wave above a sleeping town.",
        ],
    };

    let usable = (track.duration_secs as f32 * 0.86).max(20.0);
    let step = usable / raw.len().max(1) as f32;
    raw.into_iter()
        .enumerate()
        .map(|(i, line)| LyricLine {
            time_secs: 4.0 + i as f32 * step,
            text: line.to_string(),
        })
        .collect()
}

fn dummy_track(
    id: usize,
    title: &str,
    artist: &str,
    album: &str,
    duration_secs: u32,
    explicit: bool,
    liked: bool,
    primary_rgb: [u8; 3],
    accent_rgb: [u8; 3],
) -> Track {
    Track {
        id,
        spotify_id: None,
        uri: None,
        image_url: None,
        title: title.into(),
        artist: artist.into(),
        album: album.into(),
        duration_secs,
        explicit,
        liked,
        primary_rgb,
        accent_rgb,
    }
}

fn dummy_tracks() -> Vec<Track> {
    vec![
        dummy_track(0, "Midnight Rust", "Kif & The Borrow Checkers", "Zero Cost Abstractions", 214, false, true, [29, 185, 84], [15, 68, 34]),
        dummy_track(1, "Neon Terminal", "Wayland Nights", "Fedora Dreams", 196, true, true, [142, 68, 173], [44, 21, 74]),
        dummy_track(2, "Async Await", "Tokio Drift", "Futures Unpinned", 243, false, false, [41, 128, 185], [18, 49, 77]),
        dummy_track(3, "Late Night Compile", "Cargo Cult", "Release Mode", 185, false, true, [230, 126, 34], [94, 43, 12]),
        dummy_track(4, "Coffee & Semicolons", "Bandung Lo-Fi", "Rainy Campus", 168, false, true, [22, 160, 133], [12, 66, 56]),
        dummy_track(5, "Static Lifetime", "Unsafe Club", "Raw Pointers", 229, true, false, [192, 57, 43], [79, 20, 16]),
        dummy_track(6, "Senja di Dago", "Kopi Pagi", "Jalan Panjang", 251, false, true, [211, 84, 0], [82, 33, 5]),
        dummy_track(7, "Packet Loss", "Loopback 127", "Localhost Blocked", 199, false, false, [52, 73, 94], [24, 34, 46]),
        dummy_track(8, "Velvet Synth", "Midnight Drive", "Afterglow", 236, false, true, [185, 29, 115], [74, 14, 49]),
        dummy_track(9, "Clean Diff", "Senior Dev", "Ship It", 174, true, false, [39, 174, 96], [17, 74, 41]),
        dummy_track(10, "Orbiting Saturn", "Deep Space Radio", "Telemetry", 278, false, false, [72, 52, 212], [30, 22, 92]),
        dummy_track(11, "Subzero Frame", "Wayland Nights", "60 FPS", 205, false, true, [0, 168, 255], [8, 63, 97]),
    ]
}

fn dummy_playlist(
    name: &str,
    description: &str,
    creator: &str,
    track_ids: Vec<usize>,
    primary_rgb: [u8; 3],
    accent_rgb: [u8; 3],
) -> Playlist {
    Playlist {
        spotify_id: None,
        image_url: None,
        loaded: true,
        name: name.into(),
        description: description.into(),
        creator: creator.into(),
        track_ids,
        primary_rgb,
        accent_rgb,
    }
}

fn dummy_playlists() -> Vec<Playlist> {
    vec![
        dummy_playlist("Coding Flow", "Deep focus synthwave & lo-fi beats for late-night Rust sessions.", "kif", vec![0, 2, 3, 4, 7, 9, 11], [29, 185, 84], [18, 58, 32]),
        dummy_playlist("Night Drive", "Neon synth and mellow electronic tracks after midnight.", "kif", vec![1, 5, 8, 10, 11], [142, 68, 173], [45, 22, 66]),
        dummy_playlist("Chill Campus", "Acoustic, indie, and warm afternoon vibes.", "Spotify Mix", vec![4, 6, 0, 8], [230, 126, 34], [84, 43, 14]),
        dummy_playlist("Daily Mix 1", "Kif & The Borrow Checkers, Wayland Nights, Tokio Drift and more.", "Made for kif", vec![0, 1, 2, 6, 8, 10], [41, 128, 185], [19, 53, 82]),
        dummy_playlist("Discover Weekly", "Your weekly mixtape of fresh music.", "Made for kif", vec![5, 7, 9, 10, 3], [192, 57, 43], [77, 22, 18]),
        dummy_playlist("Top 50 - Indonesia", "Your daily update of the most played tracks right now.", "Spotify", vec![6, 4, 0, 8, 1, 11], [228, 49, 77], [110, 18, 38]),
        dummy_playlist("Morning Vibes", "Warm acoustic & upbeat chill tracks to start your day.", "kif", vec![4, 6, 9, 2], [214, 158, 46], [82, 56, 14]),
    ]
}

fn dummy_artists() -> Vec<Artist> {
    vec![
        Artist {
            name: "Wayland Nights".into(),
            monthly_listeners: "1,420,890 monthly listeners".into(),
            image_url: None,
            primary_rgb: [152, 86, 206],
            accent_rgb: [42, 22, 70],
        },
        Artist {
            name: "Kopi Pagi".into(),
            monthly_listeners: "894,210 monthly listeners".into(),
            image_url: None,
            primary_rgb: [222, 112, 48],
            accent_rgb: [78, 34, 16],
        },
        Artist {
            name: "Tokio Drift".into(),
            monthly_listeners: "2,105,400 monthly listeners".into(),
            image_url: None,
            primary_rgb: [46, 146, 204],
            accent_rgb: [18, 48, 82],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_next_and_prev_respecting_repeat() {
        let mut state = AppState::default();
        state.queue = vec![0, 1];
        state.queue_pos = 1;
        state.repeat = RepeatMode::Off;
        state.next_track();
        assert!(!state.is_playing);
        assert_eq!(state.position_secs, 0.0);

        state.repeat = RepeatMode::All;
        state.queue_pos = 1;
        state.next_track();
        assert!(state.is_playing);
        assert_eq!(state.queue_pos, 0);

        state.prev_track();
        assert_eq!(state.queue_pos, 1);
    }

    #[test]
    fn toggle_like_updates_track() {
        let mut state = AppState::default();
        let was_liked = state.tracks[2].liked;
        state.toggle_like(2);
        assert_eq!(state.tracks[2].liked, !was_liked);
    }

    #[test]
    fn snapshot_includes_synced_lyrics() {
        let mut state = AppState::default();
        let snap = state.snapshot();
        assert!(!snap.lyrics.is_empty());
        assert_eq!(snap.active_context_name, "Coding Flow");
    }

    #[test]
    fn queue_and_playlist_mutations_work() {
        let mut state = AppState::default();
        let initial_q = state.queue.len();
        state.add_to_queue(10);
        assert_eq!(state.queue.len(), initial_q + 1);
        state.remove_from_queue(initial_q);
        assert_eq!(state.queue.len(), initial_q);
        state.clear_queue();
        assert_eq!(state.queue.len(), state.queue_pos + 1);

        let idx = state.create_playlist("Test Mix".into());
        assert_eq!(state.playlists[idx].name, "Test Mix");
        state.rename_playlist(idx, "Renamed Mix".into());
        assert_eq!(state.playlists[idx].name, "Renamed Mix");
        state.add_track_to_playlist(idx, 10);
        assert!(state.playlists[idx].track_ids.contains(&10));
        state.remove_track_from_playlist(idx, 10);
        assert!(!state.playlists[idx].track_ids.contains(&10));
        let before = state.playlists.len();
        state.delete_playlist(idx);
        assert_eq!(state.playlists.len(), before - 1);
    }
}
