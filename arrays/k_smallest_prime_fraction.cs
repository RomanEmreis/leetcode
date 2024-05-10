/*
  You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
  For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
  Return the kth smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].

  Example 1:
    Input: arr = [1,2,3,5], k = 3
    Output: [2,5]
    Explanation: The fractions to be considered in sorted order are:
    1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
    The third fraction is 2/5.
*/
public class Solution {
    public int[] KthSmallestPrimeFraction(int[] arr, int k) {
        int n = arr.Length;
        if (n == 2) return arr;

        var (i, j) = Enumerable.Range(0, n)
            .SelectMany(i => Enumerable.Range(i + 1, n - 1 - i).Select(j => (arr[i], arr[j])))
            .OrderBy(x => (float) x.Item1 / x.Item2)
            .Skip(k - 1)
            .First();

        return [i, j];
    }
}
