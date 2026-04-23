/*
  2615. Sum of Distances
  
  You are given a 0-indexed integer array nums. There exists an array arr of length nums.length, 
  where arr[i] is the sum of |i - j| over all j such that nums[j] == nums[i] and j != i. If there is no such j, set arr[i] to be 0.
  
  Return the array arr.
  
  Example 1:
  Input: nums = [1,3,1,1,2]
  Output: [5,0,3,4,0]
  Explanation: 
  When i = 0, nums[0] == nums[2] and nums[0] == nums[3]. Therefore, arr[0] = |0 - 2| + |0 - 3| = 5. 
  When i = 1, arr[1] = 0 because there is no other index with value 3.
  When i = 2, nums[2] == nums[0] and nums[2] == nums[3]. Therefore, arr[2] = |2 - 0| + |2 - 3| = 3. 
  When i = 3, nums[3] == nums[0] and nums[3] == nums[2]. Therefore, arr[3] = |3 - 0| + |3 - 2| = 4. 
  When i = 4, arr[4] = 0 because there is no other index with value 2. 
  
  Example 2:
  Input: nums = [0,5,3]
  Output: [0,0,0]
  Explanation: Since each element in nums is distinct, arr[i] = 0 for all i.
*/
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();

        let mut order: Vec<u32> = (0..n as u32).collect();
        order.sort_unstable_by_key(|&i| unsafe {
            *nums.get_unchecked(i as usize)
        });

        let mut res = vec![0i64; n];

        let mut i = 0usize;
        while i < n {
            let val = unsafe { *nums.get_unchecked(*order.get_unchecked(i) as usize) };

            let mut j = i + 1;
            while j < n && unsafe {
                *nums.get_unchecked(*order.get_unchecked(j) as usize) == val
            } {
                j += 1;
            }

            let group = unsafe { order.get_unchecked_mut(i..j) };
            group.sort_unstable();

            let sz = (j - i) as i64;

            let mut total: i64 = 0;
            for &idx in group.iter() {
                total += idx as i64;
            }

            let mut prefix: i64 = 0;
            for (k, &idx) in group.iter().enumerate() {
                let idx = idx as i64;
                let k   = k as i64;
                let left  = idx * k - prefix;
                let right = (total - prefix - idx) - idx * (sz - 1 - k);
                unsafe {
                    *res.get_unchecked_mut(idx as usize) = left + right;
                }
                prefix += idx;
            }

            i = j;
        }

        res
    }
}
