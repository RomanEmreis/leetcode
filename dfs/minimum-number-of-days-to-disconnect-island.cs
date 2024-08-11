/*
  You are given an m x n binary grid grid where 1 represents land and 0 represents water. An island is a maximal 4-directionally (horizontal or vertical) connected group of 1's.
  The grid is said to be connected if we have exactly one island, otherwise is said disconnected.

  In one day, we are allowed to change any single land cell (1) into a water cell (0).

  Return the minimum number of days to disconnect the grid.

  Example 1:
    Input: grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]
    Output: 2
    Explanation: We need at least 2 days to get a disconnected grid.
    Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.

  Example 2:
    Input: grid = [[1,1]]
    Output: 2
    Explanation: Grid of full water is also disconnected ([[1,1]] -> [[0,0]]), 0 islands.
*/
public class Solution {
    public int MinDays(int[][] grid) {
        int rows = grid.Length, cols = grid[0].Length;
        if (IsDisconnected(grid, rows, cols)) return 0;

        for (int i = 0; i < rows; ++i) {
            for (int j = 0; j < cols; ++j) {
                if (grid[i][j] == 1) {
                    grid[i][j] = 0;

                    if (IsDisconnected(grid, rows, cols)) return 1;
                    
                    grid[i][j] = 1;
                }
            }
        }

        return 2;
    }

    private static bool IsDisconnected(int[][] grid, int rows, int cols) {
        bool found = false;
        bool[,] visited = new bool[rows, cols];

        for (int i = 0; i < rows; ++i) {
            for (int j = 0; j < cols; ++j) {
                if (!found && grid[i][j] == 1) {
                    Dfs(grid, visited, rows, cols, i, j);
                    found = true;
                } else if (grid[i][j] == 1 && !visited[i, j]) {
                    return true;
                }
            }
        }

        return !found;
    }

    private static void Dfs(int[][] grid, bool[,] visited, int rows, int cols, int i, int j) {
        if (i < 0 || j < 0 || i >= rows || j >= cols || grid[i][j] == 0 || visited[i, j]) return;

        visited[i, j] = true;

        Dfs(grid, visited, rows, cols, i + 1, j);
        Dfs(grid, visited, rows, cols, i - 1, j);
        Dfs(grid, visited, rows, cols, i, j + 1);
        Dfs(grid, visited, rows, cols, i, j - 1);
    }
}
