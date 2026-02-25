use derive_more::From;

use crate::wasm::wasm_bindgen;

#[wasm_bindgen(js_namespace = ["paste"])]
#[derive(Clone, Debug, From, Default)]
pub struct PasteEditKind(#[wasm_bindgen(readonly)] pub &'static str);

impl PasteEditKind {
    pub const EMPTY: Self = PasteEditKind("");

    /// - "Insert Plain Text"
    pub const TEXT: Self = PasteEditKind("text");

    pub const TEXT_UPDATE_IMPORTS: Self = PasteEditKind("text.updateImports");
}

/// Built-in but undocumented.
impl PasteEditKind {
    pub const TEXT_PLAIN: Self = PasteEditKind("text.plain");

    pub const HTML: Self = PasteEditKind("html");

    pub const URI_PATH_ABSOLUTE: Self = PasteEditKind("uri.path.absolute");
    pub const URI_PATH_RELATIVE: Self = PasteEditKind("uri.path.relative");

    pub const CHAT_ATTACH_TEXT: Self = PasteEditKind("chat.attach.text");
    pub const CHAT_ATTACH_IMAGE: Self = PasteEditKind("chat.attach.image");
    pub const CHAT_ATTACH_ATTACHMENTS: Self = PasteEditKind("chat.attach.attachments");
}

/// TinyMist
///
/// https://github.com/Myriad-Dreamin/tinymist/blob/017c40dd22a991bb6c98b863406e8e5a2bad7736/editors/vscode/src/features/drop-paste.def.ts
impl PasteEditKind {
    /// Base kind for any sort of markdown link, including both path and media links
    pub const TYPST_LINK: Self = PasteEditKind("typst.link");

    /// Kind for normal markdown links, i.e. include "path/to/file.typ"
    pub const TYPST_LINK_URI: Self = PasteEditKind("typst.link.uri");

    pub const TYPST_LINK_IMAGE: Self = PasteEditKind("typst.link.image");

    pub const TYPST_LINK_IMAGE_ATTACHMENT: Self = PasteEditKind("typst.link.image.attachment");
}

/// ib-typ
impl PasteEditKind {
    pub const IB_KINDS: [Self; 1] = [Self::TYPST_IB_LINK_LIST_TO_TREE];

    pub const TYPST_IB_LINK_LIST_TO_TREE: Self = PasteEditKind("typst.ib.link.list_to_tree");
}
