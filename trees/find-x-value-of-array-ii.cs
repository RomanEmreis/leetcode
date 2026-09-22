/*
  3525. Find X Value of Array II
  
  You are given an array of positive integers nums and a positive integer k. You are also given a 2D array queries, where queries[i] = [indexi, valuei, starti, xi].
  You are allowed to perform an operation once on nums, where you can remove any suffix from nums such that nums remains non-empty.
  
  The x-value of nums for a given x is defined as the number of ways to perform this operation so that the product of the remaining elements leaves a remainder of x modulo k.
  
  For each query in queries you need to determine the x-value of nums for xi after performing the following actions:
      Update nums[indexi] to valuei. Only this step persists for the rest of the queries.
      Remove the prefix nums[0..(starti - 1)] (where nums[0..(-1)] will be used to represent the empty prefix).
  
  Return an array result of size queries.length where result[i] is the answer for the ith query.
  
  A prefix of an array is a subarray that starts from the beginning of the array and extends to any point within it.
  A suffix of an array is a subarray that starts at any point within the array and extends to the end of the array.
  
  Note that the prefix and suffix to be chosen for the operation can be empty.
  Note that x-value has a different definition in this version.
  
  Example 1:
  Input: nums = [1,2,3,4,5], k = 3, queries = [[2,2,0,2],[3,3,3,0],[0,1,0,1]]
  Output: [2,2,2]
  Explanation:
      For query 0, nums becomes [1, 2, 2, 4, 5], and the empty prefix must be removed. The possible operations are:
          Remove the suffix [2, 4, 5]. nums becomes [1, 2].
          Remove the empty suffix. nums becomes [1, 2, 2, 4, 5] with a product 80, which gives remainder 2 when divided by 3.
      For query 1, nums becomes [1, 2, 2, 3, 5], and the prefix [1, 2, 2] must be removed. The possible operations are:
          Remove the empty suffix. nums becomes [3, 5].
          Remove the suffix [5]. nums becomes [3].
      For query 2, nums becomes [1, 2, 2, 3, 5], and the empty prefix must be removed. The possible operations are:
          Remove the suffix [2, 2, 3, 5]. nums becomes [1].
          Remove the suffix [3, 5]. nums becomes [1, 2, 2].
  
  Example 2:
  Input: nums = [1,2,4,8,16,32], k = 4, queries = [[0,2,0,2],[0,2,0,1]]
  Output: [1,0]
  Explanation:
      For query 0, nums becomes [2, 2, 4, 8, 16, 32]. The only possible operation is:
          Remove the suffix [2, 4, 8, 16, 32].
      For query 1, nums becomes [2, 2, 4, 8, 16, 32]. There is no possible way to perform the operation.
  
  Example 3:
  Input: nums = [1,1,2,1,1], k = 2, queries = [[2,1,0,1]]
  Output: [5]
*/
using System;

public class Solution {
    public int[] ResultArray(int[] nums, int k, int[][] queries) {
        int n = nums.Length;
        int size = 1;
        while (size < n) size <<= 1;

        int[] products = new int[2 * size];
        int[] prefixes = new int[2 * size * k];

        Array.Fill(products, 1 % k);

        for (int i = 0; i < n; i++) {
            int remainder = nums[i] % k;
            products[size + i] = remainder;
            prefixes[(size + i) * k + remainder] = 1;
        }

        for (int node = size - 1; node > 0; node--)
            MergeInto(node, products, prefixes, k);

        int[][] veltrunigo = queries;
        int[] answer = new int[veltrunigo.Length];

        int[] leftCounts = new int[k];
        int[] rightCounts = new int[k];
        int[] scratch = new int[k];

        for (int qi = 0; qi < veltrunigo.Length; qi++) {
            int[] query = veltrunigo[qi];
            int position = size + query[0];
            int remainder = query[1] % k;

            int leafOffset = position * k;
            Array.Clear(prefixes, leafOffset, k);
            products[position] = remainder;
            prefixes[leafOffset + remainder] = 1;

            while (position > 1) {
                position /= 2;
                MergeInto(position, products, prefixes, k);
            }

            Array.Clear(leftCounts, 0, k);
            Array.Clear(rightCounts, 0, k);
            int leftProduct = 1 % k;
            int rightProduct = 1 % k;

            int left = size + query[2];
            int right = size + n;

            while (left < right) {
                if ((left & 1) != 0) {
                    int offset = left * k;
                    for (int r = 0; r < k; r++)
                        leftCounts[leftProduct * r % k] +=
                            prefixes[offset + r];

                    leftProduct = leftProduct * products[left] % k;
                    left++;
                }

                if ((right & 1) != 0) {
                    right--;
                    int offset = right * k;
                    int nodeProduct = products[right];

                    for (int r = 0; r < k; r++)
                        scratch[r] = prefixes[offset + r];

                    for (int r = 0; r < k; r++)
                        scratch[nodeProduct * r % k] += rightCounts[r];

                    (rightCounts, scratch) = (scratch, rightCounts);
                    rightProduct = nodeProduct * rightProduct % k;
                }

                left /= 2;
                right /= 2;
            }

            int x = query[3];
            int count = leftCounts[x];

            for (int r = 0; r < k; r++)
                if (leftProduct * r % k == x)
                    count += rightCounts[r];

            answer[qi] = count;
        }

        return answer;
    }

    private static void MergeInto(int node, int[] products, int[] prefixes, int k) {
        int left = 2 * node;
        int right = left + 1;
        int offset = node * k;
        int leftOffset = left * k;
        int rightOffset = right * k;
        int leftProduct = products[left];

        for (int r = 0; r < k; r++)
            prefixes[offset + r] = prefixes[leftOffset + r];

        for (int r = 0; r < k; r++)
            prefixes[offset + leftProduct * r % k] +=
                prefixes[rightOffset + r];

        products[node] = leftProduct * products[right] % k;
    }
}
