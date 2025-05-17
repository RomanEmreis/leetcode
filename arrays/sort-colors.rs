/*
  75. Sort Colors
  
  Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
  We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
  
  You must solve this problem without using the library's sort function.
  
  Example 1:
  Input: nums = [2,0,2,1,1,0]
  Output: [0,0,1,1,2,2]
  
  Example 2:
  Input: nums = [2,0,1]
  Output: [0,1,2]
*/
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0,0,0];
        for num in nums.iter() {
            counts[*num as usize] += 1;
        }

        let mut idx = 0;
        for i in 0..3 {
            for _ in 0..counts[i] {
                nums[idx] = i as i32;
                idx += 1;
            }
        }
    }
}
