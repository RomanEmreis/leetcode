/*
  A distinct string is a string that is present only once in an array.
  Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".

  Note that the strings are considered in the order in which they appear in the array.

  Example:
    Input: arr = ["d","b","c","b","c","a"], k = 2
    Output: "a"
    Explanation:
    The only distinct strings in arr are "d" and "a".
    "d" appears 1st, so it is the 1st distinct string.
    "a" appears 2nd, so it is the 2nd distinct string.
    Since k == 2, "a" is returned. 
*/
public class Solution {
    public string KthDistinct(string[] arr, int k) {
        Dictionary<string, int> dict = [];
        foreach (string s in arr) {
            if (dict.ContainsKey(s)) ++dict[s];
            else dict[s] = 1;
        }

        foreach (var (s, n) in dict) {
            if (n == 1 && k-- == 1) return s;
        }

        return string.Empty;
    }
}
