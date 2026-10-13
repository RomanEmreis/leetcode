/*
  3727. Maximum Alternating Sum of Squares
  
  You are given an integer array nums. You may rearrange the elements in any order.
  The alternating score of an array arr is defined as:
      score = arr[0]2 - arr[1]2 + arr[2]2 - arr[3]2 + ...
  
  Return an integer denoting the maximum possible alternating score of nums after rearranging its elements.
  
  Example 1:
  Input: nums = [1,2,3]
  Output: 12
  Explanation:
  A possible rearrangement for nums is [2,1,3], which gives the maximum alternating score among all possible rearrangements.
  
  The alternating score is calculated as:
  score = 22 - 12 + 32 = 4 - 1 + 9 = 12
  
  Example 2:
  Input: nums = [1,-1,2,-2,3,-3]
  Output: 16
  Explanation:
  A possible rearrangement for nums is [-3,-1,-2,1,3,2], which gives the maximum alternating score among all possible rearrangements.
  
  The alternating score is calculated as:
  score = (-3)2 - (-1)2 + (-2)2 - (1)2 + (3)2 - (2)2 = 9 - 1 + 4 - 1 + 9 - 4 = 16
*/
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        const MAX_ABS: usize = 40_000;

        let mut frequency = vec![0u32; MAX_ABS + 1];
        let mut total = 0i64;
        let mut highest = 0usize;

        for &value in &nums {
            let absolute = value.unsigned_abs() as usize;
            let square = absolute as i64 * absolute as i64;

            frequency[absolute] += 1;
            total += square;
            highest = highest.max(absolute);
        }

        let mut negative = nums.len() / 2;
        let mut penalty = 0i64;

        for absolute in 0..=highest {
            let taken = negative.min(frequency[absolute] as usize);

            if taken == 0 {
                continue;
            }

            let value = absolute as i64;
            penalty += 2 * taken as i64 * value * value;
            negative -= taken;

            if negative == 0 {
                break;
            }
        }

        total - penalty
    }
}
