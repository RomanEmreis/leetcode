/*
  3108. Minimum Cost Walk in Weighted Graph
  
  There is an undirected weighted graph with n vertices labeled from 0 to n - 1.
  
  You are given the integer n and an array edges, where edges[i] = [ui, vi, wi] indicates that there is an edge between vertices ui and vi with a weight of wi.
  A walk on a graph is a sequence of vertices and edges. The walk starts and ends with a vertex, and each edge connects the vertex that comes before 
  it and the vertex that comes after it. It's important to note that a walk may visit the same edge or vertex more than once.
  The cost of a walk starting at node u and ending at node v is defined as the bitwise AND of the weights of the edges traversed during the walk. In other words, 
  if the sequence of edge weights encountered during the walk is w0, w1, w2, ..., wk, then the cost is calculated as w0 & w1 & w2 & ... & wk, where & denotes the bitwise AND operator.
  You are also given a 2D array query, where query[i] = [si, ti]. For each query, you need to find the minimum cost of the walk starting at vertex si and ending at vertex ti. If there exists no such walk, the answer is -1.
  
  Return the array answer, where answer[i] denotes the minimum cost of a walk for query i.
  
  Example 1:
  Input: n = 5, edges = [[0,1,7],[1,3,7],[1,2,1]], query = [[0,3],[3,4]]
  Output: [1,-1]
  Explanation:
  To achieve the cost of 1 in the first query, we need to move on the following edges: 0->1 (weight 7), 1->2 (weight 1), 2->1 (weight 1), 1->3 (weight 7).
  In the second query, there is no walk between nodes 3 and 4, so the answer is -1.
  
  Example 2:
  Input: n = 3, edges = [[0,2,7],[0,1,15],[1,2,6],[1,2,1]], query = [[1,2]]
  Output: [0]
  Explanation:
  To achieve the cost of 0 in the first query, we need to move on the following edges: 1->2 (weight 1), 2->1 (weight 6), 1->2 (weight 1).
*/
public class Solution {
    public int[] MinimumCost(int n, int[][] edges, int[][] query) {
        Uf uf = new(n);
        foreach (int[] e in edges) {
            uf.Union(e[0], e[1]);
        }

        Span<int> graph = stackalloc int[n];
        graph.Fill(-1);
        foreach (int[] e in edges) {
            int root = uf.Find(e[0]);
            graph[root] &= e[2];
        }

        int m = query.Length;
        int[] result = new int[m];
        for (int i = 0; i < m; ++i) {
            int s = query[i][0];
            int t = query[i][1];
            if (s != t) {
                int x = uf.Find(s);
                int y = uf.Find(t);
                result[i] = x == y ? graph[x] : -1;
            } else {
                result[i] = 0;
            }
        }
        return result;
    }
}

ref struct Uf {
    private readonly int[] p;
    private readonly int[] size;

    public Uf(int n) {
        p = new int[n];
        size = new int[n];

        for (int i = 0; i < n; ++i) {
            p[i] = i;
            size[i] = 1;
        }
    }

    public int Find(int x) {
        if (p[x] != x) {
            p[x] = Find(p[x]);
        }
        return p[x];
    }

    public bool Union(int x, int y) {
        int px = Find(x);
        int py = Find(y);

        if (px == py) return false;

        if (size[px] > size[py]) {
            p[py] = px;
            size[px] += size[py];
        } else {
            p[px] = py;
            size[py] += size[px];
        }

        return true;
    }

    public int Size(int x) => size[Find(x)];
}
