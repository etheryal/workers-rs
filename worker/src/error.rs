use thiserror::Error;
use wasm_bindgen::{prelude::*, JsValue};

/// All possible Error variants that might be encountered while working with a Worker.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("content-type mismatch")]
    BadEncoding,

    #[error("body has already been read")]
    BodyUsed,

    #[error("{0} (status: {1})")]
    Json(String, u16),

    #[error("Javascript error: {0}")]
    JsError(String),

    #[error("no binding found for `{0}`")]
    BindingError(String),

    #[error("route has no corresponding shared data")]
    RouteNoDataError,

    #[error("invalid status code: {0}")]
    InvalidStatusCode(u16),

    #[error("failed to insert route: {0}")]
    RouteInsertError(#[from] matchit::InsertError),

    #[error("Serde Error: {0}")]
    RustError(String),

    #[error("Serde Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Serde WASM bindgen Error: {0}")]
    SerdeWasmBindgenError(String),

    #[error("Kv Error: {0}")]
    KvError(String),

    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

impl From<Error> for JsValue {
    fn from(e: Error) -> Self {
        JsValue::from_str(&e.to_string())
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::SerdeWasmBindgenError(e.to_string())
    }
}

impl From<worker_kv::KvError> for Error {
    fn from(e: worker_kv::KvError) -> Self {
        Error::KvError(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(a: &str) -> Self {
        Error::RustError(a.to_string())
    }
}

impl From<String> for Error {
    fn from(a: String) -> Self {
        Error::RustError(a.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = String)]
            pub fn to_string(value: &JsValue) -> String;
        }

        Error::JsError(to_string(&value))
    }
}
