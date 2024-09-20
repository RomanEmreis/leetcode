/*
  You are given a string s. You can convert s to a palindrome by adding characters in front of it.
  
  Return the shortest palindrome you can find by performing this transformation.
  
  Example 1:
  Input: s = "aacecaaa"
  Output: "aaacecaaa"
  
  Example 2:
  Input: s = "abcd"
  Output: "dcbabcd"
*/
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() <= 1 { return s; }

        let m: i64 = 1000000007;
        let base: i64 = 26;

        let (mut hash, mut reverse_hash, mut power) = (0i64, 0i64, 1i64);
        let (mut length, mut idx) = (0, 0);

        for ch in s.chars() {
            let c_num = (ch as i64) - ('a' as i64) + 1;
            hash = (hash * base + c_num) % m;
            reverse_hash = (reverse_hash + c_num * power) % m;
            power = (power * base) % m;

            if hash == reverse_hash { length = idx + 1; }
            idx += 1;
        }

        let rev_suffix = Self::reverse(&s[length..]);
        format!("{}{}", rev_suffix, s)
    }

    fn reverse(s: &str) -> String {
        s.chars().rev().collect::<String>()
    }
}
