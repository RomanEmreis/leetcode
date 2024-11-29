/*
  2577. Minimum Time to Visit a Cell In a Grid
  
  You are given a m x n matrix grid consisting of non-negative integers where grid[row][col] represents the minimum time required to be able to visit the cell (row, col), which means you can visit the cell (row, col)
  only when the time you visit it is greater than or equal to grid[row][col].
  You are standing in the top-left cell of the matrix in the 0th second, and you must move to any adjacent cell in the four directions: up, down, left, and right. Each move you make takes 1 second.
  
  Return the minimum time required in which you can visit the bottom-right cell of the matrix. If you cannot visit the bottom-right cell, then return -1.
  
  Example 1:
  Input: grid = [[0,1,3,2],[5,1,2,5],[4,3,8,6]]
  Output: 7
  Explanation: One of the paths that we can take is the following:
  - at t = 0, we are on the cell (0,0).
  - at t = 1, we move to the cell (0,1). It is possible because grid[0][1] <= 1.
  - at t = 2, we move to the cell (1,1). It is possible because grid[1][1] <= 2.
  - at t = 3, we move to the cell (1,2). It is possible because grid[1][2] <= 3.
  - at t = 4, we move to the cell (1,1). It is possible because grid[1][1] <= 4.
  - at t = 5, we move to the cell (1,2). It is possible because grid[1][2] <= 5.
  - at t = 6, we move to the cell (1,3). It is possible because grid[1][3] <= 6.
  - at t = 7, we move to the cell (2,3). It is possible because grid[2][3] <= 7.
  The final time is 7. It can be shown that it is the minimum time possible.
*/
public class Solution {
    public int MinimumTime(int[][] grid) {
        if (grid[0][1] > 1 && grid[1][0] > 1) return -1;

        int m = grid.Length;
        int n = grid[0].Length;
        int[][] dist = new int[m][];
        for (int i = 0; i < dist.Length; ++i) {
            dist[i] = new int[n];
            Array.Fill(dist[i], 1 << 30);
        }
        dist[0][0] = 0;

        PriorityQueue<(int, int, int), int> pq = new();
        pq.Enqueue((0,0,0), 0);

        Span<int> directions = [-1,0,1,0,-1];
        while (true) {
            var (i, j, k) = pq.Dequeue();
            if (i == m - 1 && j == n - 1) return k;

            for (int d = 0; d < 4; ++d) {
                int x = i + directions[d];
                int y = j + directions[d + 1];

                if (x >= 0 && x < m && y >= 0 && y < n) {
                    int nk = k + 1;
                    if (nk < grid[x][y]) nk = grid[x][y] + (grid[x][y] - nk) % 2;
                    if (nk < dist[x][y]) {
                        dist[x][y] = nk;
                        pq.Enqueue((x, y, nk), nk);
                    }
                }
            }
        }
    }
}
