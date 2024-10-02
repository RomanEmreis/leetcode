/*
  Given an array of integers arr, replace each element with its rank.
  
  The rank represents how large the element is. The rank has the following rules:
  Rank is an integer starting from 1.
  The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
  Rank should be as small as possible.
   
  Example 1:
    Input: arr = [40,10,20,30]
    Output: [4,1,2,3]
    Explanation: 40 is the largest element. 10 is the smallest. 20 is the second smallest. 30 is the third smallest.
  
  Example 2:
    Input: arr = [100,100,100]
    Output: [1,1,1]
    Explanation: Same elements share the same rank.
*/
public class Solution {
    public int[] ArrayRankTransform(int[] arr) {
        int[] sorted = [..arr];        
        Array.Sort(sorted);

        Dictionary<int, int> map = [];
        for (int i = 0, rank = 1; i < sorted.Length; ++i) {
            if (!map.ContainsKey(sorted[i])) {
                map[sorted[i]] = rank++;
            }
        }

        for (int i = 0; i < arr.Length; ++i) {
            arr[i] = map[arr[i]];
        }

        return arr;
    }
}
