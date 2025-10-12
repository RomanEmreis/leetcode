/*
  3539. Find Sum of Array Product of Magical Sequences
  
  You are given two integers, m and k, and an integer array nums.
  A sequence of integers seq is called magical if:
      seq has a size of m.
      0 <= seq[i] < nums.length
      The binary representation of 2seq[0] + 2seq[1] + ... + 2seq[m - 1] has k set bits.
  
  The array product of this sequence is defined as prod(seq) = (nums[seq[0]] * nums[seq[1]] * ... * nums[seq[m - 1]]).
  
  Return the sum of the array products for all valid magical sequences.
  
  Since the answer may be large, return it modulo 109 + 7.
  A set bit refers to a bit in the binary representation of a number that has a value of 1.
  
  Example 1:
  Input: m = 5, k = 5, nums = [1,10,100,10000,1000000]
  Output: 991600007
  Explanation:
  All permutations of [0, 1, 2, 3, 4] are magical sequences, each with an array product of 1013.
  
  Example 2:
  Input: m = 2, k = 2, nums = [5,4,3,2,1]
  Output: 170
  Explanation:
  The magical sequences are [0, 1], [0, 2], [0, 3], [0, 4], [1, 0], [1, 2], [1, 3], [1, 4], [2, 0], [2, 1], [2, 3], [2, 4], [3, 0], [3, 1], [3, 2], [3, 4], [4, 0], [4, 1], [4, 2], and [4, 3].
  
  Example 3:
  Input: m = 1, k = 1, nums = [28]
  Output: 28
  Explanation:
  The only magical sequence is [0].
*/
using System.Runtime.InteropServices;

public sealed class Solution {
    private const int Mod = 1_000_000_007;

    public int MagicalSum(int m, int k, int[] nums) {
        int[,] comb = GetComb(m, m);
        int[,,,] mem = new int[m + 1, k + 1, nums.Length + 1, m + 1];

        Span<int> span = MemoryMarshal.CreateSpan(ref mem[0, 0, 0, 0], mem.Length);
        span.Fill(-1);

        return Dp(m, k, 0, 0, nums, mem, comb);
    }

    private static int Dp(int m, int k, int i, int carry, int[] nums, int[,,,] mem, int[,] comb) {
        int popCount = BitOperations.PopCount((uint) carry);

        if (m < 0 || k < 0 || m + popCount < k)
            return 0;
        if (m == 0)
            return k == popCount ? 1 : 0;
        if (i == nums.Length)
            return 0;
        if (mem[m, k, i, carry] != -1)
            return mem[m, k, i, carry];

        int res = 0;

        for (int count = 0; count <= m; ++count) {
            long contribution = comb[m, count] * ModPow(nums[i], count) % Mod;
            int newCarry = carry + count;
            res = (int) ((res 
                + (long) Dp(m - count, k - (newCarry % 2), i + 1, newCarry / 2, nums, mem, comb) 
                * contribution)
            % Mod);

        }

        return mem[m, k, i, carry] = res;
    }

    private static int[,] GetComb(int n, int k) {
        int[,] comb = new int[n + 1, k + 1];
        
        for (int i = 0; i <= n; ++i)
            comb[i, 0] = 1;

        for (int i = 1; i <= n; ++i)
            for (int j = 1; j <= k; ++j)
                comb[i, j] = comb[i - 1, j] + comb[i - 1, j - 1];
        
        return comb;
    }

    private static long ModPow(long x, long n) {
        if (n == 0)
            return 1;
        if (n % 2 == 1)
            return x * ModPow(x % Mod, n - 1) % Mod;
        return ModPow(x * x % Mod, n / 2) % Mod;
    }
}
