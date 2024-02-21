use leptos::*;

use crate::state::State;

#[component]
pub fn HeaderView() -> impl IntoView {
    let state = use_context::<State>().expect("get state failed");

    view! {
        <div class="self-center text-4xl text-blue-500 empty:before:content-['nothing'] empty:before:text-yellow-400">
            {state.cur_version}
        </div>
    }
}
