use leptos::RwSignal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeStatus {
    Pendding,
    Downloading(usize),
    Ready,
    CurVer,
}

#[test]
fn test_node_status_ord() {
    assert!(NodeStatus::Pendding < NodeStatus::Downloading(1));
    assert!(NodeStatus::Downloading(11) < NodeStatus::Downloading(22));
    assert!(NodeStatus::Downloading(33) < NodeStatus::Ready);
    assert!(NodeStatus::Ready < NodeStatus::CurVer);
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::Pendding
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
    pub version: String,
    pub lts: Option<String>,
    #[serde(skip)]
    pub hidden: RwSignal<bool>,
    #[serde(skip)]
    pub status: RwSignal<NodeStatus>,
}
