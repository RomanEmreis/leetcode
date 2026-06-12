/*
  3559. Number of Ways to Assign Edge Weights II
  
  There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer array edges of length n - 1, 
  where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
  Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
  The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
  
  You are given a 2D integer array queries. For each queries[i] = [ui, vi], determine the number of ways to assign weights to edges 
  in the path such that the cost of the path between ui and vi is odd.
  
  Return an array answer, where answer[i] is the number of valid assignments for queries[i].
  
  Since the answer may be large, apply modulo 109 + 7 to each answer[i].
  
  Note: For each query, disregard all edges not in the path between node ui and vi.
  
  Example 1:
  Input: edges = [[1,2]], queries = [[1,1],[1,2]]
  Output: [0,1]
  Explanation:
      Query [1,1]: The path from Node 1 to itself consists of no edges, so the cost is 0. Thus, the number of valid assignments is 0.
      Query [1,2]: The path from Node 1 to Node 2 consists of one edge (1 → 2). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.
  
  Example 2:
  Input: edges = [[1,2],[1,3],[3,4],[3,5]], queries = [[1,4],[3,4],[2,5]]
  Output: [2,1,4]
  Explanation:
      Query [1,4]: The path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4). Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.
      Query [3,4]: The path from Node 3 to Node 4 consists of one edge (3 → 4). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.
      Query [2,5]: The path from Node 2 to Node 5 consists of three edges (2 → 1, 1 → 3, and 3 → 5). Assigning (1,2,2), (2,1,2), (2,2,1), or (1,1,1) makes the cost odd. Thus, the number of valid assignments is 4.
*/
using System.Numerics;

public sealed class Solution {
    private const int MOD = 1_000_000_007;

    public int[] AssignEdgeWeights(int[][] edges, int[][] queries) {
        int n = edges.Length + 1;
        var start = new int[n + 1];

        foreach (var e in edges) { 
            start[e[0]]++; 
            start[e[1]]++;
        }
        
        for (int i = 1, acc = 0; i <= n; i++) { 
            int d = start[i]; 
            start[i] = acc; 
            acc += d;
        }
        
        var adj = new int[2 * (n - 1)];
        foreach (var e in edges) {
            adj[start[e[0]]++] = e[1];
            adj[start[e[1]]++] = e[0];
        }

        int m = 2 * n - 1;
        var ed = new int[m];
        var first = new int[n + 1];
        var depth = new int[n + 1];
        var parent = new int[n + 1];
        var stNode = new int[n];
        var stEdge = new int[n];

        int sp = 0, t = 0;

        stNode[0] = 1;
        stEdge[0] = start[0];
        ed[t++] = 0;

        while (sp >= 0) {
            int u = stNode[sp];
            int e = stEdge[sp], end = start[u];
            int child = 0;

            while (e < end) {
                int v = adj[e++];
                if (v != parent[u]) { 
                    child = v; 
                    break;
                }
            }

            stEdge[sp] = e;
            
            if (child != 0) {
                parent[child] = u;
                depth[child] = depth[u] + 1;
                first[child] = t;
                ed[t++] = depth[child];
                sp++;
                stNode[sp] = child;
                stEdge[sp] = start[child - 1];
            } else if (--sp >= 0) {
                ed[t++] = depth[stNode[sp]];
            }
        }

        int LOG = BitOperations.Log2((uint)m) + 1;
        var table = new int[LOG][];

        table[0] = ed;
        for (int k = 1; k < LOG; k++) {
            int half = 1 << (k - 1), len = m - (1 << k) + 1;
            var prev = table[k - 1];
            var cur = new int[len];

            for (int i = 0; i < len; i++)
                cur[i] = Math.Min(prev[i], prev[i + half]);

            table[k] = cur;
        }

        var pow2 = new int[2 * n];
        pow2[0] = 1;
        for (int i = 1; i < pow2.Length; i++)
            pow2[i] = (int)((long)pow2[i - 1] * 2 % MOD);

        var ans = new int[queries.Length];
        for (int i = 0; i < queries.Length; i++) {
            int u = queries[i][0], v = queries[i][1];
            if (u == v) 
                continue;
            
            int l = first[u], r = first[v];
            if (l > r) 
                (l, r) = (r, l);
            
            int k = BitOperations.Log2((uint)(r - l + 1));
            int minDepth = Math.Min(table[k][l], table[k][r - (1 << k) + 1]);
            
            ans[i] = pow2[depth[u] + depth[v] - 2 * minDepth - 1];
        }
        return ans;
    }
}
