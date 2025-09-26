/*
  611. Valid Triangle Number
  
  Given an integer array nums, return the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.
  
  Example 1:
  Input: nums = [2,2,3,4]
  Output: 3
  Explanation: Valid combinations are: 
  2,3,4 (using the first 2)
  2,3,4 (using the second 2)
  2,2,3
  
  Example 2:
  Input: nums = [4,2,3,4]
  Output: 4
*/
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        nums.sort();

        let mut res = 0;
        for k in (2..=n - 1).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    res += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        res as i32
    }
}
