/*
  3542. Minimum Operations to Convert All Elements to Zero
  
  You are given an array nums of size n, consisting of non-negative integers. Your task is to apply some (possibly zero) operations on the array so that all elements become 0.
  In one operation, you can select a
    [i, j] (where 0 <= i <= j < n) and set all occurrences of the minimum non-negative integer in that subarray to 0.
  
  Return the minimum number of operations required to make all elements in the array 0.
  
  Example 1:
  Input: nums = [0,2]
  Output: 1
  Explanation:
      Select the subarray [1,1] (which is [2]), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in [0,0].
      Thus, the minimum number of operations required is 1.
  
  Example 2:
  Input: nums = [3,1,2,1]
  Output: 3
  Explanation:
      Select subarray [1,3] (which is [1,2,1]), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in [3,0,2,0].
      Select subarray [2,2] (which is [2]), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in [3,0,0,0].
      Select subarray [0,0] (which is [3]), where the minimum non-negative integer is 3. Setting all occurrences of 3 to 0 results in [0,0,0,0].
      Thus, the minimum number of operations required is 3.
  
  Example 3:
  Input: nums = [1,2,1,2,1,2]
  Output: 4
  Explanation:
      Select subarray [0,5] (which is [1,2,1,2,1,2]), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in [0,2,0,2,0,2].
      Select subarray [1,1] (which is [2]), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in [0,0,0,2,0,2].
      Select subarray [3,3] (which is [2]), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in [0,0,0,0,0,2].
      Select subarray [5,5] (which is [2]), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in [0,0,0,0,0,0].
      Thus, the minimum number of operations required is 4.
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut st = Vec::new();
        st.push(0);
        for &num in nums.iter() {
            while st.last().is_some_and(|t| *t > num) {
                st.pop();
            }
            if st.last().is_none_or(|t| *t < num) {
                st.push(num);
                res += 1;
            }
        }
        res
    }
}
