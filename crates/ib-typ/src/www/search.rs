use url::Url;

use crate::www::uri_host_ancestors_try_for_each;

static SEARCH_ENGINES: &[&str] = &["google.com"];

pub fn is_host_search_engine(uri: &Url) -> bool {
    uri_host_ancestors_try_for_each(uri, |_uri, host| {
        SEARCH_ENGINES.iter().any(|&e| e == host).then_some(())
    })
    .is_some()
}

pub fn is_search_uri(uri: &Url) -> bool {
    is_host_search_engine(uri) && uri.path() == "/search"
}

pub fn get_search_query_from_title(title: &str) -> &str {
    title.strip_suffix(" - Google Search").unwrap_or(title)
}
