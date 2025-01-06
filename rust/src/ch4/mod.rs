pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut concatted = [nums1, nums2].concat();
        concatted.sort();

        if concatted.len() % 2 == 0 {
            (concatted[(concatted.len() - 1) / 2] + concatted[(concatted.len() - 1) / 2 + 1]) as f64 / 2.0
        } else {
            concatted[(concatted.len() - 1) / 2] as f64
        }
    }
}
