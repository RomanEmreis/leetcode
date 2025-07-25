/*
  3487. Maximum Unique Subarray Sum After Deletion
  
  You are given an integer array nums.
  You are allowed to delete any number of elements from nums without making it empty. After performing the deletions, select a subarray of nums such that:
  All elements in the subarray are unique.
  The sum of the elements in the subarray is maximized.
  Return the maximum sum of such a subarray.
  
  Example 1:
  Input: nums = [1,2,3,4,5]
  Output: 15
  Explanation:
  Select the entire array without deleting any element to obtain the maximum sum.
  
  Example 2:
  Input: nums = [1,1,0,1,1]
  Output: 1
  Explanation:
  Delete the element nums[0] == 1, nums[1] == 1, nums[2] == 0, and nums[3] == 1. Select the entire array [1] to obtain the maximum sum.
  
  Example 3:
  Input: nums = [1,2,-1,-2,1,0,-1]
  Output: 3
  Explanation:
  Delete the elements nums[2] == -1 and nums[3] == -2, and select the subarray [2, 1] from [1, 2, 1, 0, -1] to obtain the maximum sum.
*/
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut set = [0; 101];
        let mut result = 0;
        let mut max = -101;

        nums.iter().for_each(|&num| {
            let i = num as usize;
            if num >= 0 && set[i] == 0 {
                result += num;
                set[i] = 1;
            }
            max = max.max(num);
        });

        if max < 0 {
            max
        } else {
            result
        }
    }
}
