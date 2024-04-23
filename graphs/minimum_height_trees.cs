public class Solution {
    public IList<int> FindMinHeightTrees(int n, int[][] edges) {
        if (n == 1) return [0];

        Dictionary<int, List<int>> graph = [];
        Span<int> degree = stackalloc int[n];

        foreach (int[] edge in edges) {
            int u = edge[0];
            int v = edge[1];

            degree[u]++;
            degree[v]++;

            if (!graph.ContainsKey(u)) graph[u] = [];
            if (!graph.ContainsKey(v)) graph[v] = [];

            graph[u].Add(v);
            graph[v].Add(u);
        }

        Queue<int> leaves = [];
        for (int i = 0; i < degree.Length; ++i) {
            if (degree[i] == 1) leaves.Enqueue(i);
        }

        int remaining = n;
        while (remaining > 2) {
            int leavesCount = leaves.Count;
            remaining -= leavesCount;

            for (int i = 0; i < leavesCount; ++i) {
                int leaf = leaves.Dequeue();
                foreach (int neighbor in graph[leaf]) {
                    if (--degree[neighbor] == 1) leaves.Enqueue(neighbor);
                }
            }
        }

        return leaves.ToArray();
    }
}
