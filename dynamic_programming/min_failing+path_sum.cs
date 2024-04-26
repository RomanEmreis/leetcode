public class Solution {
    public int MinFallingPathSum(int[][] grid) {
        int n = grid.Length;
        if (n == 1) return grid[0][0];
        if (n == 2) return Math.Min(grid[0][0] + grid[1][1], grid[0][1] + grid[1][0]);

        int result = int.MaxValue;

        int[,] dp = new int[n, n];
        for (int i = 0; i < n; ++i) {
            dp[n - 1, i] = grid[n - 1][i];
        }

        for (int i = n - 2; i >= 0; --i) {
            int max1 = int.MaxValue, max2 = int.MaxValue, i1 = 0, i2 = 0;

            for (int j = 0; j < n; ++j) {
                if (dp[i + 1, j] <= max1) {
                    max2 = max1;
                    max1 = dp[i + 1, j];
                    i1 = j;
                } else if (dp[i + 1, j] <= max2 && dp[i + 1, j] != max1) {
                    max2 = dp[i + 1, j];
                    i2 = j;
                }
            }

            for (int j = 0; j < n; ++j) {
                if (i1 != j) dp[i, j] = max1 + grid[i][j];
                else dp[i, j] = max2 + grid[i][j];
            }
        }

        for (int i = 0; i < n; ++i) {
            result = Math.Min(result, dp[0, i]);
        }

        return result;
    }
}
