/*
  Given an integer array nums and an integer k, return the length of the shortest non-empty subarray of nums with a sum of at least k. If there is no such subarray, return -1.
  
  A subarray is a contiguous part of an array.
  
  Example 1:
  Input: nums = [1], k = 1
  Output: 1
  
  Example 2:
  Input: nums = [1,2], k = 4
  Output: -1

  Example 3:
  Input: nums = [2,-1,2], k = 3
  Output: 3
*/
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

public unsafe class Solution {
    private ref struct Deque(int* buffer) {
        private int start = 0;
        private int end = 0;

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int Prune(long target, long* prefix, int idx) {
            var (curr, limit, min) = (prefix[idx], prefix[idx] - target, int.MaxValue);
            for (; start < end && prefix[buffer[start]] <= limit; ++start)
                min = Math.Min(min, idx - buffer[start]);
            for (; start < end && curr <= prefix[buffer[end - 1]]; --end) ;
            return min;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void AddLast(int n) => buffer[end++] = n;
    }

    private const int BUFFER_LEN = 100_000;
    private static readonly int* mem = (int*)NativeMemory.Alloc(BUFFER_LEN, sizeof(int));

    [SkipLocalsInit]
    public int ShortestSubarray(int[] nums, int k) {
        var (min, sum) = (int.MaxValue, 0L);
        var prefix = stackalloc long[nums.Length + 1];
        prefix[0] = 0;
        for (var i = 1; i <= nums.Length; ++i)
            prefix[i] = sum += nums[i - 1];
        Deque deque = new(mem);
        for (var i = 0; i <= nums.Length; ++i) {
            min = Math.Min(min, deque.Prune(k, prefix, i));
            deque.AddLast(i);
        }
        return min == int.MaxValue ? -1 : min;
    }
}
