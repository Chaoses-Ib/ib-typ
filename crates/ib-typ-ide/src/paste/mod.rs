use bon::Builder;
use ib_typ::{link::Link, plain};

use crate::{paste::kind::PasteEditKind, wasm::wasm_bindgen};

pub mod kind;

#[wasm_bindgen(js_namespace = ["paste"])]
#[derive(Clone, Debug)]
pub struct PasteEditProvider {
    pub link_list_to_tree_typ: bool,
    pub plain_note: bool,
}

#[wasm_bindgen(getter_with_clone, js_namespace = ["paste"])]
#[derive(Builder, Clone, Debug)]
#[builder(on(String, into))]
pub struct PasteEdit {
    /// The text to insert at the pasted locations.
    #[wasm_bindgen(readonly)]
    pub text: String,

    /// Human readable label that describes the edit.
    #[wasm_bindgen(readonly)]
    pub title: String,

    #[builder(default)]
    #[wasm_bindgen(readonly)]
    pub kind: PasteEditKind,

    /// Controls ordering when multiple paste edits can potentially be applied.
    ///
    /// If this edit yields to another, it will be shown lower in the list of possible paste edits shown to the user.
    #[builder(default)]
    #[wasm_bindgen(readonly)]
    pub yield_to: Vec<PasteEditKind>,
}

#[wasm_bindgen]
impl PasteEditProvider {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            link_list_to_tree_typ: true,
            plain_note: true,
        }
    }

    /// This is used to filter out providers when a specific kind of edit is requested.
    pub fn kinds(&self) -> Vec<PasteEditKind> {
        PasteEditKind::IB_KINDS.into()
    }

    pub fn provide_edits(&self, text: &str) -> Vec<PasteEdit> {
        let mut edits = Vec::new();

        if self.link_list_to_tree_typ
            && let Ok(links) = Link::try_from_uri_title_lines(text)
        {
            edits.push(
                PasteEdit::builder()
                    .text(ib_typ::link::tree::link_list_to_tree_typ(&links))
                    .title("Link List to Typst Tree")
                    .kind(PasteEditKind::TYPST_IB_LINK_LIST_TO_TREE)
                    .build(),
            );
        }

        // Plain note conversion
        if self.plain_note {
            edits.push(
                PasteEdit::builder()
                    .text(plain::plain_to_typ(text))
                    .title("Plain Text Note to Typst")
                    .kind(PasteEditKind::TYPST_IB_PLAIN)
                    .build(),
            );
        }

        edits
    }
}
