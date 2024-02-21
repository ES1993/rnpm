use leptos::*;

use crate::state::{DisplayMode, State};

#[component]
pub fn OptionsView() -> impl IntoView {
    let state = use_context::<State>().expect("get state failed");

    let update_filter_version = move |ev| {
        let v = event_target_value(&ev);
        state.filter_version.set(v);
    };

    view! {
        <div class="flex flex-row items-center gap-6 self-center">
            <input
                class="rounded-md border-2 border-gray-400 px-3 py-1 text-gray-700 outline-none focus:border-blue-400"
                placeholder="version filter"
                prop:value=state.filter_version
                on:input=update_filter_version />

            <div class="flex flex-row *:flex *:w-24 *:items-center *:justify-center *:border-2 *:text-gray-500 [&>div+div]:-ml-0.5 [&>div:first-child]:rounded-bl-md [&>div:first-child]:rounded-tl-md [&>div:has(input:checked)]:z-10 [&>div:has(input:checked)]:border-blue-500 [&>div:has(input:checked)]:text-blue-500 [&>div:last-child]:rounded-br-md [&>div:last-child]:rounded-tr-md [&_input]:hidden">
                <div>
                    <input
                        type="radio"
                        id="Remote"
                        name="selected-mod-radio-group"
                        checked=move||state.display_mod.get() == DisplayMode::Remote
                        on:input=move|_|state.display_mod.set(DisplayMode::Remote) />

                    <label for="Remote">Remote</label>
                </div>

                <div>
                    <input
                        type="radio"
                        id="Local"
                        name="selected-mod-radio-group"
                        checked=move||state.display_mod.get() == DisplayMode::Local
                        on:input=move|_|state.display_mod.set(DisplayMode::Local) />

                    <label for="Local">Local</label>
                </div>
            </div>
        </div>
    }
}
