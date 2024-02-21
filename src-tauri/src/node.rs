use std::str::FromStr;

use globenv::set_path;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{api::dialog::blocking::confirm, AppHandle, Manager, State};
use tempfile::tempdir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_stream::StreamExt;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
    unpack::unpack,
};

mod lts {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum Lts {
        Bool(bool),
        Str(String),
    }

    impl From<Lts> for Option<String> {
        fn from(status: Lts) -> Self {
            match status {
                Lts::Bool(_) => None,
                Lts::Str(x) => Some(x),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Lts::deserialize(deserializer)?.into())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    pub version: String,
    #[serde(deserialize_with = "lts::deserialize")]
    pub lts: Option<String>,
    // pub files: Vec<String>,
}

#[tauri::command]
pub async fn node_init(app: AppHandle, state: State<'_, AppState>) -> AppResult<Value> {
    node_list(app.clone(), state.clone()).await?;
    node_local_versions(app.clone(), state.clone()).await?;
    node_cur_version(app, state).await?;
    Ok(serde_json::json!("success"))
}

#[tauri::command]
pub async fn node_list(app: AppHandle, state: State<'_, AppState>) -> AppResult<Value> {
    let mut ns = state.node_state.lock().await;
    let mut nodes = &ns.all;

    if nodes.is_empty() {
        let url = state.config.list_url();
        let res = reqwest::get(url).await?.json::<Vec<Node>>().await?;
        let res = res
            .into_iter()
            .filter(|node| {
                let mut res = false;
                if let Ok(version) = Version::from_str(&node.version.trim_start_matches('v')) {
                    if version.major >= 4 {
                        res = true;
                    } else {
                        let minors = [10u64, 12u64];
                        if version.major == 0 && minors.contains(&version.minor) {
                            res = true;
                        }
                    }
                }

                return res;
            })
            .collect::<Vec<_>>();
        dbg!(res.len());

        ns.all = res;
        nodes = &ns.all;
    }

    let json = serde_json::json!(nodes);

    app.emit_all("node_list", &json)?;

    Ok(json)
}

#[tauri::command]
pub async fn node_local_versions(app: AppHandle, state: State<'_, AppState>) -> AppResult<Value> {
    let mut res = tokio::fs::read_dir(&state.config.node_dir).await?;

    let mut local_versions = vec![];
    while let Some(entry) = res.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_dir() {
            if let Ok(dir_name) = entry.file_name().into_string() {
                let version = dir_name.trim_start_matches('v');
                if let Ok(_) = Version::from_str(version) {
                    local_versions.push(dir_name);
                }
            }
        }
    }

    let mut ns = state.node_state.lock().await;
    ns.local_versions = local_versions.clone();

    let json = serde_json::json!(local_versions);

    app.emit_all("node_local_versions", json.clone())?;

    Ok(json)
}

#[tauri::command]
pub async fn node_cur_version(app: AppHandle, state: State<'_, AppState>) -> AppResult<Value> {
    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&state.config.node_dir.join("version"))
        .await?;
    let mut version = Vec::new();
    file.read_to_end(&mut version).await?;
    let version = String::from_utf8(version)?;
    let json = serde_json::json!(version);

    if let Ok(_) = Version::from_str(version.trim_start_matches('v')) {
        let ns = state.node_state.lock().await;
        if ns.local_versions.contains(&version) {
            app.emit_all("node_cur_version", json.clone())?;
        }
    }

    Ok(json)
}

#[tauri::command]
pub async fn node_set_cur_version(
    version: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&state.config.node_dir.join("version"))
        .await?;

    file.write_all(version.as_bytes()).await?;

    set_path(r#"$HOME/.rnpm/$(cat "$HOME/.rnpm/version")/bin:$PATH"#)?;

    node_cur_version(app, state).await?;
    Ok(())
}

#[tauri::command]
pub async fn node_download(
    version: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> AppResult<()> {
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join(state.config.filename(&version));
    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .await?;

    let url = state.config.download_url(&version);
    let response = reqwest::Client::new().get(url).send().await?;
    let total = response.content_length().unwrap_or(0);
    let mut progress = 0u64;
    let mut body = response.bytes_stream();
    let event_name_progrss = format!("node_download:{}", version.replace(".", "-"));
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        progress += chunk.len() as u64;
        file.write_all(&chunk).await?;
        app.emit_all(
            &event_name_progrss,
            serde_json::json!({
                "total": total,
                "progress": progress
            }),
        )?
    }

    unpack(version, file_path, state.config.node_dir.clone())?;

    node_local_versions(app, state).await?;

    Ok(())
}

#[tauri::command]
pub async fn node_delete(
    version: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> AppResult<()> {
    let window = app
        .get_focused_window()
        .ok_or(AppError("no main window".to_string()))?;

    let message = format!("Will the local node version {version} be deleted?");
    let res = confirm(Some(&window), "confirm deletion？", message);

    if res {
        std::fs::remove_dir_all(state.config.node_dir.join(version))?;
        node_local_versions(app, state).await?;
    }

    Ok(())
}
