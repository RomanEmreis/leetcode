/*
  3203. Find Minimum Diameter After Merging Two Trees
  
  There exist two undirected trees with n and m nodes, numbered from 0 to n - 1 and from 0 to m - 1, respectively. You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree.
  You must connect one node from the first tree with another node from the second tree with an edge.
  Return the minimum possible diameter of the resulting tree.
  
  The diameter of a tree is the length of the longest path between any two nodes in the tree.
  
  Example 1:
  Input: edges1 = [[0,1],[0,2],[0,3]], edges2 = [[0,1]]
  Output: 3
  Explanation:
  We can obtain a tree of diameter 3 by connecting node 0 from the first tree with any node from the second tree.
  
  Example 2:
  Input: edges1 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]], edges2 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]
  Output: 5
  Explanation:
  We can obtain a tree of diameter 5 by connecting node 0 from the first tree with node 0 from the second tree.
*/
public class Solution {
    public int MinimumDiameterAfterMerge(int[][] edges1, int[][] edges2) {
        int d1 = GetDiameter(edges1);
        int d2 = GetDiameter(edges2);
        return Math.Max(Math.Max(d1, d2), (d1 + 1) / 2 + (d2 + 1) / 2 + 1);
    }

    private static int GetDiameter(int[][] edges) {
        int n = edges.Length + 1;
        var graph = new List<int>[n];
        for (int i = 0; i < graph.Length; ++i) {
            graph[i] = [];
        }

        foreach (int[] edge in edges) {
            int u = edge[0], v = edge[1];
            graph[u].Add(v);
            graph[v].Add(u);
        }

        int result = 0, a = 0;
        Dfs(0, -1, 0);
        Dfs(a, -1, 0);
        return result;

        void Dfs(int i, int fa, int t) {
            foreach (int j in graph[i]) {
                if (j != fa) Dfs(j, i, t + 1);
            }
            if (result < t) {
                result = t;
                a = i;
            }
        }
    }
}
