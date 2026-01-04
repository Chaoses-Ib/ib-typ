use url::Url;

pub mod community;

/// - `www.` subdomain is skipped.
pub fn uri_host_ancestors_try_for_each<R>(
    url: &Url,
    mut f: impl FnMut(&Url, &str) -> Option<R>,
) -> Option<R> {
    let domain = url.domain()?;
    let domain = domain.strip_prefix("www.").unwrap_or(domain);

    if let Some(c) = f(url, domain) {
        return Some(c);
    }

    let mut it = domain.split('.');
    while let Some(_s) = it.next() {
        let Some(rem) = it.remainder() else {
            continue;
        };
        if let Some(c) = f(url, rem) {
            return Some(c);
        }
    }

    None
}

pub fn uri_media(url: &Url) -> String {
    if let Some(com) = community::uri_community(url) {
        return com;
    }
    let auth = url.authority();
    let auth = auth.strip_prefix("www.").unwrap_or(auth);
    auth.into()
}

pub fn uri_media_str(url: &str) -> String {
    match url.try_into() {
        Ok(url) => uri_media(&url),
        Err(url::ParseError::RelativeUrlWithoutBase) => uri_media_str(&format!("http://{url}")),
        Err(_) => url.into(),
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use crate::wasm::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn uri_media(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s = str::from_utf8(s)?;
        to_bytes!(uri_media_str(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_media_() {
        assert_eq!(uri_media_str("www.example.org"), "example.org");
        assert_eq!(uri_media_str("https://www.example.org"), "example.org");
    }
}
