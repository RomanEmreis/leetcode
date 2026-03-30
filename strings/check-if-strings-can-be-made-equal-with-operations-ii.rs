/*
  2840. Check if Strings Can be Made Equal With Operations II
  
  You are given two strings s1 and s2, both of length n, consisting of lowercase English letters.
  You can apply the following operation on any of the two strings any number of times:
      Choose any two indices i and j such that i < j and the difference j - i is even, then swap the two characters at those indices in the string.
  
  Return true if you can make the strings s1 and s2 equal, and false otherwise.
  
  Example 1:
  Input: s1 = "abcdba", s2 = "cabdab"
  Output: true
  Explanation: We can apply the following operations on s1:
  - Choose the indices i = 0, j = 2. The resulting string is s1 = "cbadba".
  - Choose the indices i = 2, j = 4. The resulting string is s1 = "cbbdaa".
  - Choose the indices i = 1, j = 5. The resulting string is s1 = "cabdab" = s2.
  
  Example 2:
  Input: s1 = "abe", s2 = "bea"
  Output: false
  Explanation: It is not possible to make the two strings equal.
*/
impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut cnt = [0; 256];

        let b1 = s1.into_bytes();
        let b2 = s2.into_bytes();

        for i in 0..b1.len() {
            let offset = (i & 1) << 7;
            cnt[offset + b1[i] as usize] += 1;
            cnt[offset + b2[i] as usize] -= 1;
        }

        cnt.iter().all(|&c| c == 0)
    }
}
