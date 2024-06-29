/*
  You are given a positive integer n representing the number of nodes of a Directed Acyclic Graph (DAG). The nodes are numbered from 0 to n - 1 (inclusive).
  You are also given a 2D integer array edges, where edges[i] = [fromi, toi] denotes that there is a unidirectional edge from fromi to toi in the graph.

  Return a list answer, where answer[i] is the list of ancestors of the ith node, sorted in ascending order.

  A node u is an ancestor of another node v if u can reach v via a set of edges.

  Example:
    Input: n = 8, edgeList = [[0,3],[0,4],[1,3],[2,4],[2,7],[3,5],[3,6],[3,7],[4,6]]
    Output: [[],[],[],[0,1],[0,2],[0,1,3],[0,1,2,3,4],[0,1,2,3]]
  Explanation:
    The above diagram represents the input graph.
    - Nodes 0, 1, and 2 do not have any ancestors.
    - Node 3 has two ancestors 0 and 1.
    - Node 4 has two ancestors 0 and 2.
    - Node 5 has three ancestors 0, 1, and 3.
    - Node 6 has five ancestors 0, 1, 2, 3, and 4.
    - Node 7 has four ancestors 0, 1, 2, and 3.
*/
public class Solution {
    public IList<IList<int>> GetAncestors(int n, int[][] edges) {
        List<int>[] result = new List<int>[n];
        List<int>[] graph = new List<int>[n];

        for (int i = 0; i < n; ++i) graph[i] = [];
        foreach (int[] edge in edges) graph[edge[1]].Add(edge[0]);

        Span<bool> visited = stackalloc bool[n];
        for (int i = 0; i < n; ++i) {
            if (!visited[i]) Dfs(i, graph, result, ref visited);
        }

        return result;
    }

    private static void Dfs(int node, List<int>[] graph, List<int>[] result, ref Span<bool> visited) {
        visited[node] = true;
        List<int> ancestors = [];

        foreach (int parent in graph[node]) {
            if (!visited[parent]) Dfs(parent, graph, result, ref visited);
            ancestors.Add(parent);
            ancestors.AddRange(result[parent]);
        }

        result[node] = ancestors.Distinct().ToList();
        result[node].Sort();
    }
}
