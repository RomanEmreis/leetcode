/*
  You are given a positive integer k. You are also given:
    a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
    a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].
  The two arrays contain integers from 1 to k.

  You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.

  The matrix should also satisfy the following conditions:
    The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
    The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.

  Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.

  Example 1:
    Input: k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
    Output: [[3,0,0],[0,0,1],[0,2,0]]
    Explanation: The diagram above shows a valid example of a matrix that satisfies all the conditions.
    The row conditions are the following:
    - Number 1 is in row 1, and number 2 is in row 2, so 1 is above 2 in the matrix.
    - Number 3 is in row 0, and number 2 is in row 2, so 3 is above 2 in the matrix.
    The column conditions are the following:
    - Number 2 is in column 1, and number 1 is in column 2, so 2 is left of 1 in the matrix.
    - Number 3 is in column 0, and number 2 is in column 1, so 3 is left of 2 in the matrix.
    Note that there may be multiple correct answers.

  Example 2:
    Input: k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
    Output: []
    Explanation: From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
    No matrix can satisfy all the conditions, so we return the empty matrix.
*/
class Solution {
private:
    static vector<int> topological_sort(const vector<vector<int>>& graph, const int& k) {
        vector<int> degree(k + 1, 0), sorted;
        queue<int> q;
        for (int i = 1; i <= k; ++i) {
            for (const int& j : graph[i]) ++degree[j];
        }
        for (int i = 1; i <= k; ++i) {
            if (degree[i] == 0) q.push(i);
        }

        while (!q.empty()) {
            int node = q.front();
            q.pop();
            sorted.push_back(node);
            for (const int& n : graph[node]) {
                if (--degree[n] == 0) q.push(n);
            }
        }

        return sorted.size() == k ? sorted : vector<int>();
    }
public:
    vector<vector<int>> buildMatrix(int k, vector<vector<int>>& rowConditions, vector<vector<int>>& colConditions) {
        vector<vector<int>> row_graph(k + 1), col_graph(k + 1);
        for (const vector<int>& condition : rowConditions) {
            row_graph[condition[0]].push_back(condition[1]);
        }
        for (const vector<int>& condition : colConditions) {
            col_graph[condition[0]].push_back(condition[1]);
        }

        vector<int> row_sorted = topological_sort(row_graph, k);
        vector<int> col_sorted = topological_sort(col_graph, k);

        if (row_sorted.empty() || col_sorted.empty()) return {};

        vector<vector<int>> result(k);
        unordered_map<int, int> row_map, col_map;
        for (int i = 0; i < k; ++i) {
            result[i].resize(k, 0);
            row_map[row_sorted[i]] = i;
            col_map[col_sorted[i]] = i;
        }
        for (int i = 1; i <= k; ++i) {
            result[row_map[i]][col_map[i]] = i;
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
