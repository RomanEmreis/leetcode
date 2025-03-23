/*
  1976. Number of Ways to Arrive at Destination
  
  You are in a city that consists of n intersections numbered from 0 to n - 1 with bi-directional roads between some intersections. 
  The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.
  You are given an integer n and a 2D integer array roads where roads[i] = [ui, vi, timei] means that there is a road between intersections 
  ui and vi that takes timei minutes to travel. You want to know in how many ways you can travel from intersection 0 to intersection n - 1 in the shortest amount of time.
  
  Return the number of ways you can arrive at your destination in the shortest amount of time. Since the answer may be large, return it modulo 109 + 7.
  
  Example 1:
  Input: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
  Output: 4
  Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
  The four ways to get there in 7 minutes are:
  - 0 ➝ 6
  - 0 ➝ 4 ➝ 6
  - 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
  - 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
  
  Example 2:
  Input: n = 2, roads = [[1,0,10]]
  Output: 1
  Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
*/
public class Solution {
    public int CountPaths(int n, int[][] roads) {
        const long inf = long.MaxValue / 2;
        const int mod = (int) 1e9 + 7;

        long[,] graph = new long[n, n];
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                graph[i, j] = inf;
            }
        }

        foreach (int[] r in roads) {
            int u = r[0];
            int v = r[1];
            int t = r[2];

            graph[u, v] = t;
            graph[v, u] = t;
        }

        graph[0, 0] = 0;

        Span<long> dist = stackalloc long[n];
        dist.Fill(inf);
        dist[0] = 0;

        Span<long> dp = stackalloc long[n];
        dp[0] = 1;

        Span<bool> visited = stackalloc bool[n];

        for (int i = 0; i < n; ++i) {
            int t = -1;
            for (int j = 0; j < n; ++j) {
                if (!visited[j] && (t == -1 || dist[j] < dist[t])) {
                    t = j;
                }
            }

            visited[t] = true;

            for (int j = 0; j < n; ++j) {
                if (j == t) continue;
                
                long ne = dist[t] + graph[t, j];
                if (dist[j] > ne) {
                    dist[j] = ne;
                    dp[j] = dp[t];
                } else if (dist[j] == ne) {
                    dp[j] = (dp[j] + dp[t]) % mod;
                }
            }
        }

        return (int) dp[n - 1];
    }
}
