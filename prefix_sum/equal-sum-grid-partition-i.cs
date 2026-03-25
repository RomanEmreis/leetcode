/*
  3546. Equal Sum Grid Partition I
  
  You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible to make either one horizontal or one vertical cut on the grid such that:
      Each of the two resulting sections formed by the cut is non-empty.
      The sum of the elements in both sections is equal.
  
  Return true if such a partition exists; otherwise return false.
  
  Example 1:
  Input: grid = [[1,4],[2,3]]
  Output: true
  Explanation:
  A horizontal cut between row 0 and row 1 results in two non-empty sections, each with a sum of 5. Thus, the answer is true.
  
  Example 2:
  Input: grid = [[1,3],[2,4]]
  Output: false
  Explanation:
  No horizontal or vertical cut results in two non-empty sections with equal sums. Thus, the answer is false.
*/
public class Solution {
    public bool CanPartitionGrid(int[][] grid) {
        int m = grid.Length;
        int n = grid[0].Length;
        
        long prefix = 0;

        Span<long> h = stackalloc long[m];
        Span<long> v = stackalloc long[n];

        for (int i = 0; i < m; ++i)A
            for (int j = 0; j < n; ++j) {
                int val = grid[i][j];
                h[i] += val;
                v[j] += val;
                prefix += val;
            }
                
        long sum = 0;
        for (int i = 0; i < m; ++i) {
            sum += h[i];
            if (sum * 2 == prefix)
                return true;
        }

        sum = 0;
        for (int j = 0; j < n; ++j) {
            sum += v[j];
            if (sum * 2 == prefix)
                return true;
        }

        return false;
    }
}
