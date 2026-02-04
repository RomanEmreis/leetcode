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
public class Solution {
    public long MaxSumTrionic(int[] nums) {
        int n = nums.Length;
        int i = 0;

        long res = long.MinValue;

        while (i < n) {
            int l = i++;
            
            while (i < n && nums[i - 1] < nums[i]) 
                ++i;

            if (i == l + 1)
                continue;

            int p = i - 1;
            long sum = nums[p - 1] + nums[p];
            while (i < n && nums[i - 1] > nums[i])
                sum += nums[i++];

            if (i == p + 1 || i == n || nums[i - 1] == nums[i])
                continue;

            int q = i - 1;
            sum += nums[i++];

            long max = 0;
            long t = 0;
            while (i < n && nums[i - 1] < nums[i]) {
                t += nums[i++];
                max = Math.Max(max, t);
            }

            sum += max;

            max = 0;
            t = 0;

            for (int j = p - 2; j >= l; --j) {
                t += nums[j];
                max = Math.Max(max, t);
            }

            sum += max;

            i = q;
            res = Math.Max(res, sum);
        }

        return res;
    }
}
