/*
  3208. Alternating Groups II
  
  There is a circle of red and blue tiles. You are given an array of integers colors and an integer k. The color of tile i is represented by colors[i]:
    colors[i] == 0 means that tile i is red.
    colors[i] == 1 means that tile i is blue.
  
  An alternating group is every k contiguous tiles in the circle with alternating colors (each tile in the group except the first and last one has a different color from its left and right tiles).
  
  Return the number of alternating groups.
  
  Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.
  
  Example 1:
  Input: colors = [0,1,0,1,0], k = 3
  Output: 3
  
  Example 2:
  Input: colors = [0,1,0,0,1,0,1], k = 6
  Output: 2
  
  Example 3:
  Input: colors = [1,1,0,1], k = 4
  Output: 0
*/
public class Solution {
    public int NumberOfAlternatingGroups(int[] colors, int k) {
        int n = colors.Length;
        int result = 0;
        int count = 0;

        for (int i = 0; i < n * 2; ++i) {
            if (i > 0 && colors[i % n] == colors[(i - 1) % n]) count = 1;
            else ++count;

            result += i >= n && count >= k ? 1 : 0; 
        }

        return result;
    }
}
