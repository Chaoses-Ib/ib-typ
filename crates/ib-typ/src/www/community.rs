use url::Url;

use crate::www::uri_host_ancestors_try_for_each;

fn uri_community_host(url: &Url, host: &str) -> Option<String> {
    match host {
        "reddit.com" => reddit_com(url),
        _ => None,
    }
}

pub fn uri_community(url: &Url) -> Option<String> {
    uri_host_ancestors_try_for_each(url, uri_community_host)
}

pub fn uri_community_str(url: &str) -> Option<String> {
    uri_community(&url.try_into().ok()?)
}

pub fn reddit_com(url: &Url) -> Option<String> {
    let mut segs = url.path_segments()?;
    if let Some("r") = segs.next() {
        return segs.next().map(|sub| format!("r/{sub}"));
    }
    None
}

#[cfg(feature = "wasm-extra")]
pub mod wasm {
    use super::*;
    use crate::wasm::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn uri_community(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s = str::from_utf8(s)?;
        to_bytes!(uri_community_str(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_community_() {
        assert_eq!(
            uri_community_str(
                "https://www.reddit.com/r/StableDiffusion/comments/1p1mmm7/comfyscript_v060_simpler_to_use/"
            ).as_deref(),
            Some("r/StableDiffusion")
        );
        assert_eq!(uri_community_str("https://old.reddit.com/"), None);
    }
}
