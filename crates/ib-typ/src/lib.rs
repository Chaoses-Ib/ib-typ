#![feature(assert_matches)]
#![feature(iter_array_chunks)]
#![feature(str_split_remainder)]

pub mod link;
pub mod time;
#[cfg(feature = "wasm")]
pub mod wasm;
pub mod www;

fn to_jinja(e: anyhow::Error) -> minijinja::Error {
    // TODO: with_source()
    minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string())
}
