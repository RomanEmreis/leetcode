/*
  976. Largest Perimeter Triangle
  
  Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, 
  formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
  
  Example 1:
  Input: nums = [2,1,2]
  Output: 5
  Explanation: You can form a triangle with three side lengths: 1, 2, and 2.
  
  Example 2:
  Input: nums = [1,2,1,10]
  Output: 0
  Explanation: 
  You cannot use the side lengths 1, 1, and 2 to form a triangle.
  You cannot use the side lengths 1, 1, and 10 to form a triangle.
  You cannot use the side lengths 1, 2, and 10 to form a triangle.
  As we cannot use any three side lengths to form a triangle of non-zero area, we return 0.
*/
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        nums.windows(3)
            .find(|w| w[1] + w[2] > w[0])
            .map(|w| w.iter().sum())
            .unwrap_or(0)
    }
}
