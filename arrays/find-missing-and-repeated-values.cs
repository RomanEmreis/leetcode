/*
  2965. Find Missing and Repeated Values
  
  You are given a 0-indexed 2D integer matrix grid of size n * n with values in the range [1, n2]. Each integer appears exactly once except a which appears twice and b which is missing. The task is to find the repeating and missing numbers a and b.
  
  Return a 0-indexed integer array ans of size 2 where ans[0] equals to a and ans[1] equals to b.
  
  Example 1:
  Input: grid = [[1,3],[2,2]]
  Output: [2,4]
  Explanation: Number 2 is repeated and number 4 is missing so the answer is [2,4].
  
  Example 2:
  Input: grid = [[9,1,7],[8,9,2],[3,4,6]]
  Output: [9,5]
  Explanation: Number 9 is repeated and number 5 is missing so the answer is [9,5].
*/
public class Solution {
    public int[] FindMissingAndRepeatedValues(int[][] grid) {
        int n = grid.Length;
        Span<int> count = stackalloc int[(n * n) + 1];

        for (int i = 0; i < grid.Length; ++i) {
            for (int j = 0; j < grid.Length; ++j) {
                ++count[grid[i][j]];
            }
        }

        int missing = 0;
        int dup = 0;

        for (int i = 1; i < count.Length; ++i) {
            if (count[i] > 1) dup = i;
            if (count[i] == 0) missing = i;
            if (dup != 0 && missing != 0) break;
        }

        return [dup, missing];   
    }
}
