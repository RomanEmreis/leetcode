/*
  1081. Smallest Subsequence of Distinct Characters
  
  Given a string s, return the of s that contains all the distinct characters of s exactly once.
  
  Example 1:
  Input: s = "bcabc"
  Output: "abc"
  
  Example 2:
  Input: s = "cbacdcbc"
  Output: "acdb"  
*/
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bytes = s.into_bytes();
        let mut pos = [0usize; 26];

        for (i, &b) in bytes.iter().enumerate() {
            pos[(b - b'a') as usize] = i;
        }

        let mut seen = [false; 26];
        let mut res = Vec::with_capacity(26);

        for (i, b) in bytes.into_iter().enumerate() {
            let ch = (b - b'a') as usize;
            if seen[ch] {
                continue;
            }

            while let Some(&t) = res.last() {
                let top = (t - b'a') as usize;
                if t <= b || pos[top] <= i {
                    break;
                }

                _ = res.pop();
                seen[top] = false;
            }

            seen[ch] = true;
            res.push(b);
        }

        String::from_utf8(res).unwrap()
    }
}
