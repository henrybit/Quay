use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::error::{AppError, AppResult};
use crate::models::ConnectionProfile;

fn store_path(app: &AppHandle) -> AppResult<PathBuf> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|err| AppError::msg(err.to_string()))?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("connections.json"))
}

pub fn load_all(app: &AppHandle) -> AppResult<Vec<ConnectionProfile>> {
    let path = store_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(path)?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&raw)?)
}

pub fn save_all(app: &AppHandle, items: &[ConnectionProfile]) -> AppResult<()> {
    let path = store_path(app)?;
    let raw = serde_json::to_string_pretty(items)?;
    std::fs::write(path, raw)?;
    Ok(())
}

pub fn upsert(app: &AppHandle, profile: ConnectionProfile) -> AppResult<ConnectionProfile> {
    let mut items = load_all(app)?;
    if let Some(existing) = items.iter_mut().find(|item| item.id == profile.id) {
        *existing = profile.clone();
    } else {
        items.push(profile.clone());
    }
    items.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    save_all(app, &items)?;
    Ok(profile)
}

pub fn remove(app: &AppHandle, id: &str) -> AppResult<()> {
    let mut items = load_all(app)?;
    items.retain(|item| item.id != id);
    save_all(app, &items)
}

pub fn get(app: &AppHandle, id: &str) -> AppResult<ConnectionProfile> {
    load_all(app)?
        .into_iter()
        .find(|item| item.id == id)
        .ok_or_else(|| AppError::msg("连接不存在"))
}
