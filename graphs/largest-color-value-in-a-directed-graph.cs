/*
  1857. Largest Color Value in a Directed Graph
  
  There is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.
  You are given a string colors where colors[i] is a lowercase English letter representing the color of the ith node in this graph (0-indexed). 
  You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.
  A valid path in the graph is a sequence of nodes x1 -> x2 -> x3 -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. 
  The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.
  
  Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.
  
  Example 1:
  Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
  Output: 3
  Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
  
  Example 2:
  Input: colors = "a", edges = [[0,0]]
  Output: -1
  Explanation: There is a cycle from 0 to 0.
*/
public class Solution {
    public int LargestPathValue(string colors, int[][] edges) {
        int n = colors.Length;
        int result = 0;
        int processed = 0;

        var graph = new List<int>[n];
        Span<int> inDegree = stackalloc int[n];
        Queue<int> q = [];
        var count = new int[n, 26];

        for (int i = 0; i < n; ++i) {
            graph[i] = [];
        }

        foreach (int[] edge in edges) {
            int u = edge[0];
            int v = edge[1];
            graph[u].Add(v);
            ++inDegree[v];
        }

        for (int i = 0; i < n; ++i) {
            if (inDegree[i] == 0) {
                q.Enqueue(i);
            }
        }

        while (q.TryDequeue(out int o)) {
            ++processed;
            result = Math.Max(result, ++count[o, colors[o] - 'a']);
            foreach (int v in graph[o]) {
                for (int i = 0; i < 26; ++i) {
                    count[v, i] = Math.Max(count[v, i], count[o, i]);
                }
                if (--inDegree[v] == 0) {
                    q.Enqueue(v);
                }
            }
        }
        return processed == n ? result : -1;
    }
}
