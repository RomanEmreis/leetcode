/*
  3342. Find Minimum Time to Reach Last Room II
  
  There is a dungeon with n x m rooms arranged as a grid.
  You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room.
  You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes one second for one move and two seconds for the next, alternating between the two.
  
  Return the minimum time to reach the room (n - 1, m - 1).
  
  Two rooms are adjacent if they share a common wall, either horizontally or vertically.
  
  Example 1:
  Input: moveTime = [[0,4],[4,4]]
  Output: 7
  Explanation:
  The minimum time required is 7 seconds.
  At time t == 4, move from room (0, 0) to room (1, 0) in one second.
  At time t == 5, move from room (1, 0) to room (1, 1) in two seconds.
  
  Example 2:
  Input: moveTime = [[0,0,0,0],[0,0,0,0]]
  Output: 6
  Explanation:
  The minimum time required is 6 seconds.
  At time t == 0, move from room (0, 0) to room (1, 0) in one second.
  At time t == 1, move from room (1, 0) to room (1, 1) in two seconds.
  At time t == 3, move from room (1, 1) to room (1, 2) in one second.
  At time t == 4, move from room (1, 2) to room (1, 3) in two seconds.
  
  Example 3:
  Input: moveTime = [[0,1],[1,2]]
  Output: 4
*/
public class Solution {
    public int MinTimeToReach(int[][] moveTime) {
        Span<(int, int)> dirs = [(0,1), (1,0), (0,-1), (-1, 0)];

        int m = moveTime.Length;
        int n = moveTime[0].Length;

        int[,] dist = new int[m,n];
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                dist[i,j] = int.MaxValue;
            }
        }

        dist[0,0] = 0;

        PriorityQueue<(int, int), int> pq = new();
        pq.Enqueue((0,0), dist[0,0]);

        var dst = (m - 1, n - 1);
        while (pq.TryDequeue(out var u, out int d)) {
            if (u == dst) return d;

            var (i, j) = u;
            if (d > dist[i,j]) continue;

            foreach (var (dx, dy) in dirs) {
                int x = i + dx;
                int y = j + dy;
                if (x < 0 || x == m || y < 0 || y == n) continue;

                int newDist = ((i + j) % 2 + 1) + (d > moveTime[x][y] ? d : moveTime[x][y]);
                if (newDist < dist[x,y]) {
                    dist[x,y] = newDist;
                    pq.Enqueue((x,y), newDist);
                }
            }
        }

        return -1;
    }
}
