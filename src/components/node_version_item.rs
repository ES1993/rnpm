use leptos::*;

use crate::{
    node::NodeStatus,
    tauri::{tauri_invoke, tauri_on},
};

#[component]
pub fn NodeVersionItemView(
    version: String,
    lts: Option<String>,
    hidden: RwSignal<bool>,
    status: RwSignal<NodeStatus>,
) -> impl IntoView {
    let ver = version.clone();
    create_resource(
        || (),
        move |_| {
            let version = ver.to_owned();
            let event_name_progrss = format!("node_download:{}", version.replace('.', "-"));
            async move {
                tauri_on::<serde_json::Value>(&event_name_progrss, move |event| {
                    let payload = event.payload;
                    let total = payload["total"].as_u64().unwrap_or(0);
                    let progress = payload["progress"].as_u64().unwrap_or(0);
                    let progress = (progress * 100 / total) as usize;
                    if status.get_untracked() != NodeStatus::Downloading(progress) {
                        status.set(NodeStatus::Downloading(progress));
                    };
                })
                .await
            }
        },
    );

    let ver = version.clone();
    let download = create_action(move |_: &()| {
        status.set(NodeStatus::Downloading(0));
        let ver = ver.to_owned();
        async move {
            if tauri_invoke!("node_download", &serde_json::json!({"version": ver}))
                .await
                .is_err()
            {
                status.set(NodeStatus::Pendding);
            }
        }
    });

    let ver = version.clone();
    let delete = create_action(move |_: &()| {
        let ver = ver.to_owned();
        async move {
            let _ = tauri_invoke!("node_delete", &serde_json::json!({"version": ver})).await;
        }
    });

    let ver = version.clone();
    let set_cur_version = create_action(move |_: &()| {
        let ver = ver.to_owned();
        async move {
            let _ =
                tauri_invoke!("node_set_cur_version", &serde_json::json!({"version": ver})).await;
        }
    });

    view! {
        <div
            class="flex cursor-default items-center gap-2 rounded-xl bg-gray-50 p-3 hover:bg-gray-100"
            class=("hidden", hidden)>

            <div class="text-2xl text-blue-500">{version}</div>
            <div class="rounded-sm bg-blue-500 px-1 text-white">{lts}</div>

            <div class="ml-auto mr-2 flex flex-row gap-3 *:hover:cursor-pointer">
                <Show when=move || status.get() == NodeStatus::Pendding>
                    <div on:click=move|_|download.dispatch(())>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3" />
                        </svg>
                    </div>
                </Show>

                {move || match status.get() {
                    NodeStatus::Downloading(progress) => Some(view! {
                        <div class="text-0.5xl">{progress}%</div>
                    }.into_view()),
                    _ => None,
                }}

                <Show when=move || status.get() == NodeStatus::Ready>
                    <div on:click=move|_|delete.dispatch(())>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                        </svg>
                    </div>

                    <div on:click=move|_|set_cur_version.dispatch(())>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z" />
                        </svg>
                    </div>
                </Show>

                <Show when=move || status.get() == NodeStatus::CurVer>
                    <div>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-5 w-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
                        </svg>
                    </div>
                </Show>
            </div>
        </div>
    }
}
