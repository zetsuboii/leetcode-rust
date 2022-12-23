pub struct Solution;

impl Solution {
    /// This works by taking two pointers from both left and right, and
    /// incrementing the short one. This way it is guaranteed the next
    /// iteration is the biggest result it could be
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut borders = (0, heights.len() - 1);
        let mut max_area = 0;

        while borders.0 != borders.1 {
            let area = (borders.1 - borders.0) as i32 * heights[borders.0].min(heights[borders.1]);

            if heights[borders.0] < heights[borders.1] {
                borders.0 += 1;
            } else {
                borders.1 -= 1;
            }

            if area > max_area {
                max_area = area;
            }
        }
        return max_area;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_max_area() {
        // Load input.txt
        let heights = std::fs::read_to_string("src/ch11/input.txt")
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let start = std::time::Instant::now();
        super::Solution::max_area(heights);
        dbg!(start.elapsed());
        assert!(start.elapsed() <= std::time::Duration::from_millis(1000));
    }
}
