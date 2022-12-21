pub struct Solution;

use std::collections::HashMap;

type Matches = HashMap<(usize, usize), bool>;

impl Solution {
    fn dp(matches: &mut Matches, s: &String, p: &String, i: usize, j: usize) -> bool {
        match matches.get(&(i, j)) {
            Some(&m) => return m,
            None => {
                let answer = if j == p.len() {
                    i == s.len()
                } else {
                    let p_bytes = p.as_bytes();
                    let s_bytes = s.as_bytes();

                    let first_match =
                        i < s.len() && (p_bytes[j] == s_bytes[i] || p_bytes[j] == b'.');

                    if j < (p.len() - 1) && p_bytes[j + 1] == b'*' {
                        Self::dp(matches, s, p, i, j + 2)
                            || (first_match && Self::dp(matches, s, p, i + 1, j))
                    } else {
                        first_match && Self::dp(matches, s, p, i + 1, j + 1)
                    }
                };

                matches.insert((i, j), answer);
                answer
            }
        }
    }

    fn is_match(s: String, p: String) -> bool {
        let mut matches: Matches = HashMap::new();
        Self::dp(&mut matches, &s, &p, 0, 0)
    }
}
