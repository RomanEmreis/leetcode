/*
  2290. Minimum Obstacle Removal to Reach Corner
  
  
  0 represents an empty cell,
  1 represents an obstacle that may be removed.
  You can move up, down, left, or right from and to an empty cell.
  
  Return the minimum number of obstacles to remove so you can move from the upper left corner (0, 0) to the lower right corner (m - 1, n - 1).
  
  Example 1:
  Input: grid = [[0,1,1],[1,1,0],[1,1,0]]
  Output: 2
  Explanation: We can remove the obstacles at (0, 1) and (0, 2) to create a path from (0, 0) to (2, 2).
  It can be shown that we need to remove at least 2 obstacles, so we return 2.
  Note that there may be other ways to remove 2 obstacles to create a path.
*/
public class Solution {
    public int MinimumObstacles(int[][] grid) {
        int m = grid.Length;
        int n = grid[0].Length;

        LinkedList<(int, int, int)> graph = [];
        graph.AddLast((0,0,0));

        bool[,] visited = new bool[m,n];
        Span<int> directions = [-1,0,1,0,-1];

        while (true) {
            var (i, j, k) = graph.First.Value;
            graph.RemoveFirst();

            if (i == m - 1 && j == n - 1) return k;
            if (visited[i,j]) continue;

            visited[i,j] = true;

            for (int h = 0; h < 4; ++h) {
                int x = i + directions[h];
                int y = j + directions[h + 1];

                if (x >= 0 && x < m && y >= 0 && y < n) {
                    if (grid[x][y] == 0) graph.AddFirst((x, y, k));
                    else graph.AddLast((x, y, k + 1));
                }
            }
        }
    }
}
