/*
  3201. Find the Maximum Length of Valid Subsequence I
  
  You are given an integer array nums.
  A subsequence sub of nums with length x is called valid if it satisfies:
    (sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.
  
  Return the length of the longest valid subsequence of nums.
  
  A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
  
  Example 1:
  Input: nums = [1,2,3,4]
  Output: 4
  Explanation:
  The longest valid subsequence is [1, 2, 3, 4].
  
  Example 2:
  Input: nums = [1,2,1,1,2,1,2]
  Output: 6
  Explanation:
  The longest valid subsequence is [1, 2, 1, 2, 1, 2].
  
  Example 3:
  Input: nums = [1,3]
  Output: 2
  Explanation:
  The longest valid subsequence is [1, 3].
*/
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        const k: i32 = 2;
        let mut odd = 0;
        let mut even = 0;
        let mut alt = 0;
        let mut parity = k;
        for &num in nums.iter() {
            let p = num % k;
            if p != parity {
                alt += 1;
                parity = p;
            }
            if p == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        odd.max(even).max(alt)
    }
}
