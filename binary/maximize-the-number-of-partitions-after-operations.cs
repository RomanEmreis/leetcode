/*
  3003. Maximize the Number of Partitions After Operations
  
  You are given a string s and an integer k.
  First, you are allowed to change at most one index in s to another lowercase English letter.
  After that, do the following partitioning operation until s is empty:
  
  Choose the longest prefix of s containing at most k distinct characters.
    Delete the prefix from s and increase the number of partitions by one. The remaining characters (if any) in s maintain their initial order.
    Return an integer denoting the maximum number of resulting partitions after the operations by optimally choosing at most one index to change.
  
  Example 1:
  Input: s = "accca", k = 2
  Output: 3
  Explanation:
  The optimal way is to change s[2] to something other than a and c, for example, b. then it becomes "acbca".
  Then we perform the operations:
  The longest prefix containing at most 2 distinct characters is "ac", we remove it and s becomes "bca".
  Now The longest prefix containing at most 2 distinct characters is "bc", so we remove it and s becomes "a".
  Finally, we remove "a" and s becomes empty, so the procedure ends.
  Doing the operations, the string is divided into 3 partitions, so the answer is 3.
  
  Example 2:
  Input: s = "aabaab", k = 3
  Output: 1
  Explanation:
  Initially s contains 2 distinct characters, so whichever character we change, it will contain at most 3 distinct characters,
  so the longest prefix with at most 3 distinct characters would always be all of it, therefore the answer is 1.
  
  Example 3:
  Input: s = "xxyz", k = 1
  Output: 4
  Explanation:
  The optimal way is to change s[0] or s[1] to something other than characters in s, for example, to change s[0] to w.
  Then s becomes "wxyz", which consists of 4 distinct characters, so as k is 1, it will divide into 4 partitions.
*/
public class Solution {
    public int MaxPartitionsAfterOperations(string s, int k) {
        int n = s.Length;

        Span<(int, int, int)> left = stackalloc (int, int, int)[n];
        Span<(int, int, int)> right = stackalloc (int, int, int)[n];

        int num = 0;
        int mask = 0;
        int count = 0;

        for (int i = 0; i < n - 1; ++i) {
            int bit = 1 << (s[i] - 'a');
            if ((mask & bit) == 0) {
                ++count;
                if (count <= k) {
                    mask |= bit;
                } else {
                    ++num;
                    mask = bit;
                    count = 1;
                }
            }
            left[i + 1] = (num, mask, count);
        }

        num = 0;
        mask = 0;
        count = 0;

        for (int i = n - 1; i > 0; --i) {
            int bit = 1 << (s[i] - 'a');
            if ((mask & bit) == 0) {
                ++count;
                if (count <= k) {
                    mask |= bit;
                } else {
                    ++num;
                    mask = bit;
                    count = 1;
                }
            }
            right[i - 1] = (num, mask, count);
        }

        int res = 0;

        for (int i = 0; i < n; ++i) {
            var (lnum, lmask, lcount) = left[i];
            var (rnum, rmask, rcount) = right[i];

            int seg = lnum + rnum + 2;
            int tmask = lmask | rmask;
            int tcount = 0;

            while (tmask != 0) {
                tmask = tmask & (tmask - 1);
                ++tcount;
            }

            if (lcount == k && rcount == k && tcount < 26)
                ++seg;
            else if (Math.Min(tcount + 1, 26) <= k)
                --seg;

            res = Math.Max(res, seg);
        }

        return res;
    }
}
