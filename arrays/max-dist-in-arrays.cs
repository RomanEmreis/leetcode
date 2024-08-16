/*
  You are given m arrays, where each array is sorted in ascending order.
  You can pick up two integers from two different arrays (each array picks one) and calculate the distance. We define the distance between two integers a and b to be their absolute difference |a - b|.
  
  Return the maximum distance.

  Example 1:
    Input: arrays = [[1,2,3],[4,5],[1,2,3]]
    Output: 4
    Explanation: One way to reach the maximum distance 4 is to pick 1 in the first or third array and pick 5 in the second array.

  Example 2:
    Input: arrays = [[1],[1]]
    Output: 0
*/
public class Solution {
    public int MaxDistance(IList<IList<int>> arrays) {
        int mn = arrays[0][0];
        int mx = arrays[0][^1];
        int result = 0;
        for (int i = 1; i < arrays.Count; ++i) {
            int currMin = arrays[i][0];
            int currMax = arrays[i][^1];

            result = Math.Max(result, Math.Max(Math.Abs(mx - currMin), Math.Abs(currMax - mn)));

            mn = Math.Min(mn, currMin);
            mx = Math.Max(mx, currMax);
        }
        return result;
    }
}
