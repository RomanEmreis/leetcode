/*
  154. Find Minimum in Rotated Sorted Array II
  
  Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,4,4,5,6,7] might become:
      [4,5,6,7,0,1,4] if it was rotated 4 times.
      [0,1,4,4,5,6,7] if it was rotated 7 times.
  
  Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
  
  Given the sorted rotated array nums that may contain duplicates, return the minimum element of this array.
  
  You must decrease the overall operation steps as much as possible.
  
  Example 1:
  Input: nums = [1,3,5]
  Output: 1
  
  Example 2:
  Input: nums = [2,2,2,0,1]
  Output: 0
*/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let m = (l + r) >> 1;
            if nums[m] == nums[r] {
                r -= 1;
            } else if nums[m] < nums[r] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[l]
    }
}
