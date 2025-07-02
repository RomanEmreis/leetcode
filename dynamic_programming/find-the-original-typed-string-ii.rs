/*
  3333. Find the Original Typed String II
  
  Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.
  You are given a string word, which represents the final output displayed on Alice's screen. You are also given a positive integer k.
  
  Return the total number of possible original strings that Alice might have intended to type, if she was trying to type a string of size at least k.
  
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: word = "aabbccdd", k = 7
  Output: 5
  Explanation:
  The possible strings are: "aabbccdd", "aabbccd", "aabbcdd", "aabccdd", and "abbccdd".
  
  Example 2:
  Input: word = "aabbccdd", k = 8
  Output: 1
  Explanation:
  The only possible string is "aabbccdd".
  
  Example 3:
  Input: word = "aaabbb", k = 3
  Output: 8
*/
impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn possible_string_count(word: String, mut k: i32) -> i32 {
        let mut result: i64 = 1;
        let mut curr = 0;
        let n = word.len();
        let bytes = word.as_bytes();
        let mut nums = Vec::with_capacity(n);

        for i in 0..n {
            curr += 1;
            if i == n - 1 || bytes[i] != bytes[i + 1] {
                if curr > 1 {
                    if k > 0 {
                        nums.push(curr - 1);
                    }
                    result = result * (curr as i64) % Self::MOD;
                }
                curr = 0;
                if k > 0 {
                    k -= 1;
                }
            }
        }

        if k < 1 {
            return result as i32;
        }

        let k = k as usize;
        let m = nums.len();
        let mut dp = vec![vec![0i64; k]; m + 1];
        dp[0][0] = 1;

        for i in 1..=m {
            let x = nums[i - 1];
            let mut s = vec![0i64; k + 1];
            for j in 0..k {
                s[j + 1] = (s[j] + dp[i - 1][j]) % Self::MOD;
            }
            for j in 0..k {
                let l = j.saturating_sub(x);
                dp[i][j] = (s[j + 1] - s[l] + Self::MOD) % Self::MOD;
            }
        }

        let sum: i64 = (0..k).fold(0, |acc, j| (acc + dp[m][j]) % Self::MOD);
        ((result - sum + Self::MOD) % Self::MOD) as i32
    }
}
