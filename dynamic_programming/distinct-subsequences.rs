/*
  115. Distinct Subsequences
  
  Given two strings s and t, return the number of distinct subsequences of s which equals t.
  
  The test cases are generated so that the answer fits on a 32-bit signed integer.
  
  Example 1:
  Input: s = "rabbbit", t = "rabbit"
  Output: 3
  Explanation:
  As shown below, there are 3 ways you can generate "rabbit" from s.
  rabbbit
  rabbbit
  rabbbit
  
  Example 2:
  Input: s = "babgbag", t = "bag"
  Output: 5
  Explanation:
  As shown below, there are 5 ways you can generate "bag" from s.
  babgbag
  babgbag
  babgbag
  babgbag
  babgbag
*/
const CAP: u64 = i32::MAX as u64;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let source = s.as_bytes();
        let target = t.as_bytes();

        if target.len() > source.len() {
            return 0;
        }

        let mut dp = vec![0u64; target.len() + 1];
        dp[0] = 1;

        for (source_index, &source_char) in source.iter().enumerate() {
            let upper = target.len().min(source_index + 1);

            for length in (1..=upper).rev() {
                if source_char == target[length - 1] {
                    dp[length] = (dp[length] + dp[length - 1]).min(CAP);
                }
            }
        }

        dp[target.len()] as i32
    }
}
