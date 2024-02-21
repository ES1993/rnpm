// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod node;
mod state;
mod unpack;

use state::AppState;
use tauri::generate_handler;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(generate_handler![
            node::node_init,
            node::node_list,
            node::node_local_versions,
            node::node_cur_version,
            node::node_set_cur_version,
            node::node_download,
            node::node_delete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
