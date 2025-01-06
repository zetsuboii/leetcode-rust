pub struct Solution;

impl Solution {
    /// Reasoning for the function
    /// 0 P     6 I    N
    /// 1 A   5 L S  I G
    /// 2 Y 4 A   H R
    /// 3 P       I
    /// If we take this zigzag, after 6 letters, pattern repeats
    /// When we print each row we're essentially printing the points that has the
    /// same distance to the middle point (this being strictly equal to the distance is
    /// coincidence, but the relation of each letter to the pattern is guaranteed)
    pub fn convert(s: String, num_rows: i32) -> String {
        let step_amount = if num_rows > 2 {
            num_rows * 2 - 2
        } else {
            num_rows
        } as usize;
        let step_mid = (step_amount / 2) as usize;

        let mut out = String::new();

        for i in 0..(num_rows as usize) {
            let mut j = i;

            while j < s.len() {
                out.push_str(&s[j..j + 1]);

                // If the letter is not on the polar ends of the row, there's a
                // counterpart
                if i > 0 && i < (num_rows as usize - 1) {
                    let ctr = j + (step_mid - i) * 2;
                    if ctr < s.len() {
                        out.push_str(&s[ctr..ctr + 1]);
                    }
                }

                j += step_amount;
            }
        }

        out
    }
}
