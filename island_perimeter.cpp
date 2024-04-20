class Solution {
public:
    int islandPerimeter(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size(), result = 0;

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (grid[i][j] == 1) {
                    if (i == 0 || grid[i - 1][j] == 0) ++result;
                    if (j == 0 || grid[i][j - 1] == 0) ++result;
                    if (i == n - 1 || grid[i + 1][j] == 0) ++result;
                    if (j == m - 1 || grid[i][j + 1] == 0) ++result;
                }
            }
        }

        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
