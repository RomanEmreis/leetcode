class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& grid) {
        int n = grid.size();
        if (n == 1) return grid[0][0];
        if (n == 2) return min(grid[0][0] + grid[1][1], grid[0][1] + grid[1][0]);

        vector<vector<int>> dp(n, vector<int>(n, 0));

        for (int i = 0; i < n; ++i) {
            dp[n - 1][i] = grid[n - 1][i];
        }

        for (int i = n - 2; i >= 0; --i) {
            int max1 = INT_MAX, max2 = INT_MAX, i1, i2;

            for (int j = 0; j < n; ++j) {
                if (dp[i + 1][j] <= max1) {
                    max2 = max1;
                    max1 = dp[i + 1][j];
                    i1 = j;
                } else if (dp[i + 1][j] <= max2 && dp[i + 1][j] != max1) {
                    max2 = dp[i + 1][j];
                    i2 = j;
                }
            }

            for (int j = 0; j < n; ++j) {
                if (i1 != j) dp[i][j] = max1 + grid[i][j];
                else dp[i][j] = max2 + grid[i][j];
            }
        }

        return *min_element(dp[0].begin(),dp[0].end());;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
