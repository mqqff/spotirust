#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod spotify;

use std::sync::Mutex;

use data::{AppState, StateSnapshot};
use spotify::{LocalConfig, SoloistManager, SpotifyClient};
use tauri::State;

struct Backend {
    state: Mutex<AppState>,
    cfg: Mutex<LocalConfig>,
    soloist: Mutex<SoloistManager>,
    client: SpotifyClient,
}

impl Backend {
    fn new() -> Self {
        let cfg = LocalConfig::load();
        let mut state = AppState::default();
        state.spotify.client_id = cfg.client_id.clone();
        state.spotify.soloist_has_key = !cfg.soloist_api_key.trim().is_empty();
        state.spotify.soloist_bin = cfg.soloist_bin.clone();
        state.spotify.soloist_bin_found = SoloistManager::find_binary(&cfg.soloist_bin).is_some();
        Self {
            state: Mutex::new(state),
            cfg: Mutex::new(cfg),
            soloist: Mutex::new(SoloistManager::default()),
            client: SpotifyClient::default(),
        }
    }

    fn refresh_local_status(&self) {
        let cfg = self.cfg.lock().unwrap().clone();
        let running = self.soloist.lock().unwrap().is_running();
        let mut st = self.state.lock().unwrap();
        st.spotify.client_id = cfg.client_id;
        st.spotify.soloist_running = running;
        st.spotify.soloist_has_key = !cfg.soloist_api_key.trim().is_empty();
        st.spotify.soloist_bin_found = SoloistManager::find_binary(&cfg.soloist_bin).is_some();
        st.spotify.soloist_bin = cfg.soloist_bin;
    }

    fn snapshot(&self) -> StateSnapshot {
        self.refresh_local_status();
        self.state.lock().unwrap().snapshot()
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
        let playlists = self.client.fetch_playlists(token).await.unwrap_or_default();
        let liked = self.client.fetch_liked_tracks(token).await.unwrap_or_default();
        let recent = self.client.fetch_recently_played(token).await.unwrap_or_default();
        let player = self.client.fetch_player(token).await.ok().flatten();
        let queue = self.client.fetch_queue(token).await.unwrap_or_default();
        let devices = self.client.fetch_devices(token).await.unwrap_or_default();

        let mut st = self.state.lock().unwrap();
        st.load_spotify_library(profile, playlists, liked, recent);
        st.apply_remote_player(player, queue, devices);
        Ok(())
    }

    async fn sync_player_only(&self, token: &str) {
        let player = self.client.fetch_player(token).await.ok().flatten();
        let queue = self.client.fetch_queue(token).await.unwrap_or_default();
        let devices = self.client.fetch_devices(token).await.unwrap_or_default();
        self.state
            .lock()
            .unwrap()
            .apply_remote_player(player, queue, devices);
    }
}

#[tauri::command]
fn get_state(backend: State<'_, Backend>) -> StateSnapshot {
    backend.snapshot()
}

