use leptos::*;

use crate::components::node_version_item::NodeVersionItemView;
use crate::state::State;

#[component]
pub fn NodeVersionListView() -> impl IntoView {
    let state = use_context::<State>().expect("get state failed");

    view! {
        <div class="grid grid-cols-3 gap-3 overflow-y-auto flex-1 auto-rows-min">
            <For
                each=move || state.all_nodes.get().into_iter()
                key=|node| node.version.clone()
                let:child>

                <NodeVersionItemView
                    version=child.version
                    lts=child.lts
                    hidden=child.hidden
                    status=child.status />
            </For>
        </div>
    }
}
