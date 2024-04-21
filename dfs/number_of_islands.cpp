class Solution {
private:
    static void discoverIslands(vector<vector<char>>& grid, int i, int j, int& n, int& m) {
        if (i < 0 || i >= n || j < 0 || j >= m || grid[i][j] == '0') return;

        grid[i][j] = '0';

        discoverIslands(grid, i - 1, j, n, m);
        discoverIslands(grid, i + 1, j, n, m);
        discoverIslands(grid, i, j - 1, n, m);
        discoverIslands(grid, i, j + 1, n, m);
    }

public:
    int numIslands(vector<vector<char>>& grid) {
        int n = grid.size(), m = grid[0].size(), result = 0;
        if (n == 0 || m == 0) return result;

        for (int i = 0; i < n; ++i) {
            for  (int j = 0; j < m; ++j) {
                if (grid[i][j] == '1') {
                    ++result;
                    discoverIslands(grid, i, j, n, m);
                }
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
