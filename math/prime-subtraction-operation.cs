/*
  You are given a 0-indexed integer array nums of length n.
  
  You can perform the following operation as many times as you want:
  
  Pick an index i that you haven’t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].
  Return true if you can make nums a strictly increasing array using the above operation and false otherwise.
  
  A strictly increasing array is an array whose each element is strictly greater than its preceding element.
  
  Example 1:
    Input: nums = [4,9,6,10]
    Output: true
    Explanation: In the first operation: Pick i = 0 and p = 3, and then subtract 3 from nums[0], so that nums becomes [1,9,6,10].
    In the second operation: i = 1, p = 7, subtract 7 from nums[1], so nums becomes equal to [1,2,6,10].
    After the second operation, nums is sorted in strictly increasing order, so the answer is true.
*/
public class Solution {
    public bool PrimeSubOperation(int[] nums) {
        List<int> primes = [];
        for (int i = 2; i <= 1000; ++i) {
            bool isPrime = true;
            for (int j = 0; j < primes.Count; ++j) {
                if (i % primes[j] == 0) {
                    isPrime = false;
                    break;
                }
            }
            if (isPrime) primes.Add(i);
        }

        int n = nums.Length;
        int p = primes.Count;

        for (int i = n - 2; i >= 0; --i) {
            if (nums[i] < nums[i + 1]) {
                continue;
            }

            int j = Search(nums[i] - nums[i + 1]);
            if (j == p || primes[j] >= nums[i]) return false;

            nums[i] -= primes[j];
        }

        return true;

        int Search(int x) {
            int l = 0;
            int r = p;

            while (l < r) {
                int m = (r + l) >> 1;
                if (primes[m] > x) r = m;
                else l = m + 1;
            }

            return l;
        }
    }
}
