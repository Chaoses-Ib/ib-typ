use std::{fmt::Write, time::Duration};

use anyhow::{Context, bail};
use bon::builder;
use logos::Logos;
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use unicode_ident::is_xid_continue;

use crate::{
    time::{Times, parse_duration_hms, parse_time},
    to_jinja,
};

impl Times {
    pub fn to_duration(&self) -> anyhow::Result<String> {
        // A duration needs at least two time points
        if self.times.len() < 2 {
            return Ok(Default::default());
        }

        let mut times = self.times.iter().peekable();
        let mut s = String::new();
        while let Some(t) = times.next() {
            if !s.is_empty() {
                // Break on time with date
                if parse_time(t).is_err() {
                    s += "\n";
                }

                s += "+";
            }

            // Time range's second op should be a time without date
            match times.next_if(|t| parse_time(t).is_ok()) {
                Some(t2) => {
                    write!(s, "{t}-{t2}")?;
                }
                None => s += t,
            }
        }
        Ok(s)
    }

    pub fn to_duration_and_eval(&self) -> anyhow::Result<DurationFormat> {
        let s = self.to_duration()?;
        match duration_eval_format(&s) {
            Ok(d) => Ok(DurationFormat {
                s: format!("{}: {s}", d.s),
                ..d
            }),
            Err(_) => Ok(DurationFormat { s, seconds: 0 }),
        }
    }
}

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
pub enum DurationToken {
    #[regex(r"\d+(?:\.\d+)?")]
    Number,

    #[regex(r"\d?\d:\d\d")]
    Time,

    #[token("~")]
    Approx,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[regex(r"[^\d~+\-()]")]
    Other,
}

