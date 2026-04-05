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
    /// `#sym.zws`
    ///
    /// e.g. YouTube Live chat
    #[regex(r"\u200B")]
    ZeroWidthSpace,

    /// Newline sequence
    #[regex("\r?\n")]
    Newline,

    /// Time in format like `1:30`, `0:00`, `23:59`
    ///
    /// - Exclude `Time` with second, e.g. `23:59:00`.
    #[regex(r"\d?\d:\d\d")]
    Time,

    /// Simplified [`DurationToken`](crate::time::duration::DurationToken)
    ///
    /// `(?<! )` is checked in [`PlainNoteToken::check()`].
    #[regex(r"(?m)  (\d+(?:\.\d+)?|[+\-~])+$")]
    Duration,

    /// Anything else (text, numbers, symbols)
    #[regex(r"[^\u200B\n]")]
    Other,
}

impl PlainNoteToken {
    pub fn check(&self, lex: &logos::Lexer<PlainNoteToken>) -> bool {
        let source = lex.source();
        let span = lex.span();
        let prev = span.start.checked_sub(1).map(|i| source.as_bytes()[i]);
        let next = source.as_bytes().get(span.end).copied();
        match self {
            PlainNoteToken::Time => next != Some(b':') && prev != Some(b':'),
            PlainNoteToken::Duration => {
                // result.ends_with(' ')
                /*
                lex.span()
                    .start
                    .checked_sub(1)
                    .is_none_or(|i| lex.source().as_bytes()[i] != b' ')
                */
                prev != Some(b' ')
            }
            _ => true,
        }
    }
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
                PlainNoteToken::ZeroWidthSpace
                | PlainNoteToken::Time
                | PlainNoteToken::Duration => {
                    if token.check(&lex) {
                        return Some((typ, true));
                    }
                }
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
            if !token.check(&lex) {
                result += lex.slice();
                last_token = PlainNoteToken::Other;
                continue;
            }
            match token {
                // Strip `ZeroWidthSpace`
                PlainNoteToken::ZeroWidthSpace => continue,
                PlainNoteToken::Newline => {
                    result += match last_token {
                        PlainNoteToken::ZeroWidthSpace => unreachable!(),
                        // Consecutive line feeds
                        PlainNoteToken::Newline | PlainNoteToken::Duration => "\n",
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
                PlainNoteToken::Duration => {
                    // Checked
                    {
                        let last_newline = result.rfind('\n').map(|p| p + 1).unwrap_or(0);
                        let line = result.split_off(last_newline);
                        let duration = &lex.slice()[2..];
                        write!(result, "- {line}  |{duration}").unwrap()
                    }
                }
                PlainNoteToken::Other => result += lex.slice(),
            }
            last_token = token;
        }
        // Trailing new line
        if self.trailing_newline.unwrap_or_else(|| text.contains('\n'))
            // && last_token != PlainNoteToken::Newline
            && !result.ends_with('\n')
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
    fn zero_width_space() {
        // YouTube Live chat
        assert_eq!(
            plain_to_typ(
                "isekaijoucho
Member (6 years)
\u{200B}\u{200B}悪口"
            ),
            r"isekaijoucho \
Member (6 years) \
悪口
"
        );
    }

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

        // Exclude `Time` with second
        assert_eq!(plain_to_typ("23:59:00"), "23:59:00");
    }

    #[test]
    fn duration() {
        // Non-duration
        assert_eq!(plain_to_typ("音楽  30 "), "音楽  30 ");
        assert_eq!(plain_to_typ("音楽   30"), "音楽   30");

        // Durations
        assert_eq!(plain_to_typ("音楽  30"), "- 音楽  |30");
        assert_eq!(plain_to_typ("  +30"), "-   |+30");
        assert_eq!(plain_to_typ("  ~5"), "-   |~5");
        assert_eq!(plain_to_typ("a\n  +30\n"), "a \\\n-   |+30\n");
        assert_eq!(plain_to_typ("a\nb\n  ~5\n"), "a \\\nb \\\n-   |~5\n");
    }
}
