// #[cfg(all(not(test), feature = "wasm"))]
pub use wasm_minimal_protocol::initiate_protocol;

#[cfg(any(test, not(feature = "wasm")))]
pub use nop_macros::nop as wasm_func;
#[cfg(all(not(test), feature = "wasm"))]
pub use wasm_minimal_protocol::wasm_func;

pub use crate::to_bytes;

#[macro_export]
macro_rules! to_bytes {
    ($r:expr) => {{
        let mut bytes = Vec::new();
        ciborium::into_writer(&$r, &mut bytes)?;
        Ok(bytes)
    }};
}
