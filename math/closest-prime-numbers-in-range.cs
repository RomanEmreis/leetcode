/*
  2523. Closest Prime Numbers in Range
  
  Given two positive integers left and right, find the two integers num1 and num2 such that:
    left <= num1 < num2 <= right .
    Both num1 and num2 are prime numbers.
    num2 - num1 is the minimum amongst all other pairs satisfying the above conditions.
  
  Return the positive integer array ans = [num1, num2]. If there are multiple pairs satisfying these conditions, return the one with the smallest num1 value. If no such numbers exist, return [-1, -1].
  
  Example 1:
  Input: left = 10, right = 19
  Output: [11,13]
  Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
  The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
  Since 11 is smaller than 17, we return the first pair.
  Example 2:
  
  Input: left = 4, right = 6
  Output: [-1,-1]
  Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
*/
public class Solution {
    public int[] ClosestPrimes(int left, int right) {
        Span<bool> primes = stackalloc bool[right + 1];
        primes.Fill(true);
        primes[0] = false;

        for (int p = 2; p * p <= right; ++p) {
            if (primes[p]) {
                for (int i = p * p; i <= right; i += p) primes[i] = false;
            }
        }

        List<int> primesInRange = [];
        for (int i = Math.Max(2, left); i <= right; ++i) {
            if (primes[i]) primesInRange.Add(i);
        }

        if (primesInRange.Count < 2) return [-1, -1];

        int[] result = new int[2];
        int min = int.MaxValue;
        for (int i = 1; i < primesInRange.Count; ++i) {
            int diff = primesInRange[i] - primesInRange[i - 1];
            if (diff < min) {
                min = diff;
                result[0] = primesInRange[i - 1];
                result[1] = primesInRange[i];
            }
        }

        return result;
    }
}
