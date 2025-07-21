/*
  1957. Delete Characters to Make Fancy String
  
  A fancy string is a string where no three consecutive characters are equal.
  
  Given a string s, delete the minimum possible number of characters from s to make it fancy.
  
  Return the final string after the deletion. It can be shown that the answer will always be unique.
  
  Example 1:
  Input: s = "leeetcode"
  Output: "leetcode"
  Explanation:
  Remove an 'e' from the first group of 'e's to create "leetcode".
  No three consecutive characters are equal, so return "leetcode".
  
  Example 2:
  Input: s = "aaabaaaa"
  Output: "aabaa"
  Explanation:
  Remove an 'a' from the first group of 'a's to create "aabaaaa".
  Remove two 'a's from the second group of 'a's to create "aabaa".
  No three consecutive characters are equal, so return "aabaa".
  
  Example 3:
  Input: s = "aab"
  Output: "aab"
  Explanation: No three consecutive characters are equal, so return "aab".
*/
impl Solution {
    pub fn make_fancy_string(mut s: String) -> String {
        let n = s.len();
        let mut write = 2;
        let mut bytes = unsafe { s.as_mut_vec() };
        for read in 2..n {
            if bytes[read] != bytes[write - 1] || bytes[read] != bytes[write - 2] {
                bytes[write] = bytes[read];
                write += 1;
            }
        }
        bytes.truncate(write);
        s
    }
}
