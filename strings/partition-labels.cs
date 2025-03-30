/*
  763. Partition Labels
  
  You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part. For example, the string "ababcc" can be partitioned into ["abab", "cc"], but partitions such as ["aba", "bcc"] or ["ab", "ab", "cc"] are invalid.
  
  Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
  
  Return a list of integers representing the size of these parts.
  
  Example 1:
  Input: s = "ababcbacadefegdehijhklij"
  Output: [9,7,8]
  Explanation:
  The partition is "ababcbaca", "defegde", "hijhklij".
  This is a partition so that each letter appears in at most one part.
  A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
  
  Example 2:
  Input: s = "eccbbbbdec"
  Output: [10]
*/
public class Solution {
    public IList<int> PartitionLabels(string s) {
        Span<int> chars = stackalloc int[26];
        for (int i = 0; i < s.Length; ++i) {
            chars[s[i] - 'a'] = i;
        }

        List<int> result = [];
        for (int i = 0, j = 0, max = 0; i < s.Length; ++i) {
            if (max < chars[s[i] - 'a']) {
                max = chars[s[i] - 'a'];
            }
            if (max == i) {
                result.Add(i - j + 1);
                j = i + 1;
            }
        }
        return result;
    }
}
