/*
  1079. Letter Tile Possibilities
  
  You have n  tiles, where each tile has one letter tiles[i] printed on it.
  
  Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.
  
  Example 1:
  Input: tiles = "AAB"
  Output: 8
  Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
  
  Example 2:
  Input: tiles = "AAABBC"
  Output: 188
  
  Example 3:
  Input: tiles = "V"
  Output: 1
*/
public class Solution {
    public int NumTilePossibilities(string tiles) {
        Span<int> count = stackalloc int[26];
        foreach (char ch in tiles) {
            ++count[ch - 'A'];
        }

        return Dfs(ref count);

        int Dfs(ref Span<int> count) {
            int result = 0;
            for (int i = 0; i < 26; ++i) {
                if (count[i] == 0) continue;

                ++result;
                --count[i];
                result += Dfs(ref count);
                ++count[i];
            }
            return result;
        }
    }
}
