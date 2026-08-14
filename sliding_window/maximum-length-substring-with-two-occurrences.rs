/*
  3090. Maximum Length Substring With Two Occurrences
  
  Given a string s, return the maximum length of a  such that it contains at most two occurrences of each character.
  
  Example 1:
  Input: s = "bcbbbcba"
  Output: 4
  Explanation:
  The following substring has a length of 4 and contains at most two occurrences of each character: "bcbbbcba".
  
  Example 2:
  Input: s = "aaaa"
  Output: 2
  Explanation:
  The following substring has a length of 2 and contains at most two occurrences of each character: "aaaa".
*/
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.into_bytes();
        let mut positions = [-1i8; 52];

        let mut left = 0i32;
        let mut best = 0i32;
        let upper_bound = bytes.len().min(52) as i32;

        for (right, &byte) in bytes.iter().enumerate() {
            let slot = ((byte - b'a') as usize) << 1;
            let older = positions[slot] as i32;

            left = left.max(older + 1);

            positions[slot] = positions[slot + 1];
            positions[slot + 1] = right as i8;

            best = best.max(right as i32 - left + 1);

            if best == upper_bound {
                return best;
            }
        }

        best
    }
}
