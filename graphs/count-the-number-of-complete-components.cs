/*
  2685. Count the Number of Complete Components
  
  You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1. You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting vertices ai and bi.
  Return the number of complete connected components of the graph.
  
  A connected component is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
  A connected component is said to be complete if there exists an edge between every pair of its vertices.
  
  Example 1:
  Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
  Output: 3
  Explanation: From the picture above, one can see that all of the components of this graph are complete.
  
  Example 2:
  Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
  Output: 1
  Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between every pair of two vertices. On the other hand,
  the component containing vertices 3, 4, and 5 is not complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.
*/
public class Solution {
    public int CountCompleteComponents(int n, int[][] edges) {
        var graph = new List<int>[n];
        Span<bool> visited = stackalloc bool[n];

        for (int i = 0; i < graph.Length; ++i) {
            graph[i] = [];
        }

        foreach (var edge in edges) {
            int u = edge[0];
            int v = edge[1];
            graph[u].Add(v);
            graph[v].Add(u);
        }

        int result = 0;

        for (int i = 0; i < n; ++i) {
            if (!visited[i]) {
                var (a, b) = Dfs(i, ref visited);
                if (a * (a - 1) == b) ++result;
            }
        }

        return result;

        (int, int) Dfs(int i, ref Span<bool> visited) {
            visited[i] = true;
            int x = 1;
            int y = graph[i].Count;

            foreach (int j in graph[i]) {
                if (!visited[j]) {
                    var (a, b) = Dfs(j, ref visited);
                    x += a;
                    y += b;
                }
            }

            return (x, y);
        }
    }
}
