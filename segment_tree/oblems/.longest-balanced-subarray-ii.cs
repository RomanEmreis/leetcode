/*
  3721. Longest Balanced Subarray II
  
  You are given an integer array nums.
  A is called balanced if the number of distinct even numbers in the subarray is equal to the number of distinct odd numbers.
  
  Return the length of the longest balanced subarray.
  
  Example 1:
  Input: nums = [2,5,4,3]
  Output: 4
  Explanation:
      The longest balanced subarray is [2, 5, 4, 3].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [5, 3]. Thus, the answer is 4.
  
  Example 2:
  Input: nums = [3,2,2,5,4]
  Output: 5
  Explanation:
      The longest balanced subarray is [3, 2, 2, 5, 4].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [3, 5]. Thus, the answer is 5.
  
  Example 3:
  Input: nums = [1,2,3,2]
  Output: 3
  Explanation:
      The longest balanced subarray is [2, 3, 2].
      It has 1 distinct even number [2] and 1 distinct odd number [3]. Thus, the answer is 3.
*/
public class Solution {
    public int LongestBalanced(int[] nums) {
        int n = nums.Length;
        var st = new SegmentTree(n);

        int now = 0;
        int res = 0;

        Dictionary<int, int> last = [];

        for (int i = 1; i <= n; ++i) {
            int x = nums[i - 1];
            int dt = (x & 1) > 0 ? 1 : -1;

            if (last.TryGetValue(x, out int l)) {
                st.Modify(1, l, n, -dt);
                now -= dt;
            }

            last[x] = i;
            st.Modify(1, i, n, dt);
            now += dt;

            int j = st.Query(1, now);
            res = Math.Max(res, i - j);
        }

        return res;
    }
}

public struct Node {
    public int l, r;
    public int min, max;
    public int lazy;

    public Node() {
        l = 0;
        r = 0;
        min = 0;
        max = 0;
        lazy = 0;
    }
}

public sealed class SegmentTree {
    private readonly Node[] nodes;

    public SegmentTree(int n) {
        nodes = new Node[n << 2];
        Build(1, 0, n);
    }

    public void Modify(int u, int l, int r, int v) {
        if (nodes[u].l >= l && nodes[u].r <= r) {
            Apply(u, v);
            return;
        }

        PushDown(u);

        int mid = (nodes[u].l + nodes[u].r) >> 1;
        if (l <= mid)
            Modify(u << 1, l, r, v);

        if (r > mid)
            Modify(u << 1 | 1, l, r, v);

        PushUp(u);
    }

    public int Query(int u, int target) {
        if (nodes[u].l == nodes[u].r) {
            return nodes[u].l;
        }

        PushDown(u);

        int lc = u << 1;
        int rc = u << 1 | 1;
        if (nodes[lc].min <= target && target <= nodes[lc].max)
            return Query(lc, target);

        return Query(rc, target);
    }

    private void Build(int u, int l, int r) {
        nodes[u].l = l;
        nodes[u].r = r;

        if (l == r)
            return;

        int mid = (l + r) >> 1;

        Build(u << 1, l, mid);
        Build(u << 1 | 1, mid + 1, r);
    }

    private void Apply(int u, int v) {
        nodes[u].min += v;
        nodes[u].max += v;
        nodes[u].lazy += v;
    }

    private void PushUp(int u) {
        nodes[u].min = Math.Min(nodes[u << 1].min, nodes[u << 1 | 1].min);
        nodes[u].max = Math.Max(nodes[u << 1].max, nodes[u << 1 | 1].max);
    }

    private void PushDown(int u) {
        if (nodes[u].lazy != 0) {
            Apply(u << 1, nodes[u].lazy);
            Apply(u << 1 | 1, nodes[u].lazy);
            nodes[u].lazy = 0;
        }
    }
}
