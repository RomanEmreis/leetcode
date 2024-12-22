/*
  2940. Find Building Where Alice and Bob Can Meet
  
  You are given a 0-indexed array heights of positive integers, where heights[i] represents the height of the ith building.
  If a person is in building i, they can move to any other building j if and only if i < j and heights[i] < heights[j].
  You are also given another array queries where queries[i] = [ai, bi]. On the ith query, Alice is in building ai while Bob is in building bi.
  Return an array ans where ans[i] is the index of the leftmost building where Alice and Bob can meet on the ith query. If Alice and Bob cannot move to a common building on query i, set ans[i] to -1.
  
  Example 1:
  Input: heights = [6,4,8,5,2,7], queries = [[0,1],[0,3],[2,4],[3,4],[2,2]]
  Output: [2,5,-1,5,2]
  Explanation: In the first query, Alice and Bob can move to building 2 since heights[0] < heights[2] and heights[1] < heights[2]. 
  In the second query, Alice and Bob can move to building 5 since heights[0] < heights[5] and heights[3] < heights[5]. 
  In the third query, Alice cannot meet Bob since Alice cannot move to any other building.
  In the fourth query, Alice and Bob can move to building 5 since heights[3] < heights[5] and heights[4] < heights[5].
  In the fifth query, Alice and Bob are already in the same building.  
  For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
  For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.
  
  Example 2:
  Input: heights = [5,3,8,2,6,1,4,6], queries = [[0,7],[3,5],[5,2],[3,0],[1,6]]
  Output: [7,6,-1,4,6]
  Explanation: In the first query, Alice can directly move to Bob's building since heights[0] < heights[7].
  In the second query, Alice and Bob can move to building 6 since heights[3] < heights[6] and heights[5] < heights[6].
  In the third query, Alice cannot meet Bob since Bob cannot move to any other building.
  In the fourth query, Alice and Bob can move to building 4 since heights[3] < heights[4] and heights[0] < heights[4].
  In the fifth query, Alice can directly move to Bob's building since heights[1] < heights[6].
  For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
  For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.
*/
public ref struct BinaryIndexTree {
    private const int Inf = 1 << 30;

    private int _n;
    private Span<int> _c;

    public BinaryIndexTree(int n) {
        _n = n;
        _c = new int[n + 1];
        _c.Fill(Inf);
    }

    public void Update(int x, int v) {
        while (x <= _n) {
            if (v < _c[x]) _c[x] = v;
            x += x & -x;
        }
    }

    public int Query(int x) {
        int min = Inf;
        while (x > 0) {
            if (_c[x] < min) min = _c[x];
            x -= x & -x; 
        }
        return min == Inf ? -1 : min;
    }
}

public class Solution {
    public int[] LeftmostBuildingQueries(int[] heights, int[][] queries) {
        for (int i = 0; i < queries.Length; ++i) {
            if (queries[i][0] > queries[i][1]) {
                (queries[i][0], queries[i][1]) = (queries[i][1], queries[i][0]);
            }
        }

        int m = queries.Length;
        Span<int> idx = new int[m];
        for (int i = 0; i < queries.Length; ++i) {
            idx[i] = i;
        }
        idx.Sort((i, j) => queries[j][1].CompareTo(queries[i][1]));

        int n = heights.Length;
        Span<int> s = new int[n];
        for (int i = 0; i < s.Length; ++i) {
            s[i] = heights[i];
        }
        s.Sort();

        int[] result = new int[m];
        int j = n - 1;
        BinaryIndexTree tree = new(n);
        foreach (int i in idx) {
            int l = queries[i][0], r = queries[i][1];
            while (j > r) {
                int k = n - s.BinarySearch(heights[j]) + 1;
                tree.Update(k, j);
                --j;
            }

            if (l == r || heights[l] < heights[r]) {
                result[i] = r;
            } else {
                int k = n - s.BinarySearch(heights[l]);
                result[i] = tree.Query(k);
            }
        }
        return result;
    }
}
