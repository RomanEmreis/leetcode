/*
  3640. Trionic Array II
  
  You are given an integer array nums of length n.
  A trionic subarray is a contiguous subarray nums[l...r] (with 0 <= l < r < n) for which there exist indices l < p < q < r such that:
      nums[l...p] is strictly increasing,
      nums[p...q] is strictly decreasing,
      nums[q...r] is strictly increasing.
  
  Return the maximum sum of any trionic subarray in nums.
  
  Example 1:
  Input: nums = [0,-2,-1,-3,0,2,-1]
  Output: -4
  Explanation:
  Pick l = 1, p = 2, q = 3, r = 5:
      nums[l...p] = nums[1...2] = [-2, -1] is strictly increasing (-2 < -1).
      nums[p...q] = nums[2...3] = [-1, -3] is strictly decreasing (-1 > -3)
      nums[q...r] = nums[3...5] = [-3, 0, 2] is strictly increasing (-3 < 0 < 2).
      Sum = (-2) + (-1) + (-3) + 0 + 2 = -4.
  
  Example 2:
  Input: nums = [1,4,2,7]
  Output: 14
  Explanation:
  Pick l = 0, p = 1, q = 2, r = 3:
      nums[l...p] = nums[0...1] = [1, 4] is strictly increasing (1 < 4).
      nums[p...q] = nums[1...2] = [4, 2] is strictly decreasing (4 > 2).
      nums[q...r] = nums[2...3] = [2, 7] is strictly increasing (2 < 7).
      Sum = 1 + 4 + 2 + 7 = 14.
*/
impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut i = 0;
        let mut res = i64::MIN;

        while i < n {
            let l = i;
            i += 1;
            while i < n && nums[i - 1] < nums[i] {
                i += 1;
            }
            if i == l + 1 {
                continue;
            }

            let p = i - 1;
            let mut s = nums[p - 1] as i64 + nums[p] as i64;
            while i < n && nums[i - 1] > nums[i] {
                s += nums[i] as i64;
                i += 1;
            }
            if i == p + 1 || i == n || nums[i - 1] == nums[i] {
                continue;
            }

            let q = i - 1;
            s += nums[i] as i64;
            i += 1;
            
            let mut max = 0i64;
            let mut t = 0i64;
            while i < n && nums[i - 1] < nums[i] {
                t += nums[i] as i64;
                i += 1;
                max = max.max(t);
            }

            s += max;

            max = 0;
            t = 0;
            let mut j = p as isize - 2;
            while j >= l as isize {
                t += nums[j as usize] as i64;
                j -= 1;
                max = max.max(t);
            }

            s += max;

            res = res.max(s);
            i = q;
        }

        res
    }
}
