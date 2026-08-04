/*
  3731. Find Missing Elements
  
  You are given an integer array nums consisting of unique integers.
  Originally, nums contained every integer within a certain range. However, some integers might have gone missing from the array.
  
  The smallest and largest integers of the original range are still present in nums.
  
  Return a sorted list of all the missing integers in this range. If no integers are missing, return an empty list.
  
  Example 1:
  Input: nums = [1,4,2,5]
  Output: [3]
  Explanation:
  The smallest integer is 1 and the largest is 5, so the full range should be [1,2,3,4,5]. Among these, only 3 is missing.
  
  Example 2:
  Input: nums = [7,8,6,9]
  Output: []
  Explanation:
  The smallest integer is 6 and the largest is 9, so the full range is [6,7,8,9]. All integers are already present, so no integer is missing.
  
  Example 3:
  Input: nums = [5,1]
  Output: [2,3,4]
  Explanation:
  The smallest integer is 1 and the largest is 5, so the full range should be [1,2,3,4,5]. The missing integers are 2, 3, and 4.
*/
impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut lo = 0u64;
        let mut hi = 0u64;
        let mut min = 101;
        let mut max = 0;

        for v in nums {
            if v < 64 {
                lo |= 1u64 << v as u32;
            } else {
                hi |= 1u64 << (v - 64) as u32;
            }

            min = min.min(v);
            max = max.max(v);
        }

        let missing_count = (max - min + 1) as usize - n;
        let mut res = Vec::with_capacity(missing_count);

        for v in min + 1..max {
            let seen = if v < 64 {
                lo & (1u64 << v as u32) != 0
            } else {
                hi & (1u64 << (v - 64) as u32) != 0
            };

            if !seen {
                res.push(v);
            }
        }

        res
    }
}
