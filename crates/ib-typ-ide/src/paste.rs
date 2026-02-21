use ib_typ::link::Link;

use crate::wasm::wasm_bindgen;

#[wasm_bindgen(js_namespace = ["paste"])]
#[derive(Clone, Debug)]
pub struct PasteEditProvider {
    pub link_list_to_tree_typ: bool,
}

#[wasm_bindgen(getter_with_clone, js_namespace = ["paste"])]
#[derive(Clone, Debug)]
pub struct PasteEdit {
    /// The text to insert at the pasted locations.
    #[wasm_bindgen(readonly)]
    pub text: String,

    /// Human readable label that describes the edit.
    #[wasm_bindgen(readonly)]
    pub title: String,
}

#[wasm_bindgen]
impl PasteEditProvider {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            link_list_to_tree_typ: true,
        }
    }

    pub fn provide_edits(&self, text: &str) -> Vec<PasteEdit> {
        let mut edits = Vec::new();

        if self.link_list_to_tree_typ
            && let Ok(links) = Link::try_from_uri_title_lines(text)
        {
            edits.push(PasteEdit {
                text: ib_typ::link::tree::link_list_to_tree_typ(&links),
                title: "Link list to Typst tree".into(),
            });
        }

        edits
    }
}
