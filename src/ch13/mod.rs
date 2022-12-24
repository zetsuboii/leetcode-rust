use std::collections::hash_map::HashMap;

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
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_letters: HashMap<char, i32> = HashMap::new();
        roman_letters.insert('I', 1);
        roman_letters.insert('V', 5);
        roman_letters.insert('X', 10);
        roman_letters.insert('L', 50);
        roman_letters.insert('C', 100);
        roman_letters.insert('D', 500);
        roman_letters.insert('M', 1000);

        let mut sum: i32 = 0;

        let mut i: usize = 0;

        while i < s.len() {
            let roman_value = roman_letters.get(&s.chars().nth(i).unwrap()).unwrap();

            match s.chars().nth(i + 1) {
                Some(next_roman) => {
                    let next_roman_value = roman_letters.get(&next_roman).unwrap();

                    if *next_roman_value == roman_value * 5 || *next_roman_value == roman_value * 10
                    {
                        sum += next_roman_value - roman_value;
                        i += 2;
                    } else {
                        sum += roman_value;
                        i += 1;
                    }
                }
                None => {
                    sum += roman_value;
                    i += 1;
                }
            }
        }

        sum
    }

    pub fn roman_to_int_alt(s: String) -> i32 {
        let mut sum = 0;
        let mut prev;
        let mut curr = '\0';

        for c in s.chars() {
            prev = curr;
            curr = c;

            match curr {
                'I' => sum += 1,
                'V' => sum += if prev == 'I' { 3 } else { 5 },
                'X' => sum += if prev == 'I' { 8 } else { 10 },
                'L' => sum += if prev == 'X' { 30 } else { 50 },
                'C' => sum += if prev == 'X' { 80 } else { 100 },
                'D' => sum += if prev == 'C' { 300 } else { 500 },
                'M' => sum += if prev == 'C' { 800 } else { 1000 },
                _ => println!("Skipping invalid roman"),
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn test_roman_to_int_alt() {
        assert_eq!(Solution::roman_to_int_alt(String::from("MCMXCIV")), 1994);
    }
}