#[builder]
pub fn duration_eval_pre(
    #[builder(start_fn)] pattern: &str,
    #[builder(default)] time_min_sec: bool,
) -> Result<String, anyhow::Error> {
    let mut lex = DurationToken::lexer(&pattern);
    let mut s = String::new();
    while let Some(Ok(token)) = lex.next() {
        // dbg!(token);
        match token {
            DurationToken::Time => {
                if time_min_sec {
                    let d = parse_duration_hms(lex.slice())?;
                    write!(s, "{}", d.as_secs_f64() / 60.0)?;
                    continue;
                }
                let a = parse_time(lex.slice())?;
                let op = lex.next().transpose().ok().flatten().context("op")?;
                let mut b = || lex.next().transpose().ok().flatten().context("b");
                match op {
                    // Time range
                    DurationToken::Minus | DurationToken::Approx => {
                        match b()? {
                            DurationToken::Time => {
                                let b = parse_time(lex.slice())?;
                                // let d = (b - a).total(jiff::Unit::Minute)?;
                                let mut d = b.duration_since(a).as_mins();
                                if d < 0 {
                                    d += 24 * 60;
                                }
                                if let DurationToken::Approx = op {
                                    s += "~";
                                }
                                write!(s, "{}", d)?;
                            }
                            _ => bail!("b"),
                        }
                    }
                    _ => bail!("op"),
                }
            }
            DurationToken::ParenOpen => {
                s += "(";
                let c = &s[s.floor_char_boundary(lex.span().start - 1)..]
                    .chars()
                    .next();
                if c.is_some_and(is_xid_continue) {
                    s += "'";
                    while let Some(Ok(token)) = lex.next() {
                        match token {
                            DurationToken::ParenClose => {
                                s += "')";
                                break;
                            }
                            _ => s += lex.slice(),
                        }
                    }
                }
            }
            _ => s += lex.slice(),
        }
    }
    s += lex.remainder();
    Ok(s)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationEval {
    pub d: Duration,
    pub plus: bool,
    pub approx: bool,
}

fn mss(s: String) -> Result<f64, minijinja::Error> {
    let s = duration_eval_pre(&s)
        .time_min_sec(true)
        .call()
        .map_err(to_jinja)?;
    let d = duration_eval_inner(&s).map_err(to_jinja)?;
    Ok(d.d.as_secs_f64() / 60.0)
}

fn duration_eval_inner(s: &str) -> Result<DurationEval, anyhow::Error> {
    let (s, plus) = {
        let e = s.strip_prefix("+");
        (e.unwrap_or(&s), e.is_some())
    };
    let approx = s.contains("~");
    let s = s.replace("~", "");
    // e.g. 1h+1
    let s = s.replace("h+", "h");
    let s = s.replace("h", "*60+");
    // e.g. 1h, inputing 1+
    let s = s.strip_suffix("+").unwrap_or(&s);

    let mut env = Environment::empty();
    env.add_function("mss", mss);
    let expr = env.compile_expression(&s).with_context(|| s.to_owned())?;
    let r = expr.eval(context!()).unwrap();
    let d = Duration::from_secs_f64(TryInto::<f64>::try_into(r)? * 60.0);
    Ok(DurationEval { d, plus, approx })
}

pub fn duration_eval(s: &str) -> Result<DurationEval, anyhow::Error> {
    if s.is_empty() {
        bail!("empty");
    }
    let s = duration_eval_pre(s).call()?;
    duration_eval_inner(&s)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationFormat {
    pub s: String,
    pub seconds: u64,
}

pub fn duration_format(d: DurationEval) -> DurationFormat {
    let secs = d.d.as_secs_f64();
    let mut s = String::new();
    if d.plus {
        s += "+";
    }
    if d.approx {
        s += "~";
    }
    // For round half up instead of to even
    let secs = secs + 0.1;
    s += &if secs < 3.0 * 60.0 {
        format!("{:.0}", secs / 60.0)
    } else {
        let s = format!("{:.1}", secs / 3600.0);
        let s = s.strip_suffix(".0").unwrap_or(&s);
        format!("{s}h")
    };
    DurationFormat {
        s,
        seconds: d.d.as_secs(),
    }
}

pub fn duration_eval_format(s: &str) -> Result<DurationFormat, anyhow::Error> {
    Ok(duration_format(duration_eval(s)?))
}

pub fn duration_eval_format_s(s: &str) -> Result<String, anyhow::Error> {
    Ok(duration_format(duration_eval(s)?).s)
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use crate::wasm::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn duration_eval_format(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s = str::from_utf8(s)?;
        to_bytes!(super::duration_eval_format(s)?)
    }

    #[cfg(feature = "wasm-extra")]
    #[wasm_func]
    pub fn times_to_duration(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s: Times = from_bytes!(s);
        to_bytes!(s.to_duration()?)
    }

    #[wasm_func]
    pub fn times_to_duration_and_eval(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s: Times = from_bytes!(s);
        to_bytes!(s.to_duration_and_eval()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn duration_eval_pre(s: &str) -> Result<String, anyhow::Error> {
        super::duration_eval_pre(s).call()
    }

    #[test]
    fn duration_eval_pre_() -> anyhow::Result<()> {
        // Time range
        assert_eq!(duration_eval_pre("1+4:+2")?, "1+4:+2");

        assert_eq!(duration_eval_pre("1+0:00-4:00+2")?, "1+240+2");
        assert_eq!(duration_eval_pre("1+8:00-12:00+2")?, "1+240+2");
        assert_eq!(duration_eval_pre("1+23:00-0:00+2")?, "1+60+2");
        assert_eq!(duration_eval_pre("1+23:00-04:00+2")?, "1+300+2");
        assert_eq!(duration_eval_pre("1+0:01-0:00+2")?, "1+1439+2");

        assert_eq!(duration_eval_pre("1+8:00~12:00+2")?, "1+~240+2");
        assert_eq!(duration_eval_pre("1+8:00~12:00*0.1+2")?, "1+~240*0.1+2");
        Ok(())
    }

    #[test]
    fn duration_eval_() -> anyhow::Result<()> {
        assert_eq!(duration_eval("1h")?.d, Duration::from_mins(60));
        assert_eq!(duration_eval("1h3")?.d, Duration::from_mins(63));
        assert_eq!(
            duration_eval("1h3.3")?.d,
            Duration::from_secs(63 * 60 + 3 * 6)
        );
        Ok(())
    }

    #[test]
    fn duration_eval_mss() -> anyhow::Result<()> {
        assert_eq!(duration_eval_format_s("1+mss(1:30)")?, "3");
        assert_eq!(duration_eval_format_s("1+mss(1:15+0:14)")?, "2");
        assert_eq!(duration_eval_format_s("1+mss(1:15+0:15)")?, "3");
        assert_eq!(duration_eval_format_s("1+(1+0.9)")?, "3");
        Ok(())
    }

    #[test]
    fn duration_format_() -> anyhow::Result<()> {
        assert_eq!(duration_eval_format_s("1")?, "1");
        assert_eq!(duration_eval_format_s("3")?, "0.1h");
        assert_eq!(duration_eval_format_s("1h2")?, "1h");
        assert_eq!(duration_eval_format_s("1h3")?, "1.1h");
        assert_eq!(duration_eval_format_s("1h3.3")?, "1.1h");

        // Round half up instead of to even
        assert_eq!(duration_eval_format_s("0.5")?, "1");
        assert_eq!(duration_eval_format_s("2h2.5")?, "2h");
        assert_eq!(duration_eval_format_s("2h3")?, "2.1h");
        assert_eq!(duration_eval_format_s("2.05h")?, "2.1h");

        assert_eq!(duration_eval_format_s("+1h2")?, "+1h");

        // Time range
        assert_eq!(duration_eval_format_s("1+8:00~12:00*0.1+2")?, "~0.5h");

        Ok(())
    }
}
