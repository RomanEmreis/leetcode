/*
3558. Number of Ways to Assign Edge Weights I

There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer array edges of length n - 1, 
where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
  The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
  Select any one node x at the maximum depth. Return the number of ways to assign edge weights in the path from node 1 to x such that its total cost is odd.
  Since the answer may be large, return it modulo 109 + 7.

Note: Ignore all edges not in the path from node 1 to x.

Example 1:
Input: edges = [[1,2]]
Output: 1
Explanation:
    The path from Node 1 to Node 2 consists of one edge (1 → 2).
    Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.

Example 2:
Input: edges = [[1,2],[1,3],[3,4],[3,5]]
Output: 2
Explanation:
    The maximum depth is 2, with nodes 4 and 5 at the same depth. Either node can be selected for processing.
    For example, the path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4).
    Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.
*/
public class Solution {
    private const int MOD = 1_000_000_007;

    public int AssignEdgeWeights(int[][] edges) {
        int n = edges.Length + 1;
        if (n == 1) 
            return 0;

        var head = new int[n + 1];
        var next = new int[2 * (n - 1)];
        var to   = new int[2 * (n - 1)];
        Array.Fill(head, -1);

        int idx = 0;
        foreach (var e in edges) {
            int u = e[0], v = e[1];
            to[idx] = v; next[idx] = head[u]; head[u] = idx++;
            to[idx] = u; next[idx] = head[v]; head[v] = idx++;
        }

        var queue = new int[n];
        var depth = new int[n + 1];
        Array.Fill(depth, -1);

        int qh = 0, qt = 0;
        queue[qt++] = 1; depth[1] = 0;
        int last = 1;

        while (qh < qt) {
            int u = queue[qh++];
            last = u;
            for (int e = head[u]; e != -1; e = next[e]) {
                int v = to[e];
                if (depth[v] < 0) { 
                    depth[v] = depth[u] + 1; 
                    queue[qt++] = v;
                }
            }
        }

        int p = depth[last] - 1;
        long res = 1, b = 2;

        while (p > 0) {
            if ((p & 1) == 1) 
                res = res * b % MOD;

            b = b * b % MOD;
            p >>= 1;
        }

        return (int) res;
    }
}
