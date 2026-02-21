use std::fmt;

use thiserror::Error;
use url::Url;

pub mod tree;

#[derive(Error, Debug)]
pub enum LinksError {
    #[error("neither uri")]
    NeitherUri,
    #[error("both uri")]
    BothUri,
    #[error("invalid link")]
    InvalidLink(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub uri: String,
    pub title: String,
}

impl Link {
    pub fn new(uri: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            title: title.into(),
        }
    }

    /// Parse links from URI + title lines.
    /// - Both URI + title and title + URI are supported.
    pub fn try_from_uri_title_lines(s: &str) -> Result<Vec<Link>, LinksError> {
        let it = s.lines().filter(|l| !l.is_empty()).array_chunks();

        // Deduce & validate
        let mut b_uri = true;
        let mut n = 0;
        {
            let mut it = it.clone();
            let mut a_uri = true;
            for [a, b] in &mut it {
                // Fast fail || Strict validate
                if !a.contains("://") || Url::try_from(a).is_err() {
                    a_uri = false;
                }
                if !b.contains("://") || Url::try_from(b).is_err() {
                    b_uri = false;
                }
                if !a_uri && !b_uri {
                    return Err(LinksError::NeitherUri);
                }
                n += 1;
            }
            if let Some(rem) = it.into_remainder().and_then(|mut rem| rem.next()) {
                return Err(LinksError::InvalidLink(rem.into()));
            }
            if a_uri && b_uri {
                return Err(LinksError::BothUri);
            }
        }

        let mut links = Vec::with_capacity(n);
        for [mut title, mut uri] in it {
            if !b_uri {
                core::mem::swap(&mut title, &mut uri);
            }
            links.push(Link::new(uri, title));
        }
        Ok(links)
    }

    pub fn display(&self) -> LinkDisplay<'_> {
        LinkDisplay { link: self }
    }
}

pub struct LinkDisplay<'a> {
    link: &'a Link,
}

impl fmt::Display for LinkDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#a[{}][{}]", self.link.title, self.link.uri)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn try_from_uri_title_lines() {
        assert_matches!(
            Link::try_from_uri_title_lines("abcd"),
            Err(LinksError::InvalidLink(s)) if s == "abcd"
        );

        let s = "vscode.DocumentPasteEditProvider exmaple - Google Search
https://www.google.com/search?q=vscode.DocumentPasteEditProvider exmaple

vscode-extension-samples/document-paste/src/extension.ts at main · microsoft/vscode-extension-samples
https://github.com/microsoft/vscode-extension-samples/blob/main/document-paste/src/extension.ts
";
        let links = Link::try_from_uri_title_lines(s).unwrap();
        dbg!(&links);
        assert_eq!(links.len(), 2);
        assert_eq!(
            links[0].title,
            "vscode.DocumentPasteEditProvider exmaple - Google Search"
        );

        let s = "https://www.google.com/search?q=vscode.DocumentPasteEditProvider exmaple
vscode.DocumentPasteEditProvider exmaple - Google Search

https://github.com/microsoft/vscode-extension-samples/blob/main/document-paste/src/extension.ts
vscode-extension-samples/document-paste/src/extension.ts at main · microsoft/vscode-extension-samples
";
        let links2 = Link::try_from_uri_title_lines(s).unwrap();
        assert_eq!(links, links2);

        // Both contain ://
        let s = "https://www.google.com/search?q=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DSNcpoS4cs_g
https://www.youtube.com/watch?v=SNcpoS4cs_g - Google Search
https://www.youtube.com/watch?v=SNcpoS4cs_g
【#RKMusic歌枠リレー】大祝福【CULUA】 - YouTube
";
        let links = Link::try_from_uri_title_lines(s).unwrap();
        dbg!(&links);
        assert_eq!(links.len(), 2);
        assert_eq!(
            links[1].title,
            "【#RKMusic歌枠リレー】大祝福【CULUA】 - YouTube"
        );
    }
}
