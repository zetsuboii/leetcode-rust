pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .fold(Option::<String>::None, |prev, now| {
                prev.and_then(|v| {
                    Some(
                        v.chars()
                            .zip(now.chars())
                            .take_while(|(ch1, ch2)| ch1 == ch2)
                            .map(|(ch1, _)| ch1)
                            .collect::<String>(),
                    )
                })
                .or(Some(now))
            })
            .unwrap_or(String::new())
    }
}
