/*
  2099. Find Subsequence of Length K With the Largest Sum
  
  You are given an integer array nums and an integer k. You want to find a subsequence of nums of length k that has the largest sum.
  Return any such subsequence as an integer array of length k.
  
  A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
  
  Example 1:
  Input: nums = [2,1,3,3], k = 2
  Output: [3,3]
  Explanation:
  The subsequence has the largest sum of 3 + 3 = 6.
  
  Example 2:
  Input: nums = [-1,-2,3,4], k = 3
  Output: [-1,3,4]
  Explanation: 
  The subsequence has the largest sum of -1 + 3 + 4 = 6.
  
  Example 3:
  Input: nums = [3,4,3,3], k = 2
  Output: [3,4]
  Explanation:
  The subsequence has the largest sum of 3 + 4 = 7. 
  Another possible subsequence is [4, 3].
*/
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap1 = std::collections::BinaryHeap::new();
        let mut heap2 = std::collections::BinaryHeap::new();

        for (i, num) in nums.into_iter().enumerate() {
            heap1.push((-num, i as i32));
            if heap1.len() > k {
                let _ = heap1.pop();
            }
        }

        while let Some(t) = heap1.pop() {
            heap2.push((-t.1, -t.0));
        }

        let mut result = vec![0; k];
        let mut idx = 0;
        while let Some(t) = heap2.pop() {
            result[idx] = t.1;
            idx += 1;
        }
        result
    }
}
