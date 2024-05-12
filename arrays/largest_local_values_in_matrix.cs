/*
  You are given an n x n integer matrix grid.
  Generate an integer matrix maxLocal of size (n - 2) x (n - 2) such that:
  maxLocal[i][j] is equal to the largest value of the 3 x 3 matrix in grid centered around row i + 1 and column j + 1.
  In other words, we want to find the largest value in every contiguous 3 x 3 matrix in grid.

  Return the generated matrix.

  Example 1:
    Input: grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
    Output: [[9,9],[8,6]]
    Explanation: The diagram above shows the original matrix and the generated matrix.
    Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.
*/
public class Solution {
    public int[][] LargestLocal(int[][] grid) {
        int n = grid.Length;
        int m = n - 2;

        int[][] maxlocal = new int[m][];

        for (int i = 0; i < m; ++i) {
            maxlocal[i] = new int[m];
            for (int j = 0; j < m; ++j) {
                int maxVal = 0;
                for (int x = i; x < i + 3; ++x) {
                    for (int y = j; y < j + 3; ++y) {
                        maxVal = Math.Max(maxVal, grid[x][y]);
                    }
                }
                maxlocal[i][j] = maxVal;
            }
        }

        return maxlocal;
    }
}
