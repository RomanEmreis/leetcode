/*
  2182. Construct String With Repeat Limit
  
  You are given a string s and an integer repeatLimit. Construct a new string repeatLimitedString using the characters of s such that no letter appears more than repeatLimit times in a row. You do not have to use all characters from s.
  Return the lexicographically largest repeatLimitedString possible.
  A string a is lexicographically larger than a string b if in the first position where a and b differ, string a has a letter that appears later in the alphabet than the corresponding letter in b. If the first min(a.length, b.length) characters do not differ,
  then the longer string is the lexicographically larger one.
  
  Example 1:
  Input: s = "cczazcc", repeatLimit = 3
  Output: "zzcccac"
  Explanation: We use all of the characters from s to construct the repeatLimitedString "zzcccac".
  The letter 'a' appears at most 1 time in a row.
  The letter 'c' appears at most 3 times in a row.
  The letter 'z' appears at most 2 times in a row.
  Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
  The string is the lexicographically largest repeatLimitedString possible so we return "zzcccac".
  Note that the string "zzcccca" is lexicographically larger but the letter 'c' appears more than 3 times in a row, so it is not a valid repeatLimitedString.
  
  Example 2:
  Input: s = "aababab", repeatLimit = 2
  Output: "bbabaa"
  Explanation: We use only some of the characters from s to construct the repeatLimitedString "bbabaa". 
  The letter 'a' appears at most 2 times in a row.
  The letter 'b' appears at most 2 times in a row.
  Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
  The string is the lexicographically largest repeatLimitedString possible so we return "bbabaa".
  Note that the string "bbabaaa" is lexicographically larger but the letter 'a' appears more than 2 times in a row, so it is not a valid repeatLimitedString.
*/
public class Solution {
    private static readonly IComparer<char> comparer = Comparer<char>.Create((x,y) => y.CompareTo(x));

    public string RepeatLimitedString(string s, int repeatLimit) {
        Span<int> a = stackalloc int[26];
        foreach (var ch in s) {
            ++a[ch - 'a'];
        }

        StringBuilder sb = new();
        for (int i = 25, j = 24; i >= 0; --i) {
            j = Math.Min(j, i - 1);
            while (true) {
                for (int k = Math.Min(a[i], repeatLimit); k > 0; --k) {
                    sb.Append((char)('a' + i));
                    --a[i];
                }
                if (a[i] == 0) break;

                while (j >= 0 && a[j] == 0) --j;
                if (j < 0) break;

                sb.Append((char)('a' + j));
                --a[j];
            }
        }
        return sb.ToString();
    }
}
