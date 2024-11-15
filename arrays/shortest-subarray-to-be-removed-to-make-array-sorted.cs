/*
  Given an integer array arr, remove a subarray (can be empty) from arr such that the remaining elements in arr are non-decreasing.
  Return the length of the shortest subarray to remove.
  
  A subarray is a contiguous subsequence of the array.
  
  Example 1:
    Input: arr = [1,2,3,10,4,2,3,5]
    Output: 3
    Explanation: The shortest subarray we can remove is [10,4,2] of length 3. The remaining elements after that will be [1,2,3,3,5] which are sorted.
    Another correct solution is to remove the subarray [3,10,4].
*/
public class Solution {
    public int FindLengthOfShortestSubarray(int[] arr) {
        int n = arr.Length;
        int s = 0, e = n - 1;

        while (s < e && arr[s] <= arr[s + 1]) ++s;
        if (s == e) return 0;

        while (e >= s && arr[e] >= arr[e - 1]) --e;

        int result = Math.Min(e, n - 1 - s);
        int i = 0, j = e;
        while (i <= s && j < n) {
            if (arr[i] <= arr[j]) {
                result = Math.Min(result, j - i - 1);
                ++i;
            } else {
                ++j;
            }
        }

        return result;
    }
}
