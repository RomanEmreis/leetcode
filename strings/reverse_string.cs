/*
  Write a function that reverses a string. The input string is given as an array of characters s.
  You must do this by modifying the input array in-place with O(1) extra memory.
*/
public class Solution {
    public void ReverseString(char[] s) {
        for (int i = 0, j = s.Length - 1; i < j; ++i, --j) {
            (s[i], s[j]) = (s[j], s[i]);
        }
    }
}
