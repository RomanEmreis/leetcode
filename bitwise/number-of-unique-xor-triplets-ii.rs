/*
  3514. Number of Unique XOR Triplets II
  
  You are given an integer array nums.
  A XOR triplet is defined as the XOR of three elements nums[i] XOR nums[j] XOR nums[k] where i <= j <= k.
  
  Return the number of unique XOR triplet values from all possible triplets (i, j, k).
  
  Example 1:
  Input: nums = [1,3]
  Output: 2
  Explanation:
  The possible XOR triplet values are:
      (0, 0, 0) → 1 XOR 1 XOR 1 = 1
      (0, 0, 1) → 1 XOR 1 XOR 3 = 3
      (0, 1, 1) → 1 XOR 3 XOR 3 = 1
      (1, 1, 1) → 3 XOR 3 XOR 3 = 3
  
  The unique XOR values are {1, 3}. Thus, the output is 2.
  
  Example 2:
  Input: nums = [6,7,8,9]
  Output: 4
  Explanation:
  The possible XOR triplet values are {6, 7, 8, 9}. Thus, the output is 4.
*/
impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut spectrum = [0i64; 2048];
        let mut max_value = 0usize;

        for &value in &nums {
            let value = value as usize;
            spectrum[value] = 1;
            max_value = max_value.max(value);
        }

        let universe = (max_value + 1).next_power_of_two();
        let values = &mut spectrum[..universe];

        fwht(values);

        for c in values.iter_mut() {
            *c = *c * *c * *c;
        }

        fwht(values);

        values
            .into_iter()
            .filter(|c| **c != 0)
            .count() as i32
    }
}

#[inline]
fn fwht(values: &mut [i64]) {
    let mut half = 1usize;

    while half < values.len() {
        let block = half * 2;
        for start in (0..values.len()).step_by(block) {
            for offset in 0..half {
                let left = values[start + offset];
                let right = values[start + half + offset];

                values[start + offset] = left + right;
                values[start + half + offset] = left - right;
            }
        }

        half = block;
    }
}
