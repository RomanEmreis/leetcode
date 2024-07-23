/*
  Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.
  Return the sorted array.

  Example:
    Input: nums = [1,1,2,2,2,3]
    Output: [3,1,1,2,2,2]
    Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
*/
public class Solution {
    public int[] FrequencySort(int[] nums) {
        Dictionary<int, int> map = [];
        foreach (int n in nums) {
            if (map.ContainsKey(n)) ++map[n];
            else map[n] = 1;
        }

        Array.Sort(nums, Comparer<int>.Create(Compare));

        return nums;

        int Compare(int x, int y) {
            int xCount = map[x];
            int yCount = map[y];

            return xCount == yCount
                ? y.CompareTo(x)
                : xCount.CompareTo(yCount); 
        }
    }
}
