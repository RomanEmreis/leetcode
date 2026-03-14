/*
  1415. The k-th Lexicographical String of All Happy Strings of Length n
  
  A happy string is a string that:
  
  consists only of letters of the set ['a', 'b', 'c'].
  s[i] != s[i + 1] for all values of i from 1 to s.length - 1 (string is 1-indexed).
  For example, strings "abc", "ac", "b" and "abcbabcbcb" are all happy strings and strings "aa", "baa" and "ababbc" are not happy strings.
  
  Given two integers n and k, consider a list of all happy strings of length n sorted in lexicographical order.
  
  Return the kth string of this list or return an empty string if there are less than k happy strings of length n.
  
  Example 1:
  Input: n = 1, k = 3
  Output: "c"
  Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".
  
  Example 2:
  Input: n = 1, k = 4
  Output: ""
  Explanation: There are only 3 happy strings of length 1.
  
  Example 3:
  Input: n = 3, k = 9
  Output: "cab"
  Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"
*/
impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let next = [1, 2, 0, 2, 0, 1];

        let mut pow = 1 << (n - 1);
        let mut code = (k - 1) / pow;

        k = (k - 1) % pow;

        let mut res = String::new();

        if code < 3 {
            while pow > 0 {
                res.push((b'a' + code as u8) as char);
                pow >>= 1;
                code = next[code as usize * 2 + (if (k & pow) == 0 { 0 } else { 1 })];
            }
        }

        res
    }
}
