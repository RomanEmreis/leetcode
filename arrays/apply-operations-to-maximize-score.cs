/*
  2818. Apply Operations to Maximize Score
  
  You are given an array nums of n positive integers and an integer k.
  
  Initially, you start with a score of 1. You have to maximize your score by applying the following operation at most k times:
  
  Choose any non-empty subarray nums[l, ..., r] that you haven't chosen previously.
  Choose an element x of nums[l, ..., r] with the highest prime score. If multiple such elements exist, choose the one with the smallest index.
  Multiply your score by x.
  Here, nums[l, ..., r] denotes the subarray of nums starting at index l and ending at the index r, both ends being inclusive.
  
  The prime score of an integer x is equal to the number of distinct prime factors of x. For example, the prime score of 300 is 3 since 300 = 2 * 2 * 3 * 5 * 5.
  
  Return the maximum possible score after applying at most k operations.
  
  Since the answer may be large, return it modulo 10^9 + 7.
  
  Example 1:
  Input: nums = [8,3,9,3,8], k = 2
  Output: 81
  Explanation: To get a score of 81, we can apply the following operations:
  - Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2]. The score becomes 1 * 9 = 9.
  - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.
  It can be proven that 81 is the highest score one can obtain.
  
  Example 2:
  Input: nums = [19,12,14,6,10,18], k = 3
  Output: 4788
  Explanation: To get a score of 4788, we can apply the following operations: 
  - Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray. Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.
  - Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray. Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.
  - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index. Hence, we multipy the score by nums[2]. The score becomes 342 * 14 = 4788.
  It can be proven that 4788 is the highest score one can obtain.
*/
public class Solution {
    public int MaximumScore(IList<int> nums, int k) {
        const int mod = (int) 1e9 + 7;

        int n = nums.Count;
        Span<(int, int, int)> arr = stackalloc (int, int, int)[n];
        for (int i = 0; i < n; ++i) {
            arr[i] = (i, PrimeFactors(nums[i]), nums[i]);
        }

        Span<int> left = stackalloc int[n];
        Span<int> right = stackalloc int[n];

        left.Fill(-1);
        right.Fill(n);

        Stack<int> st = [];
        foreach (var (i, f, _) in arr) {
            while (st.TryPeek(out int p) && arr[p].Item2 < f) {
                st.Pop();
            }
            if (st.Count > 0) {
                left[i] = st.Peek();
            }
            st.Push(i);
        }

        st.Clear();

        for (int i = n - 1; i >= 0; --i) {
            int f = arr[i].Item2;
            while (st.TryPeek(out int p) && arr[p].Item2 <= f) {
                st.Pop();
            }
            if (st.Count > 0) {
                right[i] = st.Peek();
            }
            st.Push(i);
        }

        arr.Sort((a, b) => b.Item3 - a.Item3);

        long result = 1;
        foreach (var (i, _, x) in arr) {
            int l = left[i];
            int r = right[i];

            long count = (long) (i - l) * (r - i);
            if (count <= k) {
                result = result * QRow(x, count) % mod;
                k -= (int) count;
            } else {
                result = result * QRow(x, k) % mod;
                break;
            }
        }
        return (int) result;

        static int PrimeFactors(int n) {
            int i = 2;
            HashSet<int> set = [];
            while (i <= n / i) {
                while (n % i == 0) {
                    set.Add(i);
                    n /= i;
                }
                ++i;
            } 
            if (n > 1) set.Add(n);
            return set.Count;
        }

        int QRow(long a, long n) {
            long result = 1;
            for (; n > 0; n >>= 1) {
                if ((n & 1) == 1) {
                    result = result * a % mod;
                }
                a = a * a % mod;
            }
            return (int) result;
        }
    }
}
