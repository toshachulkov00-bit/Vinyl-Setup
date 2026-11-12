#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager as SessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

#[derive(Serialize)]
struct MediaState {
    available: bool,
    title: String,
    artist: String,
    playing: bool,
    position: f64,
    duration: f64,
}

#[tauri::command]
async fn media_state() -> Result<MediaState, String> {
    let manager = SessionManager::RequestAsync()
        .map_err(|e| e.to_string())?
        .await
        .map_err(|e| e.to_string())?;
    let session = match manager.GetCurrentSession() {
        Ok(s) => s,
        Err(_) => return Ok(MediaState { available:false, title:String::new(), artist:String::new(), playing:false, position:0.0, duration:0.0 }),
    };
    let props = session.TryGetMediaPropertiesAsync()
        .map_err(|e| e.to_string())?
        .await
        .map_err(|e| e.to_string())?;
    let playback = session.GetPlaybackInfo().map_err(|e| e.to_string())?;
    let playing = playback.PlaybackStatus().map_err(|e| e.to_string())?
        == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;
    let timeline = session.GetTimelineProperties().map_err(|e| e.to_string())?;
    let position = timeline.Position().map(|v| v.Duration as f64 / 10_000_000.0).unwrap_or(0.0);
    let duration = timeline.EndTime().map(|v| v.Duration as f64 / 10_000_000.0).unwrap_or(0.0);
    Ok(MediaState {
        available: true,
        title: props.Title().map(|v| v.to_string()).unwrap_or_default(),
        artist: props.Artist().map(|v| v.to_string()).unwrap_or_default(),
        playing, position, duration,
    })
}

#[tauri::command]
async fn media_control(action: String) -> Result<(), String> {
    let manager = SessionManager::RequestAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?;
    let session = manager.GetCurrentSession().map_err(|e| e.to_string())?;
    match action.as_str() {
        "toggle" => { session.TryTogglePlayPauseAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?; }
        "next" => { session.TrySkipNextAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?; }
        "previous" => { session.TrySkipPreviousAsync().map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?; }
        _ => return Err("Unknown media action".into()),
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![media_state, media_control])
        .run(tauri::generate_context!())
        .expect("error while running Vinyl");
}