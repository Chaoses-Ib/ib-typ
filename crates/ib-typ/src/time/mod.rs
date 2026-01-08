use anyhow::{Context, bail};
use jiff::SignedDuration;
use serde::{Deserialize, Serialize};

pub mod duration;
pub mod short;

#[derive(Debug, Serialize, Deserialize)]
pub struct Times {
    pub times: Vec<String>,
}

/// - `m:ss`
/// - `h:mm:ss`
pub fn parse_duration_hms(s: &str) -> anyhow::Result<SignedDuration> {
    /*
    Ok(jiff::fmt::strtime::parse("%M:%S", s)
        .and_then(|t| t.to_time())
        .context("M:S")?
        .duration_since(jiff::civil::Time::MIN))
    */
    let mut parts = s.split(':').rev();
    let sec: i64 = parts.next().context("M:S")?.parse().context("sec")?;
    let min: i64 = parts.next().context("M:S")?.parse().context("min")?;
    let h: i64 = match parts.next() {
        Some(h) => h.parse().context("h")?,
        None => 0,
    };
    if !parts.next().is_none() {
        bail!("M:S")
    };
    Ok(SignedDuration::from_secs(h * 3600 + min * 60 + sec))
}

pub fn parse_time(s: &str) -> anyhow::Result<jiff::civil::Time> {
    jiff::fmt::strtime::parse("%k:%M", s)
        .and_then(|t| t.to_time())
        .context("time")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_hms_() -> anyhow::Result<()> {
        assert_eq!(parse_duration_hms("1:23")?, SignedDuration::from_secs(83));
        assert_eq!(parse_duration_hms("0:1:23")?, SignedDuration::from_secs(83));
        assert_eq!(
            parse_duration_hms("1:1:23")?,
            SignedDuration::from_secs(3600 + 83)
        );
        Ok(())
    }

    #[test]
    fn parse_time_() {
        let t = parse_time("3:59").unwrap();
        let t2 = parse_time("12:59").unwrap();
        dbg!(&t);
        dbg!(&t2);
        let d = t2.duration_since(t);
        assert_eq!(d, jiff::SignedDuration::from_hours(9));
    }
}
