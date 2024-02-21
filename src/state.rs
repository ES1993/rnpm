use leptos::RwSignal;

use crate::node::Node;

#[derive(Clone, PartialEq, Debug)]
pub enum DisplayMode {
    Remote,
    Local,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Remote
    }
}

#[derive(Default, Clone)]
pub struct State {
    pub all_nodes: RwSignal<Vec<Node>>,
    pub local_versions: RwSignal<Vec<String>>,
    pub cur_version: RwSignal<Option<String>>,
    pub filter_version: RwSignal<String>,
    pub display_mod: RwSignal<DisplayMode>,
}
