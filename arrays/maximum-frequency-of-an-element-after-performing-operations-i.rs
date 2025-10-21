/*
  3346. Maximum Frequency of an Element After Performing Operations I
  
  You are given an integer array nums and two integers k and numOperations.
  You must perform an operation numOperations times on nums, where in each operation you:
    Select an index i that was not selected in any previous operations.
    Add an integer in the range [-k, k] to nums[i].
  
  Return the maximum possible frequency of any element in nums after performing the operations.
  
  Example 1:
  Input: nums = [1,4,5], k = 1, numOperations = 2
  Output: 2
  Explanation:
  We can achieve a maximum frequency of two by:
  Adding 0 to nums[1]. nums becomes [1, 4, 5].
  Adding -1 to nums[2]. nums becomes [1, 4, 4].
  
  Example 2:
  Input: nums = [5,11,20,20], k = 5, numOperations = 1
  Output: 2
  Explanation:
  We can achieve a maximum frequency of two by:
  Adding 0 to nums[1].
*/
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let k = k as usize;

        let mut freq = [0i32; 100001];
        let mut min = nums[0] as usize;
        let mut max = min;

        nums.iter().map(|&x| x as usize).for_each(|num| {
            freq[num] += 1;
            max = max.max(num);
            min = min.min(num);
        });

        let mut res = 0;
        let mut l = min;
        let mut r = min;

        let mut sum = 0;
        for num in min..=max {
            if l < num.saturating_sub(k) {
                sum -= freq[l];
                l += 1;
            }
            
            let rmax = max.min(num + k);
            while r <= rmax {
                sum += freq[r];
                r += 1;
            }

            res = res.max(freq[num] + num_operations.min(sum - freq[num]));
        }

        res
    }
}
