use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke, catch)]
    async fn invoke_base(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke, catch)]
    async fn invoke_args(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen, catch)]
    async fn on(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
pub struct TauriOn {
    #[serde(skip)]
    unlisten: js_sys::Function,
}

impl Drop for TauriOn {
    fn drop(&mut self) {
        leptos::logging::log!("Calling unlisten for listen callback");
        self.unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event<T> {
    pub event: String,
    pub window_label: Option<String>,
    pub payload: T,
}

pub async fn tauri_on<T>(
    event: &str,
    mut handler: impl FnMut(Event<T>) + 'static,
) -> AppResult<TauriOn>
where
    T: DeserializeOwned,
{
    let closure = Closure::new(move |raw| {
        match from_value::<Event<T>>(raw) {
            Ok(value) => handler(value),
            Err(error) => leptos::logging::error!("{error}"),
        };
    });

    match on(event, &closure).await {
        Ok(unlisten) => {
            closure.forget();
            Ok(TauriOn {
                unlisten: js_sys::Function::from(unlisten),
            })
        }
        Err(e) => Err(AppError::from_str(e.as_string().unwrap_or("".to_string()))),
    }
}

#[allow(dead_code)]
pub async fn handle_invoke_base<T>(cmd: &str) -> AppResult<T>
where
    T: DeserializeOwned,
{
    match invoke_base(cmd).await {
        Ok(r) => Ok(from_value::<T>(r)?),
        Err(e) => Err(AppError::from_str(e.as_string().unwrap_or("".to_string()))),
    }
}

#[allow(dead_code)]
pub async fn handle_invoke_args<T>(cmd: &str, args: &Value) -> AppResult<T>
where
    T: DeserializeOwned,
{
    let args = js_sys::JSON::parse(args.to_string().as_str())
        .map_err(|e| AppError::from_str(format!("handle_invoke_args parse args error:{:?}", e)))?;

    match invoke_args(cmd, args).await {
        Ok(r) => Ok(from_value::<T>(r)?),
        Err(e) => Err(AppError::from_str(e.as_string().unwrap_or("".to_string()))),
    }
}

macro_rules! tauri_invoke {
    ($cmd: expr) => {
        crate::tauri::handle_invoke_base::<()>($cmd)
    };

    ($ty: ty, $cmd: expr) => {
        crate::tauri::handle_invoke_base::<$ty>($cmd)
    };

    ($cmd: expr, $args: expr) => {
        crate::tauri::handle_invoke_args::<()>($cmd, $args)
    };

    ($ty: ty, $cmd: expr, $args: expr) => {
        crate::tauri::handle_invoke_args::<$ty>($cmd, $args)
    };
}

pub(crate) use tauri_invoke;

use crate::error::{AppError, AppResult};
