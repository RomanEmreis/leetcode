/*
  3013. Divide an Array Into Subarrays With Minimum Cost II
  
  You are given a 0-indexed array of integers nums of length n, and two positive integers k and dist.
  The cost of an array is the value of its first element. For example, the cost of [1,2,3] is 1 while the cost of [3,4,1] is 3.
  You need to divide nums into k disjoint contiguous , such that the difference between the starting index of the second subarray and the starting index of the kth subarray 
  should be less than or equal to dist. In other words, if you divide nums into the subarrays nums[0..(i1 - 1)], nums[i1..(i2 - 1)], ..., nums[ik-1..(n - 1)], then ik-1 - i1 <= dist.
  
  Return the minimum possible sum of the cost of these subarrays.
  
  Example 1:
  Input: nums = [1,3,2,6,4,2], k = 3, dist = 3
  Output: 5
  Explanation: The best possible way to divide nums into 3 subarrays is: [1,3], [2,6,4], and [2]. This choice is valid because ik-1 - i1 is 5 - 2 = 3 
  which is equal to dist. The total cost is nums[0] + nums[2] + nums[5] which is 1 + 2 + 2 = 5.
  It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 5.
  
  Example 2:
  Input: nums = [10,1,2,2,2,1], k = 4, dist = 3
  Output: 15
  Explanation: The best possible way to divide nums into 4 subarrays is: [10], [1], [2], and [2,2,1]. This choice is valid because ik-1 - i1 is 3 - 1 = 2 
  which is less than dist. The total cost is nums[0] + nums[1] + nums[2] + nums[3] which is 10 + 1 + 2 + 2 = 15.
  The division [10], [1], [2,2,2], and [1] is not valid, because the difference between ik-1 and i1 is 5 - 1 = 4, which is greater than dist.
  It can be shown that there is no possible way to divide nums into 4 subarrays at a cost lower than 15.
  
  Example 3:
  Input: nums = [10,8,18,9], k = 3, dist = 1
  Output: 36
  Explanation: The best possible way to divide nums into 4 subarrays is: [10], [8], and [18,9]. This choice is valid because ik-1 - i1 is 2 - 1 = 1
  which is equal to dist.The total cost is nums[0] + nums[1] + nums[2] which is 10 + 8 + 18 = 36.
  The division [10], [8,18], and [9] is not valid, because the difference between ik-1 and i1 is 3 - 1 = 2, which is greater than dist.
  It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 36.
*/
using System.Runtime.CompilerServices;

public sealed class Solution {
    public long MinimumCost(int[] nums, int k, int dist) {
        int n = nums.Length;

        Container cnt = new(k - 2);
        for (int i = 1; i < k - 1; i++)
            cnt.Add(nums[i]);

        long res = cnt.Sum() + nums[k - 1];

        for (int i = k; i < n; i++) {
            int j = i - dist - 1;
            if (j > 0) {
                cnt.Remove(nums[j]);
            }
            cnt.Add(nums[i - 1]);
            res = Math.Min(res, cnt.Sum() + nums[i]);
        }

        return res + nums[0];
    }
}

public sealed class Container {
    private readonly int k;
    private readonly PriorityQueue<int, int> st1; // max-heap via priority=-x
    private readonly PriorityQueue<int, int> st2; // min-heap via priority=x

    // membership only for st1
    private readonly Dictionary<int, int> cnt1;

    // lazy deletions
    private readonly Dictionary<int, int> del1;
    private readonly Dictionary<int, int> del2;

    private int st1Size;
    private int st2Size;
    private long sm;

    public Container(int k, int capacityHint = 0) {
        this.k = k;

        st1 = new PriorityQueue<int, int>();
        st2 = new PriorityQueue<int, int>();

        cnt1 = new Dictionary<int, int>(capacityHint > 0 ? Math.Min(capacityHint, k * 2) : 0);

        del1 = new Dictionary<int, int>(capacityHint);
        del2 = new Dictionary<int, int>(capacityHint);

        st1Size = 0;
        st2Size = 0;
        sm = 0;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static void Inc(Dictionary<int, int> dict, int key) {
        if (dict.TryGetValue(key, out int v)) dict[key] = v + 1;
        else dict[key] = 1;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static void Dec(Dictionary<int, int> dict, int key) {
        int v = dict[key] - 1;
        if (v == 0) dict.Remove(key);
        else dict[key] = v;
    }

    private void Prune1() {
        while (st1.Count > 0) {
            int x = st1.Peek();
            if (del1.TryGetValue(x, out int d) && d > 0) {
                st1.Dequeue();
                if (d == 1) del1.Remove(x);
                else del1[x] = d - 1;
            }
            else break;
        }
    }

    private void Prune2() {
        while (st2.Count > 0) {
            int x = st2.Peek();
            if (del2.TryGetValue(x, out int d) && d > 0) {
                st2.Dequeue();
                if (d == 1) del2.Remove(x);
                else del2[x] = d - 1;
            }
            else break;
        }
    }

    private int ExtractMax1() {
        Prune1();
        int x = st1.Dequeue();
        Dec(cnt1, x);
        st1Size--;
        sm -= x;
        return x;
    }

    private int ExtractMin2() {
        Prune2();
        int x = st2.Dequeue();
        st2Size--;
        return x;
    }

    private int Min2() {
        Prune2();
        return st2.Peek();
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private void Insert1(int x) {
        st1.Enqueue(x, -x);
        Inc(cnt1, x);
        st1Size++;
        sm += x;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private void Insert2(int x) {
        st2.Enqueue(x, x);
        st2Size++;
    }

    private void Adjust() {
        while (st1Size < k && st2Size > 0) {
            int x = ExtractMin2();
            Insert1(x);
        }

        while (st1Size > k) {
            int x = ExtractMax1();
            Insert2(x);
        }
    }

    public void Add(int x) {
        if (st2Size > 0) {
            int boundary = Min2(); // min(st2)
            if (x >= boundary) Insert2(x);
            else Insert1(x);
        } else {
            Insert1(x);
        }
        Adjust();
    }

    public void Remove(int x) {
        if (cnt1.TryGetValue(x, out int c1) && c1 > 0) {
            Dec(cnt1, x);
            st1Size--;
            sm -= x;
            Inc(del1, x);
        } else {
            st2Size--;
            Inc(del2, x);
        }
        Adjust();
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public long Sum() => sm;
}
