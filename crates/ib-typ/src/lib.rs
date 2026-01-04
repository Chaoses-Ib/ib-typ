#![feature(str_split_remainder)]

pub mod time;
pub mod wasm;
pub mod www;

fn to_jinja(e: anyhow::Error) -> minijinja::Error {
    // TODO: with_source()
    minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string())
}
