pub struct Solution;

impl Solution {
    // Symbol       Value
    // I             1
    // V             5
    // X             10
    // L             50
    // C             100
    // D             500
    // M             1000
    //
    // Convert 2945 to IIMCMXLV
    // 1. 2 isn't special and we're on 1000s, push IIM
    // 2. 9 is special and we're on 100s, push C and M (C+2)
    // 3. 4 is special and we're on 10s, push X and L (X+1)
    // 4. 5 isn't special and we're on 1s, push V
    pub fn int_to_roman(num: i32) -> String {
        const MAX_ROMAN: i32 = 4999;
        const ROMAN_LETTERS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

        if num > MAX_ROMAN {
            return String::from("0");
        }

        let mut remaining = num;
        let mut roman: String = String::new();
        let digits: usize = f64::from(num).log10() as usize;

        for i in (0..=digits).rev() {
            let extension = 10i32.pow(i as u32);
            let digit: i32 = remaining / extension;
            remaining -= digit * extension;

            match digit {
                4 => {
                    roman.push(ROMAN_LETTERS[i * 2]);
                    roman.push(ROMAN_LETTERS[i * 2 + 1]);
                }
                9 => {
                    roman.push(ROMAN_LETTERS[i * 2]);
                    roman.push(ROMAN_LETTERS[i * 2 + 2]);
                }
                _ => match digit < 5 {
                    true => {
                        for _ in 0..digit {
                            roman.push(ROMAN_LETTERS[i * 2]);
                        }
                    }
                    false => {
                        roman.push(ROMAN_LETTERS[i * 2 + 1]);

                        let rem_digit = digit - 5;
                        for _ in 0..rem_digit {
                            roman.push(ROMAN_LETTERS[i * 2]);
                        }
                    }
                },
            }
        }

        return roman;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(2945), String::from("MMCMXLV"));
        assert_eq!(Solution::int_to_roman(30), String::from("XXX"));
        assert_eq!(Solution::int_to_roman(1773), String::from("MDCCLXXIII"));
    }
}
