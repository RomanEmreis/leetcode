/*
  3761. Minimum Absolute Distance Between Mirror Pairs
  
  You are given an integer array nums.
  A mirror pair is a pair of indices (i, j) such that:
      0 <= i < j < nums.length, and
      reverse(nums[i]) == nums[j], where reverse(x) denotes the integer formed by reversing the digits of x. Leading zeros are omitted after reversing, for example reverse(120) = 21.
  
  Return the minimum absolute distance between the indices of any mirror pair. The absolute distance between indices i and j is abs(i - j).
  
  If no mirror pair exists, return -1.
  
  Example 1:
  Input: nums = [12,21,45,33,54]
  Output: 1
  Explanation:
  The mirror pairs are:
      (0, 1) since reverse(nums[0]) = reverse(12) = 21 = nums[1], giving an absolute distance abs(0 - 1) = 1.
      (2, 4) since reverse(nums[2]) = reverse(45) = 54 = nums[4], giving an absolute distance abs(2 - 4) = 2.
  
  The minimum absolute distance among all pairs is 1.
  
  Example 2:
  Input: nums = [120,21]
  Output: 1
  Explanation:
  There is only one mirror pair (0, 1) since reverse(nums[0]) = reverse(120) = 21 = nums[1].
  The minimum absolute distance is 1.
  
  Example 3:
  Input: nums = [21,120]
  Output: -1
  Explanation:
  There are no mirror pairs in the array.
*/
use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut res = n + 1;
        let mut pos = HashMap::with_capacity(n);

        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&j) = pos.get(&num) {
                res = res.min(i - j);
            }
            pos.insert(rev(num), i);
        }

        if res > n { -1 } else { res as i32 }
    }
}

#[inline]
fn rev(mut x: i32) -> i32 {
    let mut y = 0;
    while x > 0 {
        y = y * 10 + x % 10;
        x /= 10;
    }
    y
}
