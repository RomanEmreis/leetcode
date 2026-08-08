/*
  3302. Find the Lexicographically Smallest Valid Sequence
  
  You are given two strings word1 and word2.
  
  A string x is called almost equal to y if you can change at most one character in x to make it identical to y.
  A sequence of indices seq is called valid if:
      The indices are sorted in ascending order.
      Concatenating the characters at these indices in word1 in the same order results in a string that is almost equal to word2.
  
  Return an array of size word2.length representing the valid sequence of indices. If no such sequence of indices exists, return an empty array.
  
  Note that the answer must represent the lexicographically smallest array, not the corresponding string formed by those indices.
  
  Example 1:
  Input: word1 = "vbcca", word2 = "abc"
  Output: [0,1,2]
  Explanation:
  The lexicographically smallest valid sequence of indices is [0, 1, 2]:
      Change word1[0] to 'a'.
      word1[1] is already 'b'.
      word1[2] is already 'c'.
  
  Example 2:
  Input: word1 = "bacdc", word2 = "abc"
  Output: [1,2,4]
  Explanation:
  The lexicographically smallest valid sequence of indices is [1, 2, 4]:
      word1[1] is already 'a'.
      Change word1[2] to 'b'.
      word1[4] is already 'c'.
  
  Example 3:
  Input: word1 = "aaaaaa", word2 = "aaabc"
  Output: []
  Explanation:
  There is no valid sequence of indices.
  
  Example 4:
  Input: word1 = "abc", word2 = "ab"
  Output: [0,1]
*/
impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let s = word1.as_bytes();
        let t = word2.as_bytes();
        let (n, m) = (s.len(), t.len());

        let mut result = vec![-1i32; m];
        let mut i = n;
        for j in (0..m).rev() {
            match s[..i].iter().rposition(|&b| b == t[j]) {
                Some(pos) => {
                    result[j] = pos as i32;
                    i = pos;
                }
                None => break,
            }
        }

        let mut ti = 0usize;
        let mut mismatch_used = false;
        let mut cur = t[0];
        let mut limit = if m == 1 { i32::MAX } else { result[1] };

        for (si, &c) in s.iter().enumerate() {
            if c == cur || (!mismatch_used && limit > si as i32) {
                mismatch_used |= c != cur;
                result[ti] = si as i32;
                ti += 1;
                if ti == m {
                    return result;
                }
                cur = t[ti];
                limit = if ti + 1 == m { i32::MAX } else { result[ti + 1] };
            }
        }

        Vec::new()
    }
}
