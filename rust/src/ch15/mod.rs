pub struct Solution;

impl Solution {
    pub fn three_sum(mut num: Vec<i32>) -> Vec<Vec<i32>> {
        num.sort_unstable();
        let mut out = Vec::new();

        for i in 0..num.len() {
            if i > 0 && num[i] == num[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = num.len() - 1;

            while left < right {
                let sum = num[i] + num[left] + num[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    out.push(vec![num[i], num[left], num[right]]);

                    left += 1;
                    while left < right && num[left] == num[left - 1] {
                        left += 1;
                    }
                }
            }
        }

        out
    }
}

pub fn run() {
    let input = vec![-1, 0, 1, 2, -1, -4];
    println!("Solution: {:?}", Solution::three_sum(input));
}
