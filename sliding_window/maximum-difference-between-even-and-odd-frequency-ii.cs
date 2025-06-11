/*
  3445. Maximum Difference Between Even and Odd Frequency II
  
  You are given a string s and an integer k. Your task is to find the maximum difference between the frequency of two characters, freq[a] - freq[b], in a substring subs of s, such that:
    subs has a size of at least k.
    Character a has an odd frequency in subs.
    Character b has an even frequency in subs.
  
  Return the maximum difference.
  
  Note that subs can contain more than 2 distinct characters.
  
  Example 1:
  Input: s = "12233", k = 4
  Output: -1
  Explanation:
  For the substring "12233", the frequency of '1' is 1 and the frequency of '3' is 2. The difference is 1 - 2 = -1.
  
  Example 2:
  Input: s = "1122211", k = 3
  Output: 1
  Explanation:
  For the substring "11222", the frequency of '2' is 3 and the frequency of '1' is 2. The difference is 3 - 2 = 1.
  
  Example 3:
  Input: s = "110", k = 3
  Output: -1
*/
public class Solution {
    public int MaxDifference(string s, int k) {
        int n = s.Length;
        int max = int.MaxValue / 2;
        int result = -max;
        for (int a = 0; a < 5; ++a) {
            for (int b = 0; b < 5; ++b) {
                if (a == b) continue;
                int currA = 0, currB = 0;
                int prevA = 0, prevB = 0;
                int[,] temp = {
                    { max, max },
                    { max, max }
                };
                for (int l = -1, r = 0; r < n; ++r) {
                    currA += s[r] == ('0' + a) ? 1 : 0;
                    currB += s[r] == ('0' + b) ? 1 : 0;
                    while (r - l >= k && currB - prevB >= 2) {
                        temp[prevA & 1, prevB & 1] = Math.Min(prevA - prevB, temp[prevA & 1, prevB & 1]);
                        ++l;
                        prevA += s[l] == ('0' + a) ? 1 : 0;
                        prevB += s[l] == ('0' + b) ? 1 : 0;
                    }
                    result = Math.Max(result, currA - currB - temp[(currA & 1) ^ 1, currB & 1]);
                }
            }
        }
        return result;
    }
}
