use std::{
    i32,
    ops::{AddAssign, DivAssign},
};

pub struct Solution;
impl Solution {
    fn get_digits<T>(x: T) -> T
    where
        T: PartialOrd + DivAssign<T> + AddAssign<T> + From<u16>,
    {
        let mut rem = x;
        let mut digits = T::from(0);

        while rem > T::from(0) {
            rem /= T::from(10);
            digits += T::from(1);
        }

        digits
    }

    fn check(x: i32, digits: u32) -> bool {
        let mut rem_max_digits = 10u32;

        if digits < rem_max_digits {
            return true;
        }

        let mut rem_max = i32::MAX;
        let mut rem = x;
        loop {
            let max_pow = 10i32.pow(rem_max_digits - 1);
            let max_digit = rem_max / max_pow;
            let rem_digit = rem % 10;

            if max_digit < rem_digit {
                return false;
            } else if max_digit > rem_digit {
                return true;
            } else if rem == 0 {
                return true;
            }

            rem_max -= max_digit * max_pow;
            rem_max_digits -= 1;
            rem /= 10;
        }
    }

    pub fn reverse(x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        let x = x.abs();

        let max_digits = Solution::get_digits(x) as u32;

        if !Solution::check(x, max_digits) {
            return 0;
        }

        let mut digits = 0u32;
        let mut remaining = x;
        let mut reverse_number = 0;

        while remaining > 0 {
            let front_pow = 10i32.pow(max_digits - digits - 1);
            let front = remaining / front_pow;

            reverse_number += front * 10i32.pow(digits);
            remaining -= front_pow * front;
            digits += 1;
        }

        return reverse_number * sign;
    }

    pub fn reverse2(x: i32) -> i32 {
        match x < 0 {
            true => {
                -1 * x
                    .to_string()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap_or_default()
            }
            false => x
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or_default(),
        }
    }
}
