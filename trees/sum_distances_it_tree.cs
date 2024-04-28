/*
  There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

  You are given the integer n and the array edges where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

  Return an array answer of length n where answer[i] is the sum of the distances between the ith node in the tree and all other nodes.
*/
public class Solution {
    private readonly Dictionary<int, List<int>> graph = [];
    public int[] SumOfDistancesInTree(int n, int[][] edges) {
        if (n == 1) return [0];

        graph.Clear();

        List<int> e = null;
        foreach (int[] edge in edges) {
            int x = edge[0];
            int y = edge[1];

            if (graph.TryGetValue(x, out e)) e.Add(y);
            else graph[x] = [y];

            if (graph.TryGetValue(y, out e)) e.Add(x);
            else graph[y] = [x];
        }

        int[] result = new int[n];
        Span<int> count = stackalloc int[n];

        for (int i = 0; i < count.Length; ++i) ++count[i];

        Dfs1(0, -1, count, result);
        Dfs2(0, -1, count, result);

        return result;
    }

    private void Dfs1(in int node, in int parent, in Span<int> count, int[] result) {
        List<int> neighbors = graph[node];
        for (int i = 0; i < neighbors.Count; ++i) {
            int neighbor = neighbors[i];
            if (neighbor == parent) continue;

            Dfs1(neighbor, node, count, result);

            count[node] += count[neighbor];
            result[node] += result[neighbor] + count[neighbor];
        }
    }

    private void Dfs2(in int node, in int parent, in Span<int> count, int[] result) {
        List<int> neighbors = graph[node];
        for (int i = 0; i < neighbors.Count; ++i) {
            int neighbor = neighbors[i];
            if (neighbor == parent) continue;

            result[neighbor] = result[node] - count[neighbor] + (count.Length - count[neighbor]);
            Dfs2(neighbor, node, count, result);
        }
    }
}
