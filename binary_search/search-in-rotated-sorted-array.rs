/*
  33. Search in Rotated Sorted Array
  
  There is an integer array nums sorted in ascending order (with distinct values).
  
  Prior to being passed to your function, nums is possibly left rotated at an unknown index k (1 <= k < nums.length) 
  such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7]
  might be left rotated by 3 indices and become [4,5,6,7,0,1,2].
  
  Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
  
  You must write an algorithm with O(log n) runtime complexity.
  
  Example 1:
  Input: nums = [4,5,6,7,0,1,2], target = 0
  Output: 4
  
  Example 2:
  Input: nums = [4,5,6,7,0,1,2], target = 3
  Output: -1
  
  Example 3:
  Input: nums = [1], target = 0
  Output: -1              
*/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();

        let mut l = 0;
        let mut r = n - 1;

        while l <= r {
            let m = (l + r) >> 1;
            if nums[m] == target {
                return m as i32;
            }

            if nums[l] <= nums[m] {
                if nums[l] <= target && target < nums[m] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if nums[m] < target && target <=  nums[r] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }

        -1
    }
}
