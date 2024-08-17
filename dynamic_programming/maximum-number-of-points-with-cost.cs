/*
  You are given an m x n integer matrix points (0-indexed). Starting with 0 points, you want to maximize the number of points you can get from the matrix.
  To gain points, you must pick one cell in each row. Picking the cell at coordinates (r, c) will add points[r][c] to your score.
  However, you will lose points if you pick a cell too far from the cell that you picked in the previous row. For every two adjacent rows r and r + 1 (where 0 <= r < m - 1), 
  picking cells at coordinates (r, c1) and (r + 1, c2) will subtract abs(c1 - c2) from your score.

  Return the maximum number of points you can achieve.

  abs(x) is defined as:
    x for x >= 0.
    -x for x < 0.

  Example 1:
    Input: points = [[1,2,3],[1,5,1],[3,1,1]]
    Output: 9
    Explanation:
    The blue cells denote the optimal cells to pick, which have coordinates (0, 2), (1, 1), and (2, 0).
    You add 3 + 5 + 3 = 11 to your score.
    However, you must subtract abs(2 - 1) + abs(1 - 0) = 2 from your score.
    Your final score is 11 - 2 = 9.
*/
public class Solution {
    public long MaxPoints(int[][] points) {
        int m = points.Length;
        int n = points[0].Length;

        Span<long> dp = stackalloc long[n];
        for (int j = 0; j < n; ++j) {
            dp[j] = points[0][j];
        }

        for (int i = 1; i < m; ++i) {
            Span<long> left = stackalloc long[n];
            Span<long> right = stackalloc long[n];
            Span<long> curr = stackalloc long[n];

            left[0] = dp[0];
            for (int j = 1; j < n; ++j) {
                left[j] = Math.Max(left[j - 1], dp[j] + j);
            }

            right[n - 1] = dp[n - 1] - (n - 1);
            for (int j = n - 2; j >= 0; --j) {
                right[j] = Math.Max(right[j + 1], dp[j] - j);
            }

            for (int j = 0; j < n; ++j) {
                curr[j] = points[i][j] + Math.Max(left[j] - j, right[j] + j);
            }

            dp = curr;
        }

        long result = 0;
        foreach (long val in dp) {
            result = Math.Max(result, val);
        }
        return result;
    }
}
