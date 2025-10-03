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
*/
public sealed class Solution {
    public int TrapRainWater(int[][] heightMap) {
        Span<(int, int)> dirs = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0)
        ];

        int m = heightMap.Length;
        int n = heightMap[0].Length;

        PriorityQueue<(int, int), int> minHeap = new();
        bool[,] seen = new bool[m, n];

        int res = 0;

        for (int i = 0; i < m; ++i) {
            minHeap.Enqueue((i, 0), heightMap[i][0]);
            minHeap.Enqueue((i, n - 1), heightMap[i][n - 1]);
            seen[i, 0] = true;
            seen[i, n - 1] = true;
        }

        for (int j = 1; j < n - 1; ++j) {
            minHeap.Enqueue((0, j), heightMap[0][j]);
            minHeap.Enqueue((m - 1, j), heightMap[m - 1][j]);
            seen[0, j] = true;
            seen[m - 1, j] = true;
        }

        while (minHeap.TryDequeue(out var item, out int h)) {
            var (i, j) = item;
            foreach (var (dx, dy) in dirs) {
                int x = i + dx;
                int y = j + dy;

                if (x < 0 || y < 0 || x == m || y == n)
                    continue;

                if (seen[x, y])
                    continue;

                if (heightMap[x][y] < h) {
                    res += h - heightMap[x][y];
                    minHeap.Enqueue((x, y), h);
                } else {
                    minHeap.Enqueue((x, y), heightMap[x][y]);
                }
                
                seen[x, y] = true;
            }
        }

        return res;
    }
}
