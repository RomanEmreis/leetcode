class Solution {
private:
    static vector<int> calcilateArea(vector<vector<int>>& land, int& x, int& y, int& n, int& m) {
        int r = x, c = y;

        while (r < n - 1 && land[r + 1][y] == 1) ++r;
        while (c < m - 1 && land[x][c + 1] == 1) ++c;

        for (int i = x; i <= r; ++i) {
            for (int j = y; j <= c; ++j) {
                land[i][j] = 0;
            }
        }

        return { x, y, r, c };
    }

public:
    vector<vector<int>> findFarmland(vector<vector<int>>& land) {
        int n = land.size(), m = land[0].size();
        if (n == 0 || m == 0) return {};

        vector<vector<int>> result;

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (land[i][j] == 1) {
                    result.push_back(calcilateArea(land, i, j, n, m));
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
