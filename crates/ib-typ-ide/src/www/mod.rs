#[cfg(feature = "wasm")]
pub mod wasm {
    use crate::wasm::*;

    #[wasm_bindgen(js_namespace = www)]
    pub fn uri_media(s: &str) -> String {
        ib_typ::www::uri_media_str(s)
    }
}
