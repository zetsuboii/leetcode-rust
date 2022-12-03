pub struct Solution;

impl Solution {
    /// Finds the longest substring w/o repeating characters
    /// "abcabcbb" -> "abc"
    /// "pwwkew" -> "pwwkew"
    /// Assume letters are ASCII
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut latest_match: Option<usize> = None;
        let mut longest_match = 0;

        for (idx, ch) in s.chars().enumerate() {
            let mut is_match = false;

            // Iterate starting from the latest match to the current index
            let start_idx = latest_match.and_then(|v| Some(v + 1)).unwrap_or(0);

            for ch_idx in start_idx..idx {
                // If a character after the latest match is
                if s.chars().nth(ch_idx).unwrap() == ch {
                    latest_match = Some(ch_idx);
                    is_match = true;
                    break;
                }
            }

            // If there's a match (a repeating character), compare the length, reset
            // stuff and continue
            if is_match {
                let match_length = idx - start_idx;

                if match_length > longest_match {
                    longest_match = match_length;
                }
            }
        }

        (match latest_match {
            Some(m) => longest_match.max(s.len() - 1 - m),
            None => s.len(),
        }) as i32
    }
}
