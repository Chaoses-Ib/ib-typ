/*!
Plain text note format.

Mainly for inputing without a full keyboard, i.e. on phones/pads.

The Typst conversion result is not guaranteed to be semantically correct and needs manual check.
*/
use std::fmt::Write;

use bon::Builder;
use logos::Logos;

/// Token for plain note format
#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlainNoteToken {
    /// Newline sequence
    #[regex("\r?\n")]
    Newline,

    /// Time in format like `1:30`, `0:00`, `23:59`
    #[regex(r"\d?\d:\d\d")]
    Time,

    /// Anything else (text, numbers, symbols)
    #[regex(r".")]
    Other,
}

/// Convert an plain note into Typst code.
///
/// ## Returns
/// The Typst conversion result is not guaranteed to be semantically correct and needs manual check.
pub fn plain_to_typ(input: &str) -> String {
    PlainToTyp::builder().build().to_typ(input)
}

#[derive(Builder)]
pub struct PlainToTyp {
    /// By default, `trailing_newline` is enabled for multi-line `text`.
    ///
    /// TODO: Lookahead
    trailing_newline: Option<bool>,
}

impl PlainToTyp {
    /// ## Returns
    /// - `Some((typ, true))`: Likely plain.
    /// - `Some((typ, false))`: Maybe plain.
    /// - `None`: Likely Typst.
    pub fn detect_and_to_typ(&self, s: &str) -> Option<(String, bool)> {
        if s.contains("#t[") || s.contains(" \\\n") || s.contains(" \\\r\n") {
            return None;
        }
        let typ = self.to_typ(s);
        // typ == s || typ == s + "\n"
        if typ.starts_with(s) {
            return None;
        }
        let mut lex = PlainNoteToken::lexer(s);
        while let Some(Ok(token)) = lex.next() {
            match token {
                PlainNoteToken::Newline => (),
                PlainNoteToken::Time => return Some((typ, true)),
                PlainNoteToken::Other => (),
            }
        }
        Some((typ, false))
    }

    /// Convert an plain note into Typst code.
    ///
    /// ## Returns
    /// The Typst conversion result is not guaranteed to be semantically correct and needs manual check.
    pub fn to_typ(&self, text: &str) -> String {
        if text.is_empty() {
            return Default::default();
        }
        let mut result = String::with_capacity(text.len());
        let mut lex = PlainNoteToken::lexer(text);
        let mut last_token = PlainNoteToken::Other;
        while let Some(Ok(token)) = lex.next() {
            match token {
                PlainNoteToken::Newline => {
                    result += match last_token {
                        // Consecutive line feeds
                        PlainNoteToken::Newline => "\n",
                        // Line feed after whole-line time
                        PlainNoteToken::Time => "\n\n",
                        // Line feed
                        PlainNoteToken::Other => " \\\n",
                    };
                }
                PlainNoteToken::Time => {
                    // Leading line feed for whole-line time
                    if last_token == PlainNoteToken::Newline {
                        result += "\n";
                    }
                    write!(result, "#t[{}]", lex.slice()).unwrap();
                }
                PlainNoteToken::Other => result += lex.slice(),
            }
            last_token = token;
        }
        // Trailing new line
        if self.trailing_newline.unwrap_or_else(|| text.contains('\n'))
            && last_token != PlainNoteToken::Newline
        {
            result += "\n";
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newline() {
        assert_eq!(plain_to_typ("a\nb"), "a \\\nb\n");
        assert_eq!(
            plain_to_typ("1:30\n9:00"),
            "#t[1:30]


#t[9:00]\n"
        );
        assert_eq!(
            plain_to_typ(
                "13:44
3D アンシャ
15:03"
            ),
            r"#t[13:44]

3D アンシャ \

#t[15:03]
"
        );
    }

    #[test]
    fn time() {
        assert_eq!(plain_to_typ("1:30"), "#t[1:30]");
        assert_eq!(plain_to_typ("meeting at 9:00"), "meeting at #t[9:00]");
        assert_eq!(plain_to_typ("1:00 to 2:00"), "#t[1:00] to #t[2:00]");
        assert_eq!(plain_to_typ("no time here"), "no time here");
        assert_eq!(plain_to_typ("0:00"), "#t[0:00]");
        assert_eq!(plain_to_typ("23:59"), "#t[23:59]");
    }
}
