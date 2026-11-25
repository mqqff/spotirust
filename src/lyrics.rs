use std::time::Duration;

use serde_json::Value;
use url::Url;

use crate::data::{LyricLine, Track};

pub struct LyricsClient {
    http: reqwest::Client,
}

impl Default for LyricsClient {
    fn default() -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("SpotiRust/0.2.0 (https://github.com/mqqff/spotirust)")
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }
}

impl LyricsClient {
    pub async fn fetch_for_track(&self, track: &Track) -> Vec<LyricLine> {
        let primary_artist = track
            .artist
            .split(',')
            .next()
            .unwrap_or(&track.artist)
            .trim();

        // 1. Exact match endpoint
        if let Ok(mut url) = Url::parse("https://lrclib.net/api/get") {
            url.query_pairs_mut()
                .append_pair("track_name", &track.title)
                .append_pair("artist_name", primary_artist)
                .append_pair("album_name", &track.album)
                .append_pair("duration", &track.duration_secs.to_string());

            if let Some(lines) = self.request_single(url.as_str(), track.duration_secs).await {
                return lines;
            }
        }

        // 2. Fuzzy search fallback (handles feat./remaster/album name differences)
        if let Ok(mut url) = Url::parse("https://lrclib.net/api/search") {
            url.query_pairs_mut()
                .append_pair("track_name", &track.title)
                .append_pair("artist_name", primary_artist);

            if let Ok(res) = self.http.get(url.as_str()).send().await {
                if res.status().is_success() {
                    if let Ok(Value::Array(items)) = res.json::<Value>().await {
                        // Prefer synced lyrics closest in duration, otherwise first plain lyrics
                        let mut best_synced: Option<(u32, Vec<LyricLine>)> = None;
                        let mut best_plain: Option<Vec<LyricLine>> = None;

                        for item in items {
                            let dur = item
                                .get("duration")
                                .and_then(Value::as_f64)
                                .unwrap_or(track.duration_secs as f64)
                                .round() as u32;
                            let diff = dur.abs_diff(track.duration_secs);

                            if let Some(synced) = item
                                .get("syncedLyrics")
                                .and_then(Value::as_str)
                                .filter(|s| !s.trim().is_empty())
                            {
                                let parsed = parse_lrc(synced);
                                if !parsed.is_empty()
                                    && best_synced.as_ref().is_none_or(|(d, _)| diff < *d)
                                {
                                    best_synced = Some((diff, parsed));
                                }
                            } else if best_plain.is_none() {
                                if let Some(plain) = item
                                    .get("plainLyrics")
                                    .and_then(Value::as_str)
                                    .filter(|s| !s.trim().is_empty())
                                {
                                    let parsed = parse_plain_lyrics(plain, track.duration_secs);
                                    if !parsed.is_empty() {
                                        best_plain = Some(parsed);
                                    }
                                }
                            }
                        }

                        if let Some((_, lines)) = best_synced {
                            return lines;
                        }
                        if let Some(lines) = best_plain {
                            return lines;
                        }
                    }
                }
            }
        }

        vec![LyricLine {
            time_secs: 0.0,
            text: format!("Lyrics not found for \"{}\".", track.title),
        }]
    }

    async fn request_single(&self, url: &str, duration_secs: u32) -> Option<Vec<LyricLine>> {
        let res = self.http.get(url).send().await.ok()?;
        if !res.status().is_success() {
            return None;
        }
        let val: Value = res.json().await.ok()?;
        if val.get("instrumental").and_then(Value::as_bool) == Some(true) {
            return Some(vec![LyricLine {
                time_secs: 0.0,
                text: "♪ Instrumental ♪".into(),
            }]);
        }
        if let Some(synced) = val
            .get("syncedLyrics")
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            let lines = parse_lrc(synced);
            if !lines.is_empty() {
                return Some(lines);
            }
        }
        if let Some(plain) = val
            .get("plainLyrics")
            .and_then(Value::as_str)
            .filter(|s| !s.trim().is_empty())
        {
            let lines = parse_plain_lyrics(plain, duration_secs);
            if !lines.is_empty() {
                return Some(lines);
            }
        }
        None
    }
}

/// Parses standard LRC format (`[mm:ss.xx] line` or multiple `[mm:ss.xx]` tags per line).
pub fn parse_lrc(raw: &str) -> Vec<LyricLine> {
    let mut out = Vec::new();

    for line in raw.lines() {
        let mut rest = line.trim();
        let mut timestamps = Vec::new();

        while let Some(stripped) = rest.strip_prefix('[') {
            let Some(end) = stripped.find(']') else {
                break;
            };
            let tag = &stripped[..end];
            if let Some(secs) = parse_lrc_timestamp(tag) {
                timestamps.push(secs);
            }
            rest = stripped[end + 1..].trim_start();
        }

        if timestamps.is_empty() {
            continue;
        }
        let text = if rest.is_empty() { "♪" } else { rest };
        for t in timestamps {
            out.push(LyricLine {
                time_secs: t,
                text: text.to_string(),
            });
        }
    }

    out.sort_by(|a, b| a.time_secs.total_cmp(&b.time_secs));
    out
}

fn parse_lrc_timestamp(tag: &str) -> Option<f32> {
    let (min_str, sec_str) = tag.split_once(':')?;
    let mins: f32 = min_str.trim().parse().ok()?;
    let secs: f32 = sec_str.trim().parse().ok()?;
    if mins < 0.0 || secs < 0.0 {
        return None;
    }
    Some(mins * 60.0 + secs)
}

pub fn parse_plain_lyrics(raw: &str, duration_secs: u32) -> Vec<LyricLine> {
    let non_empty: Vec<&str> = raw
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    if non_empty.is_empty() {
        return Vec::new();
    }
    let usable = (duration_secs as f32 * 0.88).max(20.0);
    let step = usable / non_empty.len() as f32;
    non_empty
        .into_iter()
        .enumerate()
        .map(|(i, text)| LyricLine {
            time_secs: 4.0 + i as f32 * step,
            text: text.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_synced_lrc_and_multi_timestamps() {
        let sample = "\
[ar:Nadhif Basalamah]
[00:12.50] Tak sadar ku temukan
[00:25.125][01:02.00] Kan ku arungi tujuh laut samudra
[00:18.00] ";
        let lines = parse_lrc(sample);
        assert_eq!(lines.len(), 4);
        assert!((lines[0].time_secs - 12.5).abs() < 0.01);
        assert_eq!(lines[0].text, "Tak sadar ku temukan");
        assert!((lines[1].time_secs - 18.0).abs() < 0.01);
        assert_eq!(lines[1].text, "♪");
        assert!((lines[2].time_secs - 25.125).abs() < 0.01);
        assert!((lines[3].time_secs - 62.0).abs() < 0.01);
    }

    #[test]
    fn parses_plain_lyrics_evenly() {
        let lines = parse_plain_lyrics("Line 1\n\nLine 2\nLine 3", 120);
        assert_eq!(lines.len(), 3);
        assert!(lines[0].time_secs < lines[1].time_secs);
        assert!(lines[1].time_secs < lines[2].time_secs);
    }
}
