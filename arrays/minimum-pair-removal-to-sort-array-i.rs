/*
  3507. Minimum Pair Removal to Sort Array I
  
  Given an array nums, you can perform the following operation any number of times:
      Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
      Replace the pair with their sum.
  
  Return the minimum number of operations needed to make the array non-decreasing.
  
  An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).
  
  Example 1:
  Input: nums = [5,2,3,1]
  Output: 2
  Explanation:
      The pair (3,1) has the minimum sum of 4. After replacement, nums = [5,2,4].
      The pair (2,4) has the minimum sum of 6. After replacement, nums = [5,6].
  
  The array nums became non-decreasing in two operations.
  
  Example 2:
  Input: nums = [1,2,2]
  Output: 0
  Explanation:
  The array nums is already sorted.
*/
impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        while !is_sorted(&nums) {
            let mut pairs = Vec::with_capacity(nums.len());
            for i in 1..nums.len() {
                pairs.push(nums[i] + nums[i - 1]);
            }
            
            let mut min = i32::MAX;
            let mut j = 0;
            for i in 0..pairs.len() {
                if pairs[i] < min {
                    min = pairs[i];
                    j = i;
                }
            }

            nums[j] = min;
            nums.remove(j + 1);

            res += 1;
        }
        res
    }
}

#[inline]
fn is_sorted(nums: &[i32]) -> bool {
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            return false;
        }
    }
    true
}
