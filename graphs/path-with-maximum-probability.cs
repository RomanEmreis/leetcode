/*
  You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
  
  Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
  
  If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.

  Example 1:
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
    Output: 0.25000
    Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
  
  Example 2:
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
    Output: 0.30000
*/
public class Solution {
    public double MaxProbability(int n, int[][] edges, double[] succProb, int start_node, int end_node) {
        List<(int, double)>[] graph = new List<(int, double)>[n];
        for (int i = 0; i < n; ++i) graph[i] = [];

        for (int i = 0; i < edges.Length; ++i) {
            int u = edges[i][0];
            int v = edges[i][1];
            double prob = succProb[i];

            graph[u].Add((v, -Math.Log(prob)));
            graph[v].Add((u, -Math.Log(prob)));
        }

        Span<double> dist = stackalloc double[n];
        dist.Fill(double.PositiveInfinity);
        dist[start_node] = 0;

        PriorityQueue<int, double> pq = new();
        pq.Enqueue(start_node, 0);

        while (pq.TryDequeue(out int node, out double currDist)) {
            if (currDist > dist[node]) continue;

            foreach (var (neighbor, weight) in graph[node]) {
                double newDist = dist[node] + weight;
                if (newDist < dist[neighbor]) {
                    dist[neighbor] = newDist;
                    pq.Enqueue(neighbor, newDist);
                }
            }
        }

        return dist[end_node] == double.PositiveInfinity ? 0.0 : Math.Exp(-dist[end_node]);
    }
}
