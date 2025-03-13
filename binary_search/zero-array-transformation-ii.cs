/*
  3356. Zero Array Transformation II

  You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri, vali].
  
  Each queries[i] represents the following action on nums:
  
  Decrement the value at each index in the range [li, ri] in nums by at most vali.
  The amount by which each value is decremented can be chosen independently for each index.
  A Zero Array is an array with all its elements equal to 0.
  
  Return the minimum possible non-negative value of k, such that after processing the first k queries in sequence, nums becomes a Zero Array. If no such k exists, return -1.
  
  Example 1:
  Input: nums = [2,0,2], queries = [[0,2,1],[0,2,1],[1,1,3]]
  Output: 2
  Explanation:
  For i = 0 (l = 0, r = 2, val = 1):
  Decrement values at indices [0, 1, 2] by [1, 0, 1] respectively.
  The array will become [1, 0, 1].
  For i = 1 (l = 0, r = 2, val = 1):
  Decrement values at indices [0, 1, 2] by [1, 0, 1] respectively.
  The array will become [0, 0, 0], which is a Zero Array. Therefore, the minimum value of k is 2.
  
  Example 2:
  Input: nums = [4,3,2,1], queries = [[1,3,2],[0,2,1]]
  Output: -1
  Explanation:
  For i = 0 (l = 1, r = 3, val = 2):
  Decrement values at indices [1, 2, 3] by [2, 2, 1] respectively.
  The array will become [4, 1, 0, 0].
  For i = 1 (l = 0, r = 2, val = 1):
  Decrement values at indices [0, 1, 2] by [1, 1, 0] respectively.
  The array will become [3, 0, 0, 0], which is not a Zero Array.
*/
public class Solution {
    public int MinZeroArray(int[] nums, int[][] queries) {
        int n = nums.Length;
        if (n == 0) return 0;

        int m = queries.Length;
        int result = 0;
        int s = 0;
        Span<int> d = stackalloc int[n + 1];
        
        for (int i = 0; i < nums.Length; ++i) {
            while (s + d[i] < nums[i]) {
                if (result >= m) return -1;

                ++result;

                int k = result - 1;

                int r = queries[k][1];
                if (r < i) continue;

                int l = queries[k][0];
                int v = queries[k][2];

                int j = i > l ? i : l;
                d[j] += v;
                d[r + 1] -= v;
            }
            s += d[i];
        }

        return result;
    }
}
