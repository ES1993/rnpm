use tauri::async_runtime::Mutex;

use crate::{config::Config, node::Node};

#[derive(Default)]
pub struct NodeState {
    pub all: Vec<Node>,
    pub local_versions: Vec<String>,
}

pub struct AppState {
    pub config: Config,
    pub node_state: Mutex<NodeState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            node_state: Mutex::new(NodeState::default()),
        }
    }
}
