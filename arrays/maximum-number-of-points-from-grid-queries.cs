/*
  2503. Maximum Number of Points From Grid Queries
  
  You are given an m x n integer matrix grid and an array queries of size k.
  
  Find an array answer of size k such that for each integer queries[i] you start in the top left cell of the matrix and repeat the following process:
  
  If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
  Otherwise, you do not get any points, and you end this process.
  After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.
  
  Return the resulting array answer.
  
  Example 1:
  Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
  Output: [5,8,1]
  Explanation: The diagrams above show which cells we visit to get points for each query.
  
  Example 2:
  Input: grid = [[5,2,1],[1,1,2]], queries = [3]
  Output: [0]
  Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
*/
public class Solution {
    private static readonly IComparer<int> comparer = Comparer<int>.Create((a, b) => a - b);
    public int[] MaxPoints(int[][] grid, int[] queries) {
        int k = queries.Length;
        Span<(int, int)> qs = stackalloc (int, int)[k];
        for (int i = 0; i < k; ++i) {
            qs[i] = (queries[i], i);
        }

        qs.Sort((a, b) => a.Item1 - b.Item1);

        int[] result = new int[k];
        int m = grid.Length;
        int n = grid[0].Length;

        bool[,] visited = new bool[m, n];
        visited[0, 0] = true;

        PriorityQueue<(int, int, int), int> pq = new(comparer);
        pq.Enqueue((grid[0][0], 0, 0), grid[0][0]);

        Span<int> dirs = [-1, 0, 1, 0, -1];
        int count = 0;

        foreach (var (v, kk) in qs) {
            k = kk;
            while (pq.TryPeek(out _, out int val) && val < v) {
                var (_, i, j) = pq.Dequeue();
                ++count;
                for (int h = 0; h < 4; ++h) {
                    int x = i + dirs[h];
                    int y = j + dirs[h + 1];
                    if (x >= 0 && y >= 0 && x < m && y < n && !visited[x, y]) {
                        visited[x, y] = true;
                        pq.Enqueue((grid[x][y], x, y), grid[x][y]);
                    }
                }
            }
            result[k] = count;
        }

        return result;
    }
}
