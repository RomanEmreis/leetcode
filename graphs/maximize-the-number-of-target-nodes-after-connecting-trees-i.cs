/*
  3372. Maximize the Number of Target Nodes After Connecting Trees I
  
  There exist two undirected trees with n and m nodes, with distinct labels in ranges [0, n - 1] and [0, m - 1], respectively.
  You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that 
  there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree. You are also given an integer k.
  Node u is target to node v if the number of edges on the path from u to v is less than or equal to k. Note that a node is always target to itself.
  
  Return an array of n integers answer, where answer[i] is the maximum possible number of nodes target to node i of the first tree if you have to connect one node from the first tree to another node in the second tree.
  
  Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
  
  Example 1:
  Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2
  Output: [9,7,9,8,8]
  Explanation:
  For i = 0, connect node 0 from the first tree to node 0 from the second tree.
  For i = 1, connect node 1 from the first tree to node 0 from the second tree.
  For i = 2, connect node 2 from the first tree to node 4 from the second tree.
  For i = 3, connect node 3 from the first tree to node 4 from the second tree.
  For i = 4, connect node 4 from the first tree to node 4 from the second tree.
  
  Example 2:
  Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1
  Output: [6,3,3,3,3]
  Explanation:
  For every i, connect node i of the first tree with any node of the second tree.
*/
public class Solution {
    public int[] MaxTargetNodes(int[][] edges1, int[][] edges2, int k) {
        var graph2 = CreateGraph(edges2);
        int m = graph2.Length;
        
        int tmp = 0;
        for (int i = 0; i < m; ++i) {
            int count = Dfs(graph2, i, -1, k - 1);
            if (count > tmp) tmp = count;
        }

        var graph1 = CreateGraph(edges1);
        int n = graph1.Length;

        Span<int> result = stackalloc int[n];
        result.Fill(tmp);

        for (int i = 0; i < result.Length; ++i) {
            result[i] += Dfs(graph1, i, -1, k);
        }

        return result.ToArray();

        static List<int>[] CreateGraph(int[][] edges) {
            int n = edges.Length + 1;
            var graph = new List<int>[n];

            for (int i = 0; i < graph.Length; ++i) {
                graph[i] = [];
            }

            foreach (int[] edge in edges) {
                int u = edge[0];
                int v = edge[1];

                graph[u].Add(v);
                graph[v].Add(u);
            }

            return graph;
        }

        static int Dfs(List<int>[] graph, int a, int fa, int d) {
            if (d < 0) return 0;

            int count = 1;
            foreach (int b in graph[a]) {
                if (b != fa) {
                    count += Dfs(graph, b, a, d - 1);
                }
            }

            return count;
        }
    }
}
