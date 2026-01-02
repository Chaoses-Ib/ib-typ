use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
pub struct ParseDateShort {
    #[builder(start_fn, into)]
    pub s: String,
    pub yymmdd: Option<bool>,
}

impl ParseDateShort {
    pub fn yymmdd(&self) -> bool {
        self.yymmdd.unwrap_or(true)
    }

    pub fn to_int(&self) -> Option<u32> {
        let s = &self.s;
        if !self.yymmdd() && s.len() >= 6 {
            return None;
        }
        // s.chars().all(|c| c.is_ascii_digit())
        s.parse().ok()
    }
}

#[cfg(feature = "wasm")]
pub mod wasm {
    use super::*;
    use crate::wasm::*;

    initiate_protocol!();

    #[wasm_func]
    pub fn parse_date_short_to_int(s: &[u8]) -> anyhow::Result<Vec<u8>> {
        let s: ParseDateShort = from_bytes!(s);
        to_bytes!(s.to_int())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_short_to_int_() {
        assert_eq!(ParseDateShort::builder("02").build().to_int(), Some(2));
        assert_eq!(
            ParseDateShort::builder("260102").build().to_int(),
            Some(260102)
        );
        assert_eq!(
            ParseDateShort::builder("260102")
                .yymmdd(false)
                .build()
                .to_int(),
            None
        );
    }
}
