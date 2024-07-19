/*
  Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

  A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

  Example 1:
    Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
    Output: [15]
    Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.

  Example 2:
    Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
    Output: [12]
    Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
*/
class Solution {
public:
    vector<int> luckyNumbers (vector<vector<int>>& matrix) {
        int n = matrix.size(), m = matrix[0].size();

        unordered_map<int, int> map(n);
        vector<int> result;

        for (int i = 0; i < n; ++i) {
            int curr_min = INT_MAX;
            for (int j = 0; j < m; ++j) {
                curr_min = min(curr_min, matrix[i][j]);
            }
            map[curr_min] = i;
        }

        for (int i = 0; i < m; ++i) {
            int curr_max = 0;
            for (int j = 0; j < n; ++j) {
                curr_max = max(curr_max, matrix[j][i]);
            }
            if (map.contains(curr_max)) result.push_back(curr_max);
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
