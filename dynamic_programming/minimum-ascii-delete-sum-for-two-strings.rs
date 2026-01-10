/*
  712. Minimum ASCII Delete Sum for Two Strings
  
  Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.
  
  Example 1:
  Input: s1 = "sea", s2 = "eat"
  Output: 231
  Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
  Deleting "t" from "eat" adds 116 to the sum.
  At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
  
  Example 2:
  Input: s1 = "delete", s2 = "leet"
  Output: 403
  Explanation: Deleting "dee" from "delete" to turn the string into "let",
  adds 100[d] + 101[e] + 101[e] to the sum.
  Deleting "e" from "leet" adds 101[e] to the sum.
  At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
  If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
*/
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let m = s1.len();
        let n = s2.len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        let ch1 = s1.as_bytes();
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] + ch1[i - 1] as i32;
        }

        let ch2 = s2.as_bytes();
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] + ch2[j - 1] as i32;
        }

        for i in 1..=m {
            for j in 1..=n {
                if ch1[i - 1] == ch2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = std::cmp::min(
                        dp[i - 1][j] + ch1[i - 1] as i32,
                        dp[i][j - 1] + ch2[j - 1] as i32
                    );
                }
            }
        }

        dp[m][n]   
    }
}
