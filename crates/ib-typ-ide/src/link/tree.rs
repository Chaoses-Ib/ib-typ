#[cfg(feature = "wasm-extra")]
pub mod wasm {
    use ib_typ::link::Link;

    use crate::wasm::*;

    #[wasm_bindgen(js_namespace = ["link", "tree"])]
    pub fn title_uri_link_list_to_tree_typ(s: &str) -> Result<String, String> {
        let links = Link::try_from_uri_title_lines(s).anyhow()?;
        Ok(ib_typ::link::tree::link_list_to_tree_typ(&links))
    }
}
