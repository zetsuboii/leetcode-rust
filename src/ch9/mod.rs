pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let (x_start, x_end) = x.split_at(x.len() / 2);
        x_start.chars().zip(x_end.chars().rev()).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use crate::ch9::Solution;

    #[test]
    fn check_palindrome() {
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(1001), true);
        assert_eq!(Solution::is_palindrome(1000), false);
        assert_eq!(Solution::is_palindrome(122), false);
    }
}
