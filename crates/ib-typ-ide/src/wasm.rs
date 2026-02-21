pub(crate) use wasm_bindgen::prelude::*;

pub trait StdAnyhow<T> {
    fn anyhow(self) -> Result<T, String>;
}

impl<T, E> StdAnyhow<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn anyhow(self) -> Result<T, String> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub trait Anyhow<T> {
    fn anyhow(self) -> Result<T, String>;
}

impl<T> Anyhow<T> for Result<T, anyhow::Error> {
    fn anyhow(self) -> Result<T, String> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }
}
