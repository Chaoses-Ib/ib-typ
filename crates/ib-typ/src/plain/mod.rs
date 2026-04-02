/*!
Plain text note format.

Mainly for inputing without a full keyboard, i.e. on phones/pads.

The Typst conversion result is not guaranteed to be semantically correct and needs manual check.
*/
use std::fmt::Write;

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
    if input.is_empty() {
        return Default::default();
    }
    let mut result = String::with_capacity(input.len());
    let mut lex = PlainNoteToken::lexer(input);
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
    if last_token != PlainNoteToken::Newline {
        result += "\n";
    }

    result
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
        assert_eq!(plain_to_typ("1:30"), "#t[1:30]\n");
        assert_eq!(plain_to_typ("meeting at 9:00"), "meeting at #t[9:00]\n");
        assert_eq!(plain_to_typ("1:00 to 2:00"), "#t[1:00] to #t[2:00]\n");
        assert_eq!(plain_to_typ("no time here"), "no time here\n");
        assert_eq!(plain_to_typ("0:00"), "#t[0:00]\n");
        assert_eq!(plain_to_typ("23:59"), "#t[23:59]\n");
    }
}
