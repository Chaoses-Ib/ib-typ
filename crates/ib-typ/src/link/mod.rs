use std::fmt;

use thiserror::Error;

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

#[derive(Debug, Clone)]
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
        let mut n = 0;
        {
            let mut it = it.clone();
            let mut a_uri = true;
            let mut b_uri = true;
            for [a, b] in &mut it {
                if !a.contains("://") {
                    a_uri = false;
                }
                if !b.contains("://") {
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
        for [title, uri] in it {
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
        assert_eq!(Link::try_from_uri_title_lines(s).unwrap().len(), 2);

        let s = "https://www.google.com/search?q=vscode.DocumentPasteEditProvider exmaple
vscode.DocumentPasteEditProvider exmaple - Google Search

https://github.com/microsoft/vscode-extension-samples/blob/main/document-paste/src/extension.ts
vscode-extension-samples/document-paste/src/extension.ts at main · microsoft/vscode-extension-samples
";
        assert_eq!(Link::try_from_uri_title_lines(s).unwrap().len(), 2);
    }
}
