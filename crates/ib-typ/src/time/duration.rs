use std::time::Duration;

use anyhow::Context;
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationEval {
    pub d: Duration,
    pub plus: bool,
    pub approx: bool,
}

pub fn duration_eval(s: &str) -> Result<DurationEval, anyhow::Error> {
    let (s, plus) = {
        let e = s.strip_prefix("+");
        (e.unwrap_or(s), e.is_some())
    };
    let approx = s.contains("~");
    let s = s.replace("~", "");
    // e.g. 1h+1
    let s = s.replace("h+", "h");
    let s = s.replace("h", "*60+");
    // e.g. 1h, inputing 1+
    let s = s.strip_suffix("+").unwrap_or(&s);

    let env = Environment::empty();
    let expr = env.compile_expression(&s).with_context(|| s.to_owned())?;
    let r = expr.eval(context!()).unwrap();
    let d = Duration::from_secs_f64(TryInto::<f64>::try_into(r)? * 60.0);
    Ok(DurationEval { d, plus, approx })
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
    use crate::wasm::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn duration_eval_format(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s = str::from_utf8(s)?;
        to_bytes!(super::duration_eval_format(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        Ok(())
    }
}
