use std::fmt::Write;

use url::Url;

use crate::{
    link::Link,
    www::search::{get_search_query_from_title, is_search_uri},
};

pub fn link_list_to_tree_typ(links: &[Link]) -> String {
    let mut typ = String::new();
    for i in 0..links.len() {
        let link = &links[i];
        let uri: Url = match link.uri.as_str().try_into() {
            Ok(uri) => uri,
            Err(_) => {
                writeln!(typ, "- {}", link.display()).unwrap();
                continue;
            }
        };

        if is_search_uri(&uri) {
            let query = get_search_query_from_title(&link.title);
            if !typ.is_empty() {
                writeln!(typ).unwrap();
            }
            writeln!(typ, "{query}:").unwrap();
            continue;
        }

        writeln!(typ, "- {}", link.display()).unwrap();
    }
    typ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_list_to_tree_typ_() {
        let s = "テスト - Google Search
https://www.google.com/search?q=テスト

Test - Wikipedia
https://en.wikipedia.org/wiki/Test

Speedtest by Ookla - The Global Broadband Speed Test
https://www.speedtest.net/

【心理テスト】ロリコン度チェック | 無料占いの決定版 GoisuNet
https://goisu.net/cgi-bin/psychology/psychology.cgi?menu=c021
";
        let typ = link_list_to_tree_typ(&Link::try_from_uri_title_lines(s).unwrap());
        assert_eq!(typ, "テスト:
- #a[Test - Wikipedia][https://en.wikipedia.org/wiki/Test]
- #a[Speedtest by Ookla - The Global Broadband Speed Test][https://www.speedtest.net/]
- #a[【心理テスト】ロリコン度チェック | 無料占いの決定版 GoisuNet][https://goisu.net/cgi-bin/psychology/psychology.cgi?menu=c021]
", "{typ}");

        let s = "test - Google Search
https://www.google.com/search?q=test

Test - Wikipedia
https://en.wikipedia.org/wiki/Test

テスト - Google Search
https://www.google.com/search?q=テスト

【心理テスト】ロリコン度チェック | 無料占いの決定版 GoisuNet
https://goisu.net/cgi-bin/psychology/psychology.cgi?menu=c021
";
        let typ = link_list_to_tree_typ(&Link::try_from_uri_title_lines(s).unwrap());
        assert_eq!(typ, "test:
- #a[Test - Wikipedia][https://en.wikipedia.org/wiki/Test]

テスト:
- #a[【心理テスト】ロリコン度チェック | 無料占いの決定版 GoisuNet][https://goisu.net/cgi-bin/psychology/psychology.cgi?menu=c021]
", "{typ}");
    }
}
