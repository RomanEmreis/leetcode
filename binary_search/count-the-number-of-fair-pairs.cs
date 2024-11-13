/*
  Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.
  
  A pair (i, j) is fair if: 0 <= i < j < n, and lower <= nums[i] + nums[j] <= upper
    
  Example 1:
    Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
    Output: 6
    Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
  
  Example 2:
    Input: nums = [1,7,9,2,5], lower = 11, upper = 11
    Output: 1
    Explanation: There is a single fair pair: (2,3).
*/
public class Solution {
    public long CountFairPairs(int[] nums, int lower, int upper) {
        Array.Sort(nums);
        int n = nums.Length;
        long result = 0;
        for (int i = 0; i < nums.Length; ++i) {
            int j = Search(lower - nums[i], i + 1);
            int k = Search(upper - nums[i] + 1, i + 1);

            result += k - j;
        }
        return result;

        int Search(int x, int l) {
            int r = n;
            while (l < r) {
                int m = (r + l) >> 1;
                if (nums[m] >= x) r = m;
                else l = m + 1;
            }
            return l;
        }
    }
}
