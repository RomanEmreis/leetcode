/*
  2097. Valid Arrangement of Pairs

  You are given a 0-indexed 2D integer array pairs where pairs[i] = [starti, endi]. An arrangement of pairs is valid if for every index i where 1 <= i < pairs.length, we have endi-1 == starti.
  Return any valid arrangement of pairs.
  
  Note: The inputs will be generated such that there exists a valid arrangement of pairs.
  
  Example 1:
  Input: pairs = [[5,1],[4,5],[11,9],[9,4]]
  Output: [[11,9],[9,4],[4,5],[5,1]]
  Explanation:
  This is a valid arrangement since endi-1 always equals starti.
  end0 = 9 == 9 = start1 
  end1 = 4 == 4 = start2
  end2 = 5 == 5 = start3

  Example 2:
  Input: pairs = [[1,3],[3,2],[2,1]]
  Output: [[1,3],[3,2],[2,1]]
  Explanation:
  This is a valid arrangement since endi-1 always equals starti.
  end0 = 3 == 3 = start1
  end1 = 2 == 2 = start2
  The arrangements [[2,1],[1,3],[3,2]] and [[3,2],[2,1],[1,3]] are also valid.
  
  Example 3:
  Input: pairs = [[1,2],[1,3],[2,1]]
  Output: [[1,2],[2,1],[1,3]]
  Explanation:
  This is a valid arrangement since endi-1 always equals starti.
  end0 = 2 == 2 = start1
  end1 = 1 == 1 = start2
*/
public class Solution {
    private readonly Dictionary<int, List<int>> adj = [];
    private readonly Dictionary<int, int> inDegree = [], outDegree = [];

    public int[][] ValidArrangement(int[][] pairs) {
        foreach (int[] pair in pairs) {
            int u = pair[0];
            int v = pair[1];

            if (adj.ContainsKey(u)) adj[u].Add(v);
            else adj[u] = [v];

            if (outDegree.ContainsKey(u)) ++outDegree[u];
            else outDegree[u] = 1;

            if (inDegree.ContainsKey(v)) ++inDegree[v];
            else inDegree[v] = 1;
        }

        int start = GetStartNode();

        List<int> path = [];
        GetEulerianPath(start, path);

        List<int[]> result = [];

        for (int i = path.Count - 1; i > 0; --i) {
            result.Add([path[i], path[i - 1]]);
        }

        return [.. result];
    }

    private int GetStartNode() {
        int start = 0;
        foreach (var (node, degree) in outDegree) {
            if (degree - inDegree.GetValueOrDefault(node) == 1) return node;
            if (degree > 0) start = node;
        }
        return start;
    }

    private void GetEulerianPath(int current, List<int> path) {
        while (outDegree.TryGetValue(current, out int val) && val > 0) {
            --val;
            outDegree[current] = val;
            int next = adj[current][val];
            GetEulerianPath(next, path);
        }
        path.Add(current);
    }
}
