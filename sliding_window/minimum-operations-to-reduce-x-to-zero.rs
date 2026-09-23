/*
  1658. Minimum Operations to Reduce X to Zero
  
  You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost
  or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.
  
  Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.
  
  Example 1:
  Input: nums = [1,1,4,2,3], x = 5
  Output: 2
  Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
  
  Example 2:
  Input: nums = [5,6,7,8,9], x = 4
  Output: -1
  
  Example 3:
  Input: nums = [3,2,20,1,1,3], x = 10
  Output: 5
  Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total: i64 = nums.iter().map(|&value| value as i64).sum();
        let target = total - x as i64;

        if target < 0 {
            return -1;
        }

        if target == 0 {
            return nums.len() as i32;
        }

        let mut left = 0usize;
        let mut window_sum = 0i64;
        let mut longest = 0usize;

        for (right, &value) in nums.iter().enumerate() {
            window_sum += value as i64;

            while window_sum > target {
                window_sum -= nums[left] as i64;
                left += 1;
            }

            if window_sum == target {
                longest = longest.max(right - left + 1);
            }
        }

        if longest == 0 {
            -1
        } else {
            (nums.len() - longest) as i32
        }
    }
}
