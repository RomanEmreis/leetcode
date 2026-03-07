/*
  1888. Minimum Number of Flips to Make the Binary String Alternating
  
  You are given a binary string s. You are allowed to perform two types of operations on the string in any sequence:
      Type-1: Remove the character at the start of the string s and append it to the end of the string.
      Type-2: Pick any character in s and flip its value, i.e., if its value is '0' it becomes '1' and vice-versa.
  
  Return the minimum number of type-2 operations you need to perform such that s becomes alternating.
  
  The string is called alternating if no two adjacent characters are equal.
      For example, the strings "010" and "1010" are alternating, while the string "0100" is not.
  
  Example 1:
  Input: s = "111000"
  Output: 2
  Explanation: Use the first operation two times to make s = "100011".
  Then, use the second operation on the third and sixth elements to make s = "101010".
  
  Example 2:
  Input: s = "010"
  Output: 0
  Explanation: The string is already alternating.
  
  Example 3:
  Input: s = "1110"
  Output: 1
  Explanation: Use the second operation on the second element to make s = "1010".
*/
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let b = s.into_bytes();
        
        let mut cnt = [[0; 2]; 2];
        for i in 0..n {
            cnt[(b[i] - b'0') as usize][i % 2] += 1;
        }

        let mut res = std::cmp::min(
            cnt[0][0] + cnt[1][1],
            cnt[1][0] + cnt[0][1]
        );

        for i in 0..n {
            let j = (b[i] - b'0') as usize;
            cnt[j][i % 2] -= 1;
            cnt[j][(n + i) % 2] += 1;

            res = res.min(std::cmp::min(
                cnt[0][0] + cnt[1][1],
                cnt[1][0] + cnt[0][1]
            ));
        }
        
        res
    }
}
