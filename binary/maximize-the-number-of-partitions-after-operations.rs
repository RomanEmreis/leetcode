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
impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let n = s.len();
        let b = s.as_bytes();

        let mut left = vec![(0, 0, 0); n];
        let mut right = vec![(0, 0, 0); n];

        let mut num = 0;
        let mut mask = 0;
        let mut cnt = 0;

        for i in 0..n - 1 {
            let bit = 1 << (b[i] - b'a');
            if (mask & bit) == 0 {
                cnt += 1;
                if cnt <= k {
                    mask |= bit;
                } else {
                    num += 1;
                    mask = bit;
                    cnt = 1;
                }
            }
            left[i + 1] = (num, mask, cnt);
        }

        num = 0;
        mask = 0;
        cnt = 0;

        for i in (1..n).rev() {
            let bit = 1 << (b[i] - b'a');
            if (mask & bit) == 0 {
                cnt += 1;
                if cnt <= k {
                    mask |= bit;
                } else {
                    num += 1;
                    mask = bit;
                    cnt = 1;
                }
            }
            right[i - 1] = (num, mask, cnt);
        }

        let mut res = 0;

        for i in 0..n {
            let (lnum, lmask, lcnt) = left[i];
            let (rnum, rmask, rcnt) = right[i];

            let mut seg = lnum + rnum + 2;
            let mut tmask = lmask | rmask;
            let mut tcnt = 0;

            while tmask != 0 {
                tmask = tmask & (tmask - 1);
                tcnt += 1;
            }

            if lcnt == k && rcnt == k && tcnt < 26 {
                seg += 1;
            } else if std::cmp::min(tcnt + 1, 26) <= k {
                seg -= 1;
            }

            res = res.max(seg);
        }

        res
    }
}
