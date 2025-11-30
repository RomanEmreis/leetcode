/*
  1590. Make Sum Divisible by P
  
  Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the remaining elements is divisible by p. It is not allowed to remove the whole array.
  
  Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.
  
  A subarray is defined as a contiguous block of elements in the array.
  
  Example 1:
  Input: nums = [3,1,4,2], p = 6
  Output: 1
  Explanation: The sum of the elements in nums is 10, which is not divisible by 6. We can remove the subarray [4], and the sum of the remaining elements is 6, which is divisible by 6.
  
  Example 2:
  Input: nums = [6,3,5,2], p = 9
  Output: 2
  Explanation: We cannot remove a single element to get a sum divisible by 9. The best way is to remove the subarray [5,2], leaving us with [6,3] with sum 9.
  
  Example 3:
  Input: nums = [1,2,3], p = 3
  Output: 0
  Explanation: Here the sum is 6. which is already divisible by 3. Thus we do not need to remove anything.
*/
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let sum = nums.iter().map(|x| *x as i64).sum::<i64>();
        let rem = (sum % p as i64) as i32;
        if rem == 0 {
            return 0;
        }

        let n = nums.len();
        let mut res = n;
        let mut prefix = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, 0);

        for i in 0..n {
            prefix += nums[i];
            prefix %= p;

            let t = (prefix - rem + p) % p;
            if let Some(&j) = map.get(&t) {
                res = res.min(i + 1 - j);
            }
            map.insert(prefix, i + 1);
        }

        if res == n {
            -1
        } else {
            res as i32
        }
    }
}
