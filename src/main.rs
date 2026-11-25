#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod data;
mod lyrics;
mod spotify;

use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use audio::AudioEngine;
use data::{AppState, StateSnapshot};
use lyrics::LyricsClient;
use spotify::{LocalConfig, SpotifyClient};
use tauri::State;

struct Backend {
    state: Mutex<AppState>,
    cfg: Mutex<LocalConfig>,
    audio: tokio::sync::Mutex<AudioEngine>,
    client: SpotifyClient,
    lyrics: LyricsClient,
    last_user_action: Mutex<Option<Instant>>,
    poll_count: AtomicU64,
}

impl Backend {
    fn new() -> Self {
        let cfg = LocalConfig::load();
        let mut state = AppState::default();
        state.spotify.client_id = cfg.client_id.clone();
        state.spotify.audio_cached = AudioEngine::has_cached_credentials();
        Self {
            state: Mutex::new(state),
            cfg: Mutex::new(cfg),
            audio: tokio::sync::Mutex::new(AudioEngine::default()),
            client: SpotifyClient::default(),
            lyrics: LyricsClient::default(),
            last_user_action: Mutex::new(None),
            poll_count: AtomicU64::new(0),
        }
    }

    fn mark_user_action(&self) {
        *self.last_user_action.lock().unwrap() = Some(Instant::now());
    }

    fn recent_user_action(&self, window: Duration) -> bool {
        self.last_user_action
            .lock()
            .unwrap()
            .is_some_and(|t| t.elapsed() < window)
    }

    async fn snapshot_async(&self) -> StateSnapshot {
        let running = self.audio.lock().await.is_running();
        let cfg_client_id = self.cfg.lock().unwrap().client_id.clone();
        let mut st = self.state.lock().unwrap();
        st.spotify.client_id = cfg_client_id;
        st.spotify.audio_running = running;
        st.spotify.audio_cached = AudioEngine::has_cached_credentials();
        st.snapshot()
    }

    fn snapshot_sync(&self) -> StateSnapshot {
        let cfg_client_id = self.cfg.lock().unwrap().client_id.clone();
        let mut st = self.state.lock().unwrap();
        st.spotify.client_id = cfg_client_id;
        st.spotify.audio_cached = AudioEngine::has_cached_credentials();
        st.snapshot()
    }

    fn set_error(&self, err: Option<String>) {
        self.state.lock().unwrap().spotify.last_error = err;
    }

    async fn token_if_connected(&self) -> Option<String> {
        let mut cfg = self.cfg.lock().unwrap().clone();
        if !cfg.is_logged_in() {
            return None;
        }
        match self.client.ensure_token(&mut cfg).await {
            Ok(tok) => {
                *self.cfg.lock().unwrap() = cfg;
                Some(tok)
            }
            Err(e) => {
                self.set_error(Some(e));
                None
            }
        }
    }

    async fn sync_full_library(&self, token: &str) -> Result<(), String> {
        let profile = self.client.fetch_profile(token).await?;
        let (playlists, liked, recent, player, queue, devices) = tokio::join!(
            self.client.fetch_playlists(token),
            self.client.fetch_liked_tracks(token),
            self.client.fetch_recently_played(token),
            self.client.fetch_player(token),
            self.client.fetch_queue(token),
            self.client.fetch_devices(token),
        );

        let mut st = self.state.lock().unwrap();
        st.load_spotify_library(
            profile,
            playlists.unwrap_or_default(),
            liked.unwrap_or_default(),
            recent.unwrap_or_default(),
        );
        st.apply_remote_player(
            player.ok().flatten(),
            queue.unwrap_or_default(),
            devices.unwrap_or_default(),
        );
        Ok(())
    }

    async fn builtin_device_id(&self, token: &str) -> Option<String> {
        if !self.audio.lock().await.is_running() {
            return None;
        }
        self.client.find_device_id(token, "SpotiRust").await
    }

    async fn sync_player_only(&self, token: &str) {
        let (player, queue, devices) = tokio::join!(
            self.client.fetch_player(token),
            self.client.fetch_queue(token),
            self.client.fetch_devices(token),
        );
        self.state.lock().unwrap().apply_remote_player(
            player.ok().flatten(),
            queue.unwrap_or_default(),
            devices.unwrap_or_default(),
        );
    }
}

