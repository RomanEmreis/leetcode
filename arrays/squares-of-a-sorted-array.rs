/*
  977. Squares of a Sorted Array
  
  Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
  
  Example 1:
  Input: nums = [-4,-1,0,3,10]
  Output: [0,1,9,16,100]
  Explanation: After squaring, the array becomes [16,1,0,9,100].
  After sorting, it becomes [0,1,9,16,100].
  
  Example 2:
  Input: nums = [-7,-3,2,3,11]
  Output: [4,9,9,49,121]
*/
impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut i = r;
        let mut res = vec![0; n];
        while i >= 0 {
            let a = nums[l];
            let b = nums[r];
            if a.abs() < b.abs() {
                res[i] = b * b;
                r -= 1;
            } else {
                res[i] = a * a;
                l += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        res
    }
}
