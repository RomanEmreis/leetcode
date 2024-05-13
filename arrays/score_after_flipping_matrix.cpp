/*
  You are given an m x n binary matrix grid.
  A move consists of choosing any row or column and toggling each value in that row or column (i.e., changing all 0's to 1's, and all 1's to 0's).

  Every row of the matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
  Return the highest possible score after making any number of moves (including zero moves).

  Example 1:
    Input: grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
    Output: 39
    Explanation: 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
*/
class Solution {
public:
    int matrixScore(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();
        int result = n;

        for (int j = 1; j < m; ++j) {
            int set = 0;
            for (int i = 0; i < n; ++i) {
                if (grid[i][j] == grid[i][0]) ++set;
            }
            result = (result << 1) + max(set, n - set);
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
