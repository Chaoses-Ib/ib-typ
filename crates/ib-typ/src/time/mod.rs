use anyhow::Context;

pub mod duration;

pub fn parse_time(s: &str) -> anyhow::Result<jiff::civil::Time> {
    jiff::fmt::strtime::parse("%k:%M", s)
        .and_then(|t| t.to_time())
        .context("time")
}

#[cfg(test)]
mod tests {
    use super::*;

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
