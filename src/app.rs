use leptos::*;

use crate::components::header::HeaderView;
use crate::components::node_version_list::NodeVersionListView;
use crate::components::options::OptionsView;
use crate::error::StrError;
use crate::node::{Node, NodeStatus};
use crate::state::{DisplayMode, State};
use crate::tauri::{tauri_invoke, tauri_on};

#[component]
pub fn App() -> impl IntoView {
    provide_context(State::default());
    let state = use_context::<State>().expect("get state failed");

    let init = create_resource(
        || (),
        move |_| async move {
            tauri_invoke!(String, "node_init")
                .await
                .map_err(StrError::from)
                .map(|_| "".to_string())
        },
    );

    create_resource(
        || (),
        move |_| async move {
            tauri_on::<serde_json::Value>("node_list", move |event| {
                let payload = event.payload;
                if let Ok(nodes) = serde_json::from_value::<Vec<Node>>(payload) {
                    state.all_nodes.set(nodes);
                };
            })
            .await
        },
    );

    create_resource(
        || (),
        move |_| async move {
            tauri_on::<serde_json::Value>("node_local_versions", move |event| {
                let payload = event.payload;
                if let Ok(local_versions) = serde_json::from_value::<Vec<String>>(payload) {
                    state.local_versions.set(local_versions);
                };
            })
            .await
        },
    );

    create_resource(
        || (),
        move |_| async move {
            tauri_on::<serde_json::Value>("node_cur_version", move |event| {
                let payload = event.payload;
                if let Ok(cur_version) = serde_json::from_value::<String>(payload) {
                    state.cur_version.set(Some(cur_version));
                };
            })
            .await
        },
    );

    // update node status
    let _ = watch(
        move || {
            (
                state.all_nodes.get(),
                state.local_versions.get(),
                state.cur_version.get(),
            )
        },
        move |(all_nodes, local_versions, cur_version), _, _| {
            for node in all_nodes {
                let mut status = NodeStatus::Pendding;
                let version = &node.version;

                if local_versions.contains(version) {
                    status = NodeStatus::Ready;

                    if let Some(cur_version) = cur_version {
                        if version == cur_version {
                            status = NodeStatus::CurVer;
                        }
                    }
                }

                if status != node.status.get_untracked() {
                    node.status.set(status);
                }
            }
        },
        true,
    );

    // update node hidden
    let _ = watch(
        move || {
            (
                state.all_nodes.get(),
                state.display_mod.get(),
                state.local_versions.get(),
                state.filter_version.get(),
            )
        },
        move |(all_nodes, display_mode, local_versions, filter_version), _, _| {
            for node in all_nodes {
                let mut hidden = false;

                let version = &node.version;

                if !version.contains(filter_version) {
                    hidden = true;
                }

                if display_mode == &DisplayMode::Local && !local_versions.contains(version) {
                    hidden = true;
                }

                if hidden != node.hidden.get_untracked() {
                    node.hidden.set(hidden);
                }
            }
        },
        true,
    );

    view! {
        <main class="flex flex-col h-screen gap-6 bg-white p-4">
            <Suspense fallback=move || view! { <p class="m-auto text-3xl text-gray-400">"Loading..."</p> }>
                <ErrorBoundary
                    fallback=move |errors| view! {
                        <div class="m-auto flex flex-col max-h-[75%] w-2/3 text-red-500">
                            <div class="flex items-center justify-between">
                                <p class="text-2xl">Errors</p>
                                <button class="text-blue-500" on:click=move |_| init.refetch()>Retry</button>
                            </div>
                            <ul class="overflow-y-auto break-words text-xl text-red-400">
                                {move || errors.get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                    .collect_view()
                                }
                            </ul>
                        </div>
                    }>
                        {move || init.get()}
                        <HeaderView/>
                        <OptionsView/>
                        <NodeVersionListView/>
                </ErrorBoundary>
            </Suspense>
        </main>
    }
}
