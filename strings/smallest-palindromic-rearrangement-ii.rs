/*
  3518. Smallest Palindromic Rearrangement II
  
  You are given a string s and an integer k.
  
  Return the k-th palindromic of s. If there are fewer than k distinct palindromic permutations, return an empty string.
  
  Note: Different rearrangements that yield the same palindromic string are considered identical and are counted once.
  
  Example 1:
  Input: s = "abba", k = 2
  Output: "baab"
  Explanation:
      The two distinct palindromic rearrangements of "abba" are "abba" and "baab".
      Lexicographically, "abba" comes before "baab". Since k = 2, the output is "baab".
  
  Example 2:
  Input: s = "aa", k = 2
  Output: ""
  Explanation:
      There is only one palindromic rearrangement: "aa".
      The output is an empty string since k = 2 exceeds the number of possible rearrangements.
  
  Example 3:
  Input: s = "bacab", k = 1
  Output: "abcba"
  Explanation:
      The two distinct palindromic rearrangements of "bacab" are "abcba" and "bacab".
      Lexicographically, "abcba" comes before "bacab". Since k = 1, the output is "abcba".
*/
impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut res = s.into_bytes();
        let mut rank = k as u64;

        let mut freq = [0usize; 26];
        for &byte in &res {
            freq[(byte - b'a') as usize] += 1;
        }

        let mut middle = None;
        for i in 0..26 {
            if freq[i] & 1 != 0 {
                middle = Some(b'a' + i as u8);
            }
            freq[i] /= 2;
        }

        if count_permutations(&freq, rank) < rank {
            return String::new();
        }

        let n = res.len();
        let half_length = n / 2;

        for position in 0..half_length {
            let t = (half_length - position) as u64;
            
            let cap = rank.saturating_mul(t);
            let base_total = count_permutations(&freq, cap);
            let saturated = base_total >= cap;

            for i in 0..26 {
                if freq[i] == 0 {
                    continue;
                }

                let block = if saturated {
                    cap 
                } else {
                    base_total * freq[i] as u64 / t
                };

                if block >= rank {
                    freq[i] -= 1;
                    res[position] = b'a' + i as u8;
                    break;
                }

                rank -= block;
            }
        }

        if let Some(byte) = middle {
            res[half_length] = byte;
        }

        for i in 0..half_length {
            res[n - 1 - i] = res[i];
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}

fn count_permutations(counts: &[usize; 26], limit: u64) -> u64 {
    let mut ways = 1u64;
    let mut placed = 0usize;

    for &count in counts {
        if count == 0 {
            continue;
        }
        let combinations = binomial_capped(placed + count, count, limit);
        ways = ways.saturating_mul(combinations).min(limit);
        placed += count;
        if ways >= limit {
            return limit;
        }
    }
    ways
}

fn binomial_capped(n: usize, r: usize, limit: u64) -> u64 {
    let r = r.min(n - r);
    let mut value = 1u64;
    for i in 1..=r {
        value = value * (n - r + i) as u64 / i as u64;
        if value >= limit {
            return limit;
        }
    }
    value
}