#[tauri::command]
fn get_state(backend: State<'_, Backend>) -> StateSnapshot {
    backend.snapshot_sync()
}

#[tauri::command]
async fn spotify_bootstrap(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    if AudioEngine::has_cached_credentials() {
        let _ = backend.audio.lock().await.start(false).await;
    }
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.sync_full_library(&tok).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_login(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let mut cfg = backend.cfg.lock().unwrap().clone();
    if let Err(e) = backend.client.login_pkce(&mut cfg).await {
        backend.set_error(Some(e.clone()));
        return Err(e);
    }
    let tok = cfg.access_token.clone().unwrap_or_default();
    *backend.cfg.lock().unwrap() = cfg;

    if AudioEngine::has_cached_credentials() {
        let _ = backend.audio.lock().await.start(false).await;
    }

    if let Err(e) = backend.sync_full_library(&tok).await {
        backend.set_error(Some(e.clone()));
        return Err(e);
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_logout(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    {
        let mut cfg = backend.cfg.lock().unwrap();
        cfg.access_token = None;
        cfg.refresh_token = None;
        cfg.expires_at = 0;
        let _ = cfg.save();
    }
    backend.audio.lock().await.clear_cache();
    backend.state.lock().unwrap().reset_to_dummy();
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_save_settings(
    backend: State<'_, Backend>,
    client_id: String,
) -> Result<StateSnapshot, String> {
    {
        let mut cfg = backend.cfg.lock().unwrap();
        if !client_id.trim().is_empty() {
            cfg.client_id = client_id.trim().to_string();
        }
        cfg.save()?;
    }
    backend.set_error(None);
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn audio_toggle(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let mut eng = backend.audio.lock().await;
    if eng.is_running() {
        eng.stop();
        drop(eng);
        backend.set_error(None);
    } else {
        let res = eng.start(true).await;
        drop(eng);
        if let Err(e) = res {
            backend.set_error(Some(e.clone()));
            return Err(e);
        }
        backend.set_error(None);
        // Wait for SpotiRust Connect device to register, then transfer playback to it
        if let Some(tok) = backend.token_if_connected().await {
            for _ in 0..8 {
                if let Some(dev_id) = backend.client.find_device_id(&tok, "SpotiRust").await {
                    let _ = backend.client.transfer_playback(&tok, &dev_id).await;
                    tokio::time::sleep(std::time::Duration::from_millis(350)).await;
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            backend.sync_player_only(&tok).await;
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_sync_player(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    if backend.recent_user_action(Duration::from_millis(1800)) {
        return Ok(backend.snapshot_async().await);
    }
    let started_at = Instant::now();
    if let Some(tok) = backend.token_if_connected().await {
        let player = backend.client.fetch_player(&tok).await.ok().flatten();
        // Discard stale poll if user clicked play/pause/skip while request was in flight
        if backend
            .last_user_action
            .lock()
            .unwrap()
            .is_some_and(|t| t >= started_at || t.elapsed() < Duration::from_millis(1800))
        {
            return Ok(backend.snapshot_async().await);
        }
        let track_changed = backend
            .state
            .lock()
            .unwrap()
            .apply_remote_player_fast(player.clone());
        let n = backend.poll_count.fetch_add(1, Ordering::Relaxed);
        if track_changed || n % 4 == 0 {
            let (queue, devices) = tokio::join!(
                backend.client.fetch_queue(&tok),
                backend.client.fetch_devices(&tok),
            );
            if !backend.recent_user_action(Duration::from_millis(1800)) {
                backend.state.lock().unwrap().apply_remote_player(
                    player,
                    queue.unwrap_or_default(),
                    devices.unwrap_or_default(),
                );
            }
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_load_playlist(
    backend: State<'_, Backend>,
    playlist_idx: usize,
) -> Result<StateSnapshot, String> {
    let sp_id = {
        let st = backend.state.lock().unwrap();
        st.playlists.get(playlist_idx).and_then(|p| {
            if p.loaded {
                None
            } else {
                p.spotify_id.clone()
            }
        })
    };
    if let (Some(pid), Some(tok)) = (sp_id, backend.token_if_connected().await) {
        match backend.client.fetch_playlist_tracks(&tok, &pid).await {
            Ok(tracks) => {
                backend
                    .state
                    .lock()
                    .unwrap()
                    .set_playlist_tracks(playlist_idx, tracks);
                backend.set_error(None);
            }
            Err(e) => backend.set_error(Some(e)),
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_search(
    backend: State<'_, Backend>,
    query: String,
) -> Result<StateSnapshot, String> {
    if let Some(tok) = backend.token_if_connected().await {
        match backend.client.search_tracks(&tok, &query).await {
            Ok(tracks) => {
                backend.state.lock().unwrap().set_search_results(tracks);
                backend.set_error(None);
            }
            Err(e) => backend.set_error(Some(e)),
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn spotify_transfer_device(
    backend: State<'_, Backend>,
    device_id: String,
) -> Result<StateSnapshot, String> {
    if let Some(tok) = backend.token_if_connected().await {
        match backend.client.transfer_playback(&tok, &device_id).await {
            Ok(()) => {
                backend.set_error(None);
                backend.sync_player_only(&tok).await;
            }
            Err(e) => backend.set_error(Some(e)),
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn fetch_lyrics(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let target = {
        let st = backend.state.lock().unwrap();
        st.current_track().and_then(|t| {
            let key = AppState::lyrics_key(t);
            if t.spotify_id.is_some() && !st.has_cached_lyrics(&key) {
                Some((key, t.clone()))
            } else {
                None
            }
        })
    };
    if let Some((key, track)) = target {
        let lines = backend.lyrics.fetch_for_track(&track).await;
        backend.state.lock().unwrap().set_lyrics(key, lines);
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn play_playlist(
    backend: State<'_, Backend>,
    playlist_idx: usize,
) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let info = {
        let st = backend.state.lock().unwrap();
        st.playlists
            .get(playlist_idx)
            .map(|p| (p.spotify_id.clone(), p.name.clone(), p.track_ids.clone()))
    };
    let Some((sp_id, name, local_ids)) = info else {
        return Ok(backend.snapshot_async().await);
    };

    if let (Some(pid), Some(tok)) = (sp_id, backend.token_if_connected().await) {
        let ctx_uri = format!("spotify:playlist:{pid}");
        let dev_id = backend.builtin_device_id(&tok).await;
        match backend
            .client
            .play_context_uri(&tok, &ctx_uri, dev_id.as_deref())
            .await
        {
            Ok(()) => {
                {
                    let mut st = backend.state.lock().unwrap();
                    st.active_context_name = name;
                    st.is_playing = true;
                }
                backend.set_error(None);
                tokio::time::sleep(std::time::Duration::from_millis(400)).await;
                backend.sync_player_only(&tok).await;
            }
            Err(e) => {
                backend.state.lock().unwrap().is_playing = false;
                backend.set_error(Some(e));
            }
        }
    } else if !local_ids.is_empty() {
        let first = local_ids[0];
        backend
            .state
            .lock()
            .unwrap()
            .play_in_context(name, &local_ids, first);
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn play_in_context(
    backend: State<'_, Backend>,
    context_name: String,
    track_ids: Vec<usize>,
    track_id: usize,
) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let (remote_uris, offset_uri) = {
        let mut st = backend.state.lock().unwrap();
        st.play_in_context(context_name, &track_ids, track_id);
        let uris: Vec<String> = track_ids
            .iter()
            .filter_map(|&id| st.tracks.get(id)?.uri.clone())
            .take(50)
            .collect();
        let off = st.tracks.get(track_id).and_then(|t| t.uri.clone());
        (uris, off)
    };

    if !remote_uris.is_empty() {
        if let Some(tok) = backend.token_if_connected().await {
            let dev_id = backend.builtin_device_id(&tok).await;
            if let Err(e) = backend
                .client
                .play_uris(&tok, remote_uris, offset_uri, dev_id.as_deref())
                .await
            {
                backend.state.lock().unwrap().is_playing = false;
                backend.set_error(Some(e));
            } else {
                backend.set_error(None);
            }
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn toggle_play(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let now_playing = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_play();
        st.is_playing
    };
    {
        let mut eng = backend.audio.lock().await;
        if eng.is_running() {
            if now_playing {
                eng.play();
            } else {
                eng.pause();
            }
        }
    }
    if let Some(tok) = backend.token_if_connected().await {
        let res = if now_playing {
            backend.client.resume(&tok).await
        } else {
            backend.client.pause(&tok).await
        };
        if let Err(e) = res {
            if now_playing {
                backend.state.lock().unwrap().is_playing = false;
            }
            backend.set_error(Some(e));
        } else {
            backend.set_error(None);
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn next_track(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    backend.state.lock().unwrap().next_track();
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.client.next(&tok).await {
            backend.set_error(Some(e));
        } else {
            backend.set_error(None);
            tokio::time::sleep(Duration::from_millis(300)).await;
            backend.sync_player_only(&tok).await;
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn prev_track(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    backend.state.lock().unwrap().prev_track();
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.client.previous(&tok).await {
            backend.set_error(Some(e));
        } else {
            backend.set_error(None);
            tokio::time::sleep(Duration::from_millis(300)).await;
            backend.sync_player_only(&tok).await;
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn seek_ratio(backend: State<'_, Backend>, ratio: f32) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let ms = {
        let mut st = backend.state.lock().unwrap();
        st.seek_ratio(ratio);
        (st.position_secs * 1000.0) as u64
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.seek(&tok, ms).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn seek_secs(backend: State<'_, Backend>, secs: f32) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let ms = {
        let mut st = backend.state.lock().unwrap();
        st.seek_secs(secs);
        (st.position_secs * 1000.0) as u64
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.seek(&tok, ms).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn toggle_shuffle(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let enabled = {
        let mut st = backend.state.lock().unwrap();
        st.shuffle = !st.shuffle;
        st.shuffle
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_shuffle(&tok, enabled).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn cycle_repeat(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let mode = {
        let mut st = backend.state.lock().unwrap();
        st.repeat = st.repeat.cycle();
        st.repeat
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_repeat(&tok, mode.as_spotify_str()).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn set_volume(backend: State<'_, Backend>, volume: f32) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let (vol, pct) = {
        let mut st = backend.state.lock().unwrap();
        st.set_volume(volume);
        (st.volume, (st.volume * 100.0).round() as u8)
    };
    {
        let mut eng = backend.audio.lock().await;
        if eng.is_running() {
            eng.set_volume_ratio(vol);
        }
    }
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_volume(&tok, pct).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn toggle_mute(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.mark_user_action();
    let (vol, pct) = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_mute();
        if st.muted {
            (0.0, 0)
        } else {
            (st.volume, (st.volume * 100.0).round() as u8)
        }
    };
    {
        let mut eng = backend.audio.lock().await;
        if eng.is_running() {
            eng.set_volume_ratio(vol);
        }
    }
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_volume(&tok, pct).await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn toggle_like(backend: State<'_, Backend>, track_id: usize) -> Result<StateSnapshot, String> {
    let remote = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_like(track_id);
        st.tracks
            .get(track_id)
            .and_then(|t| Some((t.uri.clone()?, t.spotify_id.clone()?, t.liked)))
    };
    if let (Some((uri, sid, liked)), Some(tok)) = (remote, backend.token_if_connected().await) {
        if let Err(e) = backend.client.set_track_liked(&tok, &uri, &sid, liked).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn jump_queue(backend: State<'_, Backend>, idx: usize) -> Result<StateSnapshot, String> {
    let uri = {
        let mut st = backend.state.lock().unwrap();
        st.jump_queue(idx);
        st.current_track().and_then(|t| t.uri.clone())
    };
    if let (Some(u), Some(tok)) = (uri, backend.token_if_connected().await) {
        let dev_id = backend.builtin_device_id(&tok).await;
        let _ = backend
            .client
            .play_uris(&tok, vec![u], None, dev_id.as_deref())
            .await;
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn create_playlist(backend: State<'_, Backend>, name: String) -> Result<StateSnapshot, String> {
    let idx = backend.state.lock().unwrap().create_playlist(name.clone());
    let final_name = backend.state.lock().unwrap().playlists[idx].name.clone();
    if let Some(tok) = backend.token_if_connected().await {
        match backend.client.create_playlist(&tok, &final_name).await {
            Ok(rp) => {
                if let Some(p) = backend.state.lock().unwrap().playlists.get_mut(idx) {
                    p.spotify_id = Some(rp.spotify_id);
                }
            }
            Err(e) => backend.set_error(Some(e)),
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn rename_playlist(
    backend: State<'_, Backend>,
    playlist_idx: usize,
    name: String,
) -> Result<StateSnapshot, String> {
    let sp_id = {
        let mut st = backend.state.lock().unwrap();
        st.rename_playlist(playlist_idx, name.clone());
        st.playlists
            .get(playlist_idx)
            .and_then(|p| p.spotify_id.clone())
    };
    if let (Some(pid), Some(tok)) = (sp_id, backend.token_if_connected().await) {
        if let Err(e) = backend.client.rename_playlist(&tok, &pid, name.trim()).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
fn delete_playlist(backend: State<'_, Backend>, playlist_idx: usize) -> StateSnapshot {
    backend.state.lock().unwrap().delete_playlist(playlist_idx);
    backend.snapshot_sync()
}

#[tauri::command]
async fn add_track_to_playlist(
    backend: State<'_, Backend>,
    playlist_idx: usize,
    track_id: usize,
) -> Result<StateSnapshot, String> {
    let remote = {
        let mut st = backend.state.lock().unwrap();
        st.add_track_to_playlist(playlist_idx, track_id);
        let pid = st.playlists.get(playlist_idx).and_then(|p| p.spotify_id.clone());
        let uri = st.tracks.get(track_id).and_then(|t| t.uri.clone());
        pid.zip(uri)
    };
    if let (Some((pid, uri)), Some(tok)) = (remote, backend.token_if_connected().await) {
        if let Err(e) = backend.client.add_track_to_playlist(&tok, &pid, &uri).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn remove_track_from_playlist(
    backend: State<'_, Backend>,
    playlist_idx: usize,
    track_id: usize,
) -> Result<StateSnapshot, String> {
    let remote = {
        let mut st = backend.state.lock().unwrap();
        st.remove_track_from_playlist(playlist_idx, track_id);
        let pid = st.playlists.get(playlist_idx).and_then(|p| p.spotify_id.clone());
        let uri = st.tracks.get(track_id).and_then(|t| t.uri.clone());
        pid.zip(uri)
    };
    if let (Some((pid, uri)), Some(tok)) = (remote, backend.token_if_connected().await) {
        if let Err(e) = backend
            .client
            .remove_track_from_playlist(&tok, &pid, &uri)
            .await
        {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
async fn add_to_queue(backend: State<'_, Backend>, track_id: usize) -> Result<StateSnapshot, String> {
    let uri = {
        let mut st = backend.state.lock().unwrap();
        st.add_to_queue(track_id);
        st.tracks.get(track_id).and_then(|t| t.uri.clone())
    };
    if let (Some(u), Some(tok)) = (uri, backend.token_if_connected().await) {
        if let Err(e) = backend.client.add_to_queue(&tok, &u).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot_async().await)
}

#[tauri::command]
fn remove_from_queue(backend: State<'_, Backend>, idx: usize) -> StateSnapshot {
    backend.state.lock().unwrap().remove_from_queue(idx);
    backend.snapshot_sync()
}

#[tauri::command]
fn clear_queue(backend: State<'_, Backend>) -> StateSnapshot {
    backend.state.lock().unwrap().clear_queue();
    backend.snapshot_sync()
}

fn main() {
    tauri::Builder::default()
        .manage(Backend::new())
        .invoke_handler(tauri::generate_handler![
            get_state,
            spotify_bootstrap,
            spotify_login,
            spotify_logout,
            spotify_save_settings,
            audio_toggle,
            spotify_sync_player,
            spotify_load_playlist,
            spotify_search,
            spotify_transfer_device,
            fetch_lyrics,
            play_playlist,
            play_in_context,
            toggle_play,
            next_track,
            prev_track,
            seek_ratio,
            seek_secs,
            toggle_shuffle,
            cycle_repeat,
            set_volume,
            toggle_mute,
            toggle_like,
            jump_queue,
            create_playlist,
            rename_playlist,
            delete_playlist,
            add_track_to_playlist,
            remove_track_from_playlist,
            add_to_queue,
            remove_from_queue,
            clear_queue,
        ])
        .run(tauri::generate_context!())
        .expect("error while running SpotiRust");
}
