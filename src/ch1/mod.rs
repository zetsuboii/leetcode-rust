pub struct Ch1;

use std::collections::HashMap;

impl Ch1 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      let mut indices: Vec<i32> = Vec::new();
      let mut pairs: HashMap<i32, usize> = HashMap::new();

      for (i, num) in nums.iter().enumerate() {
        match pairs.get(&num) {
          Some(pair_idx) => {
            indices.push(i as i32);
            indices.push(pair_idx.to_owned() as i32);
          },
          None => {
            pairs.insert(target - num, i);
          }
        }
      }

      indices
    }
}
