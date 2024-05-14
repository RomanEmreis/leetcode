/*
  In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold in that cell, 0 if it is empty.
  Return the maximum amount of gold you can collect under the conditions:

  Every time you are located in a cell you will collect all the gold in that cell.
    * From your position, you can walk one step to the left, right, up, or down.
    * You can't visit the same cell more than once.
    * Never visit a cell with 0 gold.
  You can start and stop collecting gold from any position in the grid that has some gold.

  Example:
    Input: grid = [[0,6,0],[5,8,7],[0,9,0]]
    Output: 24
    Explanation:
      [[0,6,0],
       [5,8,7],
       [0,9,0]]
  Path to get the maximum gold, 9 -> 8 -> 7.
*/
class Solution {
private:
    int backtrack(int i, int j, int& n, int& m, vector<vector<int>>& grid) {
        if (i < 0 || j < 0 || i >= n || j >= m || grid[i][j] == 0) return 0;

        int val = grid[i][j];
        int maximum = val;
        grid[i][j] = 0;

        maximum = max(maximum, val + backtrack(i + 1, j, n, m, grid));
        maximum = max(maximum, val + backtrack(i - 1, j, n, m, grid));
        maximum = max(maximum, val + backtrack(i, j + 1, n, m, grid));
        maximum = max(maximum, val + backtrack(i, j - 1, n, m, grid));

        grid[i][j] = val;
        return maximum;
    }

public:
    int getMaximumGold(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();
        int result = 0;

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                result = max(result, backtrack(i, j, n, m, grid));
            }
        }

        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
