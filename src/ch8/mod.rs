use std::num::IntErrorKind;

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();

        let first_non_digit = s
            .chars()
            .skip(1)
            .enumerate()
            .find(|(_i, ch)| !ch.is_ascii_digit())
            .and_then(|(i, _)| Some(i))
            .unwrap_or(s.len() - 1);

        match &s[0..first_non_digit + 1].parse() {
            Ok(v) => *v,
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => i32::MAX,
                IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}
