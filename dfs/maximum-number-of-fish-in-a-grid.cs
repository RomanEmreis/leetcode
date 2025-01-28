/*
  2658. Maximum Number of Fish in a Grid
  
  You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:
  
  A land cell if grid[r][c] = 0, or
  A water cell containing grid[r][c] fish, if grid[r][c] > 0.
  A fisher can start at any water cell (r, c) and can do the following operations any number of times:
  
  Catch all the fish at cell (r, c), or
  Move to any adjacent water cell.
  Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.
  
  An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.
  
  Example 1:
  Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
  Output: 7
  Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.
  
  Example 2:
  Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
  Output: 1
  Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish.
*/
public class Solution {
    public int FindMaxFish(int[][] grid) {
        int m = grid.Length;
        int n = grid[0].Length;
        int result = 0;
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                int curr = Dfs(i, j);
                result = Math.Max(result, curr);
            }
        }
        return result;

        int Dfs(int i, int j) {
            if (i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == 0) return 0;

            int curr = grid[i][j];
            grid[i][j] = 0;

            curr += Dfs(i + 1, j);
            curr += Dfs(i - 1, j);
            curr += Dfs(i, j + 1);
            curr += Dfs(i, j - 1);

            return curr;
        }
    }
}
