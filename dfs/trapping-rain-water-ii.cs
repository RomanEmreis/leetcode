/*
  407. Trapping Rain Water II
  
  Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
  
  Example 1:
  Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
  Output: 4
  Explanation: After the rain, water is trapped between the blocks.
  We have two small ponds 1 and 3 units trapped.
  The total volume of water trapped is 4.
  
  Example 2:
  Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
  Output: 10
*/
public class Solution {
    public int TrapRainWater(int[][] heightMap) {
        int m = heightMap.Length;
        int n = heightMap[0].Length;
        bool[,] visited = new bool[m,n];
        PriorityQueue<(int, int, int), int> pq = new();
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (i == 0 || i == m - 1 || j == 0 || j == n - 1) {
                    pq.Enqueue((heightMap[i][j], i, j), heightMap[i][j]);
                    visited[i, j] = true;
                }
            }
        }

        int result = 0;
        Span<int> directions = [-1, 0, 1, 0, -1];
        while (pq.TryDequeue(out var item, out _)) {
            var (h, i, j) = item;
            for (int k = 0; k < 4; ++k) {
                int x = i + directions[k];
                int y = j + directions[k + 1];
                if (x >= 0 && x < m && y >= 0 && y < n && !visited[x, y]) {
                    result += Math.Max(0, h - heightMap[x][y]);
                    visited[x, y] = true;
                    int nh = Math.Max(h, heightMap[x][y]);
                    pq.Enqueue((nh, x, y), nh);
                }
            }
        }
        return result;
    }
}
