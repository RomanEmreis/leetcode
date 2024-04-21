public class Solution {
    public bool ValidPath(int n, int[][] edges, int source, int destination) {
        if (edges.Length == 0 && n == 1) return true;

        Dictionary<int, List<int>> graph = [];
        for (int i = 0; i < edges.Length; ++i) {
            int u = edges[i][0];
            int v = edges[i][1];

            if (!graph.ContainsKey(u)) graph[u] = [];
            if (!graph.ContainsKey(v)) graph[v] = [];

            graph[u].Add(v);
            graph[v].Add(u);
        }

        HashSet<int> visited = [];
        Queue<int> q = [];
        q.Enqueue(source);
        while (q.TryDequeue(out int current)) {
            if (current == destination) return true;
            foreach (int neighbor in graph[current]) {
                if (visited.Add(neighbor)) q.Enqueue(neighbor);
            }
        }

        return false;
    }
}
