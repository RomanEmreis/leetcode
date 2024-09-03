/*
  You are given a string s consisting of lowercase English letters, and an integer k.
  First, convert s into an integer by replacing each letter with its position in the alphabet (i.e., replace 'a' with 1, 'b' with 2, ..., 'z' with 26). Then, transform the integer by replacing it with the sum of its digits.
  Repeat the transform operation k times in total.
  For example, if s = "zbax" and k = 2, then the resulting integer would be 8 by the following operations:
  
  Convert: "zbax" ➝ "(26)(2)(1)(24)" ➝ "262124" ➝ 262124
  Transform #1: 262124 ➝ 2 + 6 + 2 + 1 + 2 + 4 ➝ 17
  Transform #2: 17 ➝ 1 + 7 ➝ 8
  Return the resulting integer after performing the operations described above.
  
  Example 1:
    Input: s = "iiii", k = 1
    Output: 36
    Explanation: The operations are as follows:
    - Convert: "iiii" ➝ "(9)(9)(9)(9)" ➝ "9999" ➝ 9999
    - Transform #1: 9999 ➝ 9 + 9 + 9 + 9 ➝ 36
    Thus the resulting integer is 36.
*/
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut prefix = 0;
        for ch in s.chars() {
            let value = ch as i32 - 'a' as i32 + 1;

            prefix += value % 10;
            prefix += value / 10;
        }

        for _ in 1..k {
            let mut sum = 0;
            while prefix > 0 {
                sum += prefix % 10;
                prefix /= 10;
            }
            prefix = sum;
        }

        prefix
    }
}
