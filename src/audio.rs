use std::path::PathBuf;

use librespot::{
    connect::{ConnectConfig, Spirc},
    core::{
        authentication::Credentials,
        cache::Cache,
        config::{DeviceType, SessionConfig},
        session::Session,
    },
    oauth::OAuthClientBuilder,
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::{self, MixerConfig},
        player::Player,
    },
};
use tokio::task::JoinHandle;

use crate::spotify::LocalConfig;

const OAUTH_REDIRECT_URI: &str = "http://127.0.0.1:8898/login";

pub struct AudioEngine {
    spirc: Option<Spirc>,
    task: Option<JoinHandle<()>>,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self {
            spirc: None,
            task: None,
        }
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

impl AudioEngine {
    pub fn cache_root() -> PathBuf {
        LocalConfig::path()
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("audio-cache")
    }

    pub fn has_cached_credentials() -> bool {
        Self::cache_root().join("credentials.json").is_file()
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(handle) = self.task.as_ref() {
            if handle.is_finished() {
                self.task = None;
                self.spirc = None;
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub async fn start(&mut self, interactive_auth: bool) -> Result<bool, String> {
        if self.is_running() {
            return Ok(true);
        }

        let cache_root = Self::cache_root();
        let files_dir = cache_root.join("files");
        std::fs::create_dir_all(&files_dir).map_err(|e| format!("Audio cache dir error: {e}"))?;

        let cache = Cache::new(
            Some(&cache_root),
            Some(&cache_root),
            Some(&files_dir),
            Some(256 * 1024 * 1024),
        )
        .map_err(|e| format!("Gagal inisialisasi audio cache: {e}"))?;

        let session_config = SessionConfig::default();

        let credentials = if let Some(cached) = cache.credentials() {
            cached
        } else if interactive_auth {
            let client_id = session_config.client_id.clone();
            let token = tokio::task::spawn_blocking(move || {
                OAuthClientBuilder::new(&client_id, OAUTH_REDIRECT_URI, vec!["streaming"])
                    .open_in_browser()
                    .build()
                    .map_err(|e| format!("Audio OAuth build error: {e}"))?
                    .get_access_token()
                    .map_err(|e| format!("Audio OAuth login gagal: {e}"))
            })
            .await
            .map_err(|e| format!("Audio OAuth task error: {e}"))??;
            Credentials::with_access_token(token.access_token)
        } else {
            return Ok(false);
        };

        let sink_builder = audio_backend::find(None)
            .ok_or_else(|| "Audio output backend tidak tersedia di sistem ini.".to_string())?;
        let mixer_builder = mixer::find(None)
            .ok_or_else(|| "Audio mixer tidak tersedia.".to_string())?;

        let session = Session::new(session_config, Some(cache));
        let mixer = mixer_builder(MixerConfig::default())
            .map_err(|e| format!("Gagal inisialisasi audio mixer: {e}"))?;

        let audio_format = AudioFormat::default();
        let player = Player::new(
            PlayerConfig::default(),
            session.clone(),
            mixer.get_soft_volume(),
            move || sink_builder(None, audio_format),
        );

        let connect_config = ConnectConfig {
            name: "SpotiRust".to_string(),
            device_type: DeviceType::Computer,
            initial_volume: (u16::MAX as f32 * 0.72) as u16,
            ..ConnectConfig::default()
        };

        let (spirc, spirc_task) =
            Spirc::new(connect_config, session, credentials, player, mixer)
                .await
                .map_err(|e| format!("Gagal menyalakan built-in Spotify audio engine: {e}"))?;

        let _ = spirc.activate();
        let handle = tokio::spawn(spirc_task);

        self.spirc = Some(spirc);
        self.task = Some(handle);
        Ok(true)
    }

    pub fn stop(&mut self) {
        if let Some(spirc) = self.spirc.take() {
            let _ = spirc.shutdown();
        }
        if let Some(handle) = self.task.take() {
            handle.abort();
        }
    }

    pub fn clear_cache(&mut self) {
        self.stop();
        let _ = std::fs::remove_file(Self::cache_root().join("credentials.json"));
    }

    pub fn pause(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.pause();
        }
    }

    pub fn play(&self) {
        if let Some(spirc) = &self.spirc {
            let _ = spirc.play();
        }
    }

    pub fn set_volume_ratio(&self, volume: f32) {
        if let Some(spirc) = &self.spirc {
            let v = (volume.clamp(0.0, 1.0) * u16::MAX as f32).round() as u16;
            let _ = spirc.set_volume(v);
        }
    }
}
