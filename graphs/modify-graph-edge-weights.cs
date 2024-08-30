/*
  You are given an undirected weighted connected graph containing n nodes labeled from 0 to n - 1, and an integer array edges where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.
  Some edges have a weight of -1 (wi = -1), while others have a positive weight (wi > 0).
  Your task is to modify all edges with a weight of -1 by assigning them positive integer values in the range [1, 2 * 109] so that the shortest distance between the nodes source and destination becomes equal to an integer target.
  If there are multiple modifications that make the shortest distance between source and destination equal to target, any of them will be considered correct.
  
  Return an array containing all edges (even unmodified ones) in any order if it is possible to make the shortest distance from source to destination equal to target, or an empty array if it's impossible.
  
  Note: You are not allowed to modify the weights of edges with initial positive weights.
  
  Example 1:
    Input: n = 5, edges = [[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]], source = 0, destination = 1, target = 5
    Output: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
    Explanation: The graph above shows a possible modification to the edges, making the distance from 0 to 1 equal to 5.

  Example 2:
    Input: n = 3, edges = [[0,1,-1],[0,2,5]], source = 0, destination = 2, target = 6
    Output: []
    Explanation: The graph above contains the initial edges. It is not possible to make the distance from 0 to 2 equal to 6 by modifying the edge with weight -1. So, an empty array is returned.
  
  Example 3:
    Input: n = 4, edges = [[1,0,4],[1,2,3],[2,3,5],[0,3,-1]], source = 0, destination = 2, target = 6
    Output: [[1,0,4],[1,2,3],[2,3,5],[0,3,1]]
    Explanation: The graph above shows a modified graph having the shortest distance from 0 to 2 as 6.
*/
public class Solution {
    private const int kMax = 2_000_000_000;

    public int[][] ModifiedGraphEdges(int n, int[][] edges, int source, int destination, int target) {
        List<(int, int)>[] graph = new List<(int, int)>[n];
        for (int i = 0; i < n; ++i) {
            graph[i] = [];
        }

        foreach (int[] edge in edges) {
            int u = edge[0];
            int v = edge[1];
            int w = edge[2];
            if (w == -1) continue;

            graph[u].Add((v, w));
            graph[v].Add((u, w));
        }

        int distToDestination = Dijkstra(graph, source, destination);
        if (distToDestination < target) return [];

        if (distToDestination == target) {
            foreach (int[] edge in edges) {
                if (edge[2] == -1) edge[2] = kMax;
            }
            return edges;
        }

        for (int i = 0; i < edges.Length; ++i) {
            int u = edges[i][0];
            int v = edges[i][1];
            ref int w = ref edges[i][2];
            if (w != -1) continue;
            w = 1;
            graph[u].Add((v, 1));
            graph[v].Add((u, 1));

            distToDestination = Dijkstra(graph, source, destination);
            if (distToDestination <= target) {
                w += target - distToDestination;
                for (int j = i + 1; j < edges.Length; ++j) {
                    if (edges[j][2] == -1) edges[j][2] = kMax;
                }
                return edges;
            }
        }

        return [];
    }

    private static int Dijkstra(List<(int, int)>[] graph, int src, int dst) {
        Span<int> dist = stackalloc int[graph.Length];
        dist.Fill(int.MaxValue);

        PriorityQueue<(int, int), int> minHeap = new();

        dist[src] = 0;
        minHeap.Enqueue((dist[src], src), dist[src]);

        while (minHeap.TryDequeue(out var item, out _)) {
            var (d, u) = item;
            if (d > dist[u]) continue;

            foreach (var (v, w) in graph[u]) {
                if (d + w < dist[v]) {
                    dist[v] = d + w;
                    minHeap.Enqueue((dist[v], v), dist[v]);
                }
            }
        }

        return dist[dst];
    }
}
