/*
  Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
  
  Example 1:
    Input: s = "eleetminicoworoep"
    Output: 13
    Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
*/
using System.Collections.Frozen;
public class Solution {
    private static readonly FrozenDictionary<char, int> vowelIndex = new Dictionary<char, int> {
        ['a'] = 0,
        ['e'] = 1,
        ['i'] = 2,
        ['o'] = 3,
        ['u'] = 4,
    }.ToFrozenDictionary();
    
    public int FindTheLongestSubstring(string s) {
        Dictionary<int, int> firstOccurence = new() {
            [0] = -1
        };

        int result = 0, mask = 0;
        for (int i = 0; i < s.Length; ++i) {
            char ch = s[i];
            if (vowelIndex.TryGetValue(ch, out int bit)) mask ^= 1 << bit;

            if (firstOccurence.TryGetValue(mask, out int val)) result = Math.Max(result, i - val);
            else firstOccurence[mask] = i;
        }
        return result;
    }
}
