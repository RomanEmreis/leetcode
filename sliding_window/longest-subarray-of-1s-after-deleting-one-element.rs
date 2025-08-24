/*
  1493. Longest Subarray of 1's After Deleting One Element
  
  Given a binary array nums, you should delete one element from it.
  
  Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
  
  Example 1:
  Input: nums = [1,1,0,1]
  Output: 3
  Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.
  
  Example 2:
  Input: nums = [0,1,1,1,0,1,1,0,1]
  Output: 5
  Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].
  
  Example 3:
  Input: nums = [1,1,1]
  Output: 2
  Explanation: You must delete one element.
*/
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut z = 0;
        for &n in &nums {
            if n == 0 {
                z += 1;
            }
            if z > 1 {
                if nums[l] == 0 {
                    z -= 1;
                }
                l += 1;
            }
        }
        (nums.len() - l - 1) as i32
    }
}
