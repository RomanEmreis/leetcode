/*
  2381. Shifting Letters II
  
  You are given a string s of lowercase English letters and a 2D integer array shifts where shifts[i] = [starti, endi, directioni]. For every i, shift the characters in s from the index starti to the index endi (inclusive) forward if directioni = 1, or shift the characters backward if directioni = 0.
  Shifting a character forward means replacing it with the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Similarly, shifting a character backward means replacing it with the previous letter in the alphabet (wrapping around so that 'a' becomes 'z').
  
  Return the final string after all such shifts to s are applied.
  
  Example 1:
  Input: s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
  Output: "ace"
  Explanation: Firstly, shift the characters from index 0 to index 1 backward. Now s = "zac".
  Secondly, shift the characters from index 1 to index 2 forward. Now s = "zbd".
  Finally, shift the characters from index 0 to index 2 forward. Now s = "ace".
  
  Example 2:
  Input: s = "dztz", shifts = [[0,0,0],[1,1,1]]
  Output: "catz"
  Explanation: Firstly, shift the characters from index 0 to index 0 backward. Now s = "cztz".
  Finally, shift the characters from index 1 to index 1 forward. Now s = "catz".
*/
public class Solution {
    public string ShiftingLetters(string s, int[][] shifts) {
        Span<int> prefix = stackalloc int[s.Length + 1];
        for (int i = 0; i < shifts.Length; ++i) {
            if (shifts[i][2] == 0) --shifts[i][2];
            prefix[shifts[i][0]] += shifts[i][2];
            prefix[shifts[i][1] + 1] -= shifts[i][2];
        }
        for (int i = 1; i < prefix.Length; ++i) {
            prefix[i] += prefix[i - 1];
        }

        StringBuilder sb = new();
        for (int i = 0; i < s.Length; ++i) {
            int j = (s[i] - 'a' + prefix[i] % 26 + 26) % 26;
            sb.Append((char)(j + 'a'));
        }
        return sb.ToString();
    }
}
