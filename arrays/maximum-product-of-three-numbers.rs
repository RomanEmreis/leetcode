/*
  628. Maximum Product of Three Numbers
  
  Given an integer array nums, find three numbers whose product is maximum and return the maximum product.
  
  Example 1:
  Input: nums = [1,2,3]
  Output: 6
  
  Example 2:
  Input: nums = [1,2,3,4]
  Output: 24
  
  Example 3:
  Input: nums = [-1,-2,-3]
  Output: -6
*/
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;

        nums.into_iter().for_each(|n| {
            if n <= min1 {
                min2 = min1;;
                min1 = n;
            } else if n <= min2 {
                min2 = n;
            }

            if n >= max1 {
                max3 = max2;
                max2 = max1;
                max1 = n;
            } else if n >= max2 {
                max3 = max2;
                max2 = n;
            } else if n >= max3 {
                max3 = n;
            }
        });

        std::cmp::max(
            max1 * min1 * min2,
            max1 * max2 * max3
        )
    }
}