#[tauri::command]
async fn spotify_bootstrap(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let cfg = backend.cfg.lock().unwrap().clone();
    if !cfg.soloist_api_key.trim().is_empty()
        && SoloistManager::find_binary(&cfg.soloist_bin).is_some()
    {
        let _ = backend
            .soloist
            .lock()
            .unwrap()
            .start(&cfg.soloist_api_key, &cfg.soloist_bin);
    }
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.sync_full_library(&tok).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn spotify_login(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let mut cfg = backend.cfg.lock().unwrap().clone();
    if let Err(e) = backend.client.login_pkce(&mut cfg).await {
        backend.set_error(Some(e.clone()));
        return Err(e);
    }
    let tok = cfg.access_token.clone().unwrap_or_default();
    *backend.cfg.lock().unwrap() = cfg.clone();

    if !cfg.soloist_api_key.trim().is_empty()
        && SoloistManager::find_binary(&cfg.soloist_bin).is_some()
    {
        let _ = backend
            .soloist
            .lock()
            .unwrap()
            .start(&cfg.soloist_api_key, &cfg.soloist_bin);
    }

    if let Err(e) = backend.sync_full_library(&tok).await {
        backend.set_error(Some(e.clone()));
        return Err(e);
    }
    Ok(backend.snapshot())
}

#[tauri::command]
fn spotify_logout(backend: State<'_, Backend>) -> StateSnapshot {
    {
        let mut cfg = backend.cfg.lock().unwrap();
        cfg.access_token = None;
        cfg.refresh_token = None;
        cfg.expires_at = 0;
        let _ = cfg.save();
    }
    backend.soloist.lock().unwrap().stop();
    backend.state.lock().unwrap().reset_to_dummy();
    backend.snapshot()
}

#[tauri::command]
fn spotify_save_settings(
    backend: State<'_, Backend>,
    client_id: String,
    soloist_api_key: String,
    soloist_bin: String,
) -> Result<StateSnapshot, String> {
    {
        let mut cfg = backend.cfg.lock().unwrap();
        if !client_id.trim().is_empty() {
            cfg.client_id = client_id.trim().to_string();
        }
        if !soloist_api_key.trim().is_empty() {
            cfg.soloist_api_key = soloist_api_key.trim().to_string();
        }
        cfg.soloist_bin = soloist_bin.trim().to_string();
        cfg.save()?;
    }
    backend.set_error(None);
    Ok(backend.snapshot())
}

#[tauri::command]
fn soloist_toggle(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let cfg = backend.cfg.lock().unwrap().clone();
    let mut mgr = backend.soloist.lock().unwrap();
    if mgr.is_running() {
        mgr.stop();
    } else if let Err(e) = mgr.start(&cfg.soloist_api_key, &cfg.soloist_bin) {
        drop(mgr);
        backend.set_error(Some(e.clone()));
        return Err(e);
    }
    drop(mgr);
    backend.set_error(None);
    Ok(backend.snapshot())
}

#[tauri::command]
async fn spotify_sync_player(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    if let Some(tok) = backend.token_if_connected().await {
        backend.sync_player_only(&tok).await;
    }
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
}

#[tauri::command]
async fn play_in_context(
    backend: State<'_, Backend>,
    context_name: String,
    track_ids: Vec<usize>,
    track_id: usize,
) -> Result<StateSnapshot, String> {
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
            if let Err(e) = backend.client.play_uris(&tok, remote_uris, offset_uri).await {
                backend.state.lock().unwrap().is_playing = false;
                backend.set_error(Some(e));
            } else {
                backend.set_error(None);
            }
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn toggle_play(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let now_playing = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_play();
        st.is_playing
    };
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
    Ok(backend.snapshot())
}

#[tauri::command]
async fn next_track(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.state.lock().unwrap().next_track();
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.client.next(&tok).await {
            backend.set_error(Some(e));
        } else {
            backend.set_error(None);
            backend.sync_player_only(&tok).await;
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn prev_track(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    backend.state.lock().unwrap().prev_track();
    if let Some(tok) = backend.token_if_connected().await {
        if let Err(e) = backend.client.previous(&tok).await {
            backend.set_error(Some(e));
        } else {
            backend.set_error(None);
            backend.sync_player_only(&tok).await;
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn seek_ratio(backend: State<'_, Backend>, ratio: f32) -> Result<StateSnapshot, String> {
    let ms = {
        let mut st = backend.state.lock().unwrap();
        st.seek_ratio(ratio);
        (st.position_secs * 1000.0) as u64
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.seek(&tok, ms).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn seek_secs(backend: State<'_, Backend>, secs: f32) -> Result<StateSnapshot, String> {
    let ms = {
        let mut st = backend.state.lock().unwrap();
        st.seek_secs(secs);
        (st.position_secs * 1000.0) as u64
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.seek(&tok, ms).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn toggle_shuffle(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let enabled = {
        let mut st = backend.state.lock().unwrap();
        st.shuffle = !st.shuffle;
        st.shuffle
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_shuffle(&tok, enabled).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn cycle_repeat(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let mode = {
        let mut st = backend.state.lock().unwrap();
        st.repeat = st.repeat.cycle();
        st.repeat
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_repeat(&tok, mode.as_spotify_str()).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn set_volume(backend: State<'_, Backend>, volume: f32) -> Result<StateSnapshot, String> {
    let pct = {
        let mut st = backend.state.lock().unwrap();
        st.set_volume(volume);
        (st.volume * 100.0).round() as u8
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_volume(&tok, pct).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn toggle_mute(backend: State<'_, Backend>) -> Result<StateSnapshot, String> {
    let pct = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_mute();
        if st.muted {
            0
        } else {
            (st.volume * 100.0).round() as u8
        }
    };
    if let Some(tok) = backend.token_if_connected().await {
        let _ = backend.client.set_volume(&tok, pct).await;
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn toggle_like(backend: State<'_, Backend>, track_id: usize) -> Result<StateSnapshot, String> {
    let remote = {
        let mut st = backend.state.lock().unwrap();
        st.toggle_like(track_id);
        st.tracks.get(track_id).and_then(|t| {
            Some((
                t.uri.clone()?,
                t.spotify_id.clone()?,
                t.liked,
            ))
        })
    };
    if let (Some((uri, sid, liked)), Some(tok)) = (remote, backend.token_if_connected().await) {
        if let Err(e) = backend.client.set_track_liked(&tok, &uri, &sid, liked).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
async fn jump_queue(backend: State<'_, Backend>, idx: usize) -> Result<StateSnapshot, String> {
    let uri = {
        let mut st = backend.state.lock().unwrap();
        st.jump_queue(idx);
        st.current_track().and_then(|t| t.uri.clone())
    };
    if let (Some(u), Some(tok)) = (uri, backend.token_if_connected().await) {
        let _ = backend.client.play_uris(&tok, vec![u], None).await;
    }
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
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
        st.playlists.get(playlist_idx).and_then(|p| p.spotify_id.clone())
    };
    if let (Some(pid), Some(tok)) = (sp_id, backend.token_if_connected().await) {
        if let Err(e) = backend.client.rename_playlist(&tok, &pid, name.trim()).await {
            backend.set_error(Some(e));
        }
    }
    Ok(backend.snapshot())
}

#[tauri::command]
fn delete_playlist(backend: State<'_, Backend>, playlist_idx: usize) -> StateSnapshot {
    backend.state.lock().unwrap().delete_playlist(playlist_idx);
    backend.snapshot()
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
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
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
    Ok(backend.snapshot())
}

#[tauri::command]
fn remove_from_queue(backend: State<'_, Backend>, idx: usize) -> StateSnapshot {
    backend.state.lock().unwrap().remove_from_queue(idx);
    backend.snapshot()
}

#[tauri::command]
fn clear_queue(backend: State<'_, Backend>) -> StateSnapshot {
    backend.state.lock().unwrap().clear_queue();
    backend.snapshot()
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
            soloist_toggle,
            spotify_sync_player,
            spotify_load_playlist,
            spotify_search,
            spotify_transfer_device,
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
