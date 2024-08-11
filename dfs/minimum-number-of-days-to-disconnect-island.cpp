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
class Solution {
private:
    static bool is_disconnected(vector<vector<int>>& grid, int n, int m) {
        bool found = false;
        vector<vector<bool>> visited(n, vector<bool>(m));

        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < m; ++j) {
                if (!found && grid[i][j] == 1) {
                    dfs(grid, visited, n, m, i, j);
                    found = true;
                } else if (grid[i][j] == 1 && !visited[i][j]) {
                    return true;
                }
            }
        }

        return !found;
    }

    static void dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int n, int m, int i, int j) {
        if (i < 0 || j < 0 || i >= n || j >= m || grid[i][j] == 0 || visited[i][j]) return;

        visited[i][j] = true;

        dfs(grid, visited, n, m, i + 1, j);
        dfs(grid, visited, n, m, i - 1, j);
        dfs(grid, visited, n, m, i, j + 1);
        dfs(grid, visited, n, m, i, j - 1);
    }

public:
    int minDays(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();
        if (is_disconnected(grid, n, m)) return 0;

        for (size_t i = 0; i < n; ++i) {
            for (size_t j = 0; j < m; ++j) {
                if (grid[i][j] == 1) {
                    grid[i][j] = 0;

                    if (is_disconnected(grid, n, m)) return 1;

                    grid[i][j] = 1;
                }
            }
        }

        return 2;
    }
};
