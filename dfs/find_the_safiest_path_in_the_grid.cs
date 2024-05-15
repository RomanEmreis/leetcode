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
public class Solution {
    private static readonly int[][] directions = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    public int MaximumSafenessFactor(IList<IList<int>> grid) {
        int n = grid.Count;
        if (grid[0][0] == 1 || grid[n - 1][n - 1] == 1) return 0;

        Queue<(int, int)> q = [];
        int[][] dist = Enumerable.Range(0, n)
            .Select(i => Enumerable.Repeat(int.MaxValue, n).ToArray())
            .ToArray();

        for (int i = 0; i < grid.Count; ++i) {
            for (int j = 0; j < grid[i].Count; ++j) {
                if (grid[i][j] == 1) {
                    q.Enqueue((i, j));
                    dist[i][j] = 0;
                }
            }
        }

        while (q.TryDequeue(out var item)) {
            var (r, c) = item;
            foreach (int[] direction in directions) {
                int nr = r + direction[0], nc = c + direction[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n && dist[nr][nc] == int.MaxValue) {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.Enqueue((nr, nc));
                }
            }
        }

        PriorityQueue<(int, int), int> pq = new(Comparer<int>.Create((x, y) => y.CompareTo(x)));
        pq.Enqueue((0, 0), dist[0][0]);

        int[][] maxSafeness = Enumerable.Range(0, n)
            .Select(i => Enumerable.Repeat(-1, n).ToArray())
            .ToArray();

        while (pq.TryDequeue(out var pos, out var d)) {
            var (r, c) = pos;
            if (r == n - 1 && c == n - 1) return d;

            foreach (int[] direction in directions) {
                int nr = r + direction[0], nc = c + direction[1];
                if (nr >= 0 && nr < n && nc >= 0 && nc < n) {
                    int safe = Math.Min(d, dist[nr][nc]);
                    if (safe > maxSafeness[nr][nc]) {
                        maxSafeness[nr][nc] = safe;
                        pq.Enqueue((nr, nc), safe);
                    }
                }
            }
        }

        return -1;
    }
}
