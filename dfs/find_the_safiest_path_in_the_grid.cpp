/*
  You are given a 0-indexed 2D matrix grid of size n x n, where (r, c) represents:
  A cell containing a thief if grid[r][c] = 1
  An empty cell if grid[r][c] = 0
  You are initially positioned at cell (0, 0). In one move, you can move to any adjacent cell in the grid, including cells containing thieves.

  The safeness factor of a path on the grid is defined as the minimum manhattan distance from any cell in the path to any thief in the grid.
  Return the maximum safeness factor of all paths leading to cell (n - 1, n - 1).
  An adjacent cell of cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) and (r - 1, c) if it exists.
  The Manhattan distance between two cells (a, b) and (x, y) is equal to |a - x| + |b - y|, where |val| denotes the absolute value of val.

  Example 1:
    Input: grid = [[1,0,0],[0,0,0],[0,0,1]]
    Output: 0
    Explanation: All paths from (0, 0) to (n - 1, n - 1) go through the thieves in cells (0, 0) and (n - 1, n - 1).

  Example 2:
    Input: grid = [[0,0,1],[0,0,0],[0,0,0]]
    Output: 2
    Explanation: The path depicted in the picture above has a safeness factor of 2 since:
    - The closest cell of the path to the thief at cell (0, 2) is cell (0, 0). The distance between them is | 0 - 0 | + | 0 - 2 | = 2.
      It can be shown that there are no other paths with a higher safeness factor.
*/
class Solution {
public:
    vector<vector<int>> directions = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    int maximumSafenessFactor(vector<vector<int>>& grid) {
        int n = grid.size();
        vector<vector<int>> dist(n, vector<int>(n, INT_MAX));
        queue<pair<int, int>> q;

        for (int r = 0; r < n; ++r) {
            for (int c = 0; c < n; ++c) {
                if (grid[r][c] == 1) {
                    q.push({r, c});
                    dist[r][c] = 0;
                }
            }
        }

        while (!q.empty()) {
            auto [r, c] = q.front();
            q.pop();
            for (auto& dir : directions) {
                int nr = r + dir[0], nc = c + dir[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n && dist[nr][nc] == INT_MAX) {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push({nr, nc});
                }
            }
        }

        priority_queue<pair<int, pair<int, int>>> pq;
        pq.push({dist[0][0], {0, 0}});
        vector<vector<int>> maxSafeness(n, vector<int>(n, -1));
        maxSafeness[0][0] = dist[0][0];

        while (!pq.empty()) {
            auto [d, pos] = pq.top();
            pq.pop();
            int r = pos.first, c = pos.second;
            if (r == n - 1 && c == n - 1) return d;

            for (auto& dir : directions) {
                int nr = r + dir[0], nc = c + dir[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n) {
                    int newSafe = min(d, dist[nr][nc]);
                    if (newSafe > maxSafeness[nr][nc]) {
                        maxSafeness[nr][nc] = newSafe;
                        pq.push({newSafe, {nr, nc}});
                    }
                }
            }
        }

        return -1;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
