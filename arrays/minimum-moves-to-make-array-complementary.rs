/*
  1674. Minimum Moves to Make Array Complementary
  
  You are given an integer array nums of even length n and an integer limit. 
  In one move, you can replace any integer from nums with another integer between 1 and limit, inclusive.
  
  The array nums is complementary if for all indices i (0-indexed), nums[i] + nums[n - 1 - i] equals the same number. 
  For example, the array [1,2,3,4] is complementary because for all indices i, nums[i] + nums[n - 1 - i] = 5.
  
  Return the minimum number of moves required to make nums complementary.
  
  Example 1:
  Input: nums = [1,2,4,3], limit = 4
  Output: 1
  Explanation: In 1 move, you can change nums to [1,2,2,3] (underlined elements are changed).
  nums[0] + nums[3] = 1 + 3 = 4.
  nums[1] + nums[2] = 2 + 2 = 4.
  nums[2] + nums[1] = 2 + 2 = 4.
  nums[3] + nums[0] = 3 + 1 = 4.
  Therefore, nums[i] + nums[n-1-i] = 4 for every i, so nums is complementary.
  
  Example 2:
  Input: nums = [1,2,2,1], limit = 2
  Output: 2
  Explanation: In 2 moves, you can change nums to [2,2,2,2]. You cannot change any number to 3 since 3 > limit.
  
  Example 3:
  Input: nums = [1,2,1,2], limit = 2
  Output: 0
  Explanation: nums is already complementary.
*/
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let pair_count = nums.len() / 2;

        let mut diff = vec![0; 2 * limit + 2];

        for (&left, &right) in nums.iter().zip(nums.iter().rev()).take(pair_count) {
            let a = left as usize;
            let b = right as usize;

            let lo = a.min(b);
            let hi = a.max(b);
            let sum = a + b;

            diff[lo + 1] -= 1;
            diff[sum] -= 1;
            diff[sum + 1] += 1;
            diff[hi + limit + 1] += 1;
        }

        let mut moves = (2 * pair_count) as i32;
        let mut best = moves;

        for delta in diff.iter().take(2 * limit + 1).skip(2) {
            moves += delta;
            best = best.min(moves);
        }

        best
    }
}
