pub struct Solution;

trait GetChar {
    fn get_char(&self, idx: i32) -> u8;
}

impl GetChar for String {
    fn get_char(&self, idx: i32) -> u8 {
        self.as_bytes()[idx as usize]
    }
}

struct Indices {
    start: usize,
    end: usize,
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut indices: Option<Indices> = None;

        for (i, _) in s.chars().enumerate() {
            for j in 0..(i+1) {
                let mut match_i = i as i32;
                let mut match_j = j as i32;

                while match_j <= match_i {
                    if s.get_char(match_i) != s.get_char(match_j) {
                        break;
                    }
                    match_i -= 1;
                    match_j += 1;
                }

                if match_j >= match_i {
                    indices = match indices {
                        Some(ind) => {
                            if (i - j + 1) > (ind.end - ind.start) {
                                Some(Indices {
                                    start: j,
                                    end: i + 1,
                                })
                            } else {
                                Some(ind)
                            }
                        }
                        None => Some(Indices {
                            start: j,
                            end: i + 1,
                        }),
                    }
                }
            }
        }

        let Indices { start, end } = indices.unwrap();
        s.as_str()[start..end].to_string()
    }
}
