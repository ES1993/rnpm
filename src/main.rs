mod app;
mod components;
mod error;
mod node;
mod state;
mod tauri;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
