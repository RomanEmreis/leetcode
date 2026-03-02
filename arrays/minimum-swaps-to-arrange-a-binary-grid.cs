/*
  1536. Minimum Swaps to Arrange a Binary Grid
  
  Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.
  A grid is said to be valid if all the cells above the main diagonal are zeros.
  
  Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.
  
  The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).
  
  Example 1:
  Input: grid = [[0,0,1],[1,1,0],[1,0,0]]
  Output: 3
  
  Example 2:
  Input: grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
  Output: -1
  Explanation: All rows are similar, swaps have no effect on the grid.
  
  Example 3:
  Input: grid = [[1,0,0],[1,1,0],[1,1,1]]
  Output: 0
*/
public class Solution {
    public int MinSwaps(int[][] grid) {
        int n = grid.Length;

        Span<int> suffixZeros = stackalloc int[n];
        for (int i = 0; i < n; ++i)
            suffixZeros[i] = GetSuffixZeros(grid[i]);

        int res = 0;

        for (int i = 0; i < n; ++i) {
            int neededZeros = n - i - 1;
            int j = GetFirstRowWithEnoughZeros(ref suffixZeros, i, neededZeros);

            if (j == -1)
                return -1;

            for (int k = j; k > i; --k)
                suffixZeros[k] = suffixZeros[k - 1];

            res += j - i;
        }

        return res;
    }

    private static int GetSuffixZeros(int[] row) {
        int n = row.Length;

        for (int i = n - 1; i >= 0; --i)
            if (row[i] == 1)
                return n - i - 1;
        
        return n;
    }

    private static int GetFirstRowWithEnoughZeros(ref readonly Span<int> suffixZeros, int i, int neededZeros) {
        for (int j = i; j < suffixZeros.Length; ++j)
            if (suffixZeros[j] >= neededZeros)
                return j;
        
        return -1;
    }
}
