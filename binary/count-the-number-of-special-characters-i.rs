/*
  3120. Count the Number of Special Characters I
  
  You are given a string word. A letter is called special if it appears both in lowercase and uppercase in word.
  
  Return the number of special letters in word.
  
  Example 1:
  Input: word = "aaAbcBC"
  Output: 3
  Explanation:
  The special characters in word are 'a', 'b', and 'c'.
  
  Example 2:
  Input: word = "abc"
  Output: 0
  Explanation:
  No character in word appears in uppercase.
  
  Example 3:
  Input: word = "abBCab"
  Output: 1
  Explanation:
  The only special character in word is 'b'.
*/
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut u = [0_u8; 27];
        let lt = 0x60_u8;
        let m = 0x1F_u8;

        let b1 = 1_u8;
        let b2 = 2_u8;
        let b3 = 3_u8;

        for c in word.into_bytes() {
            let cc = (c & m) as usize;
            if u[cc] == b3 {
                continue;
            }
            u[cc] |= if (c & lt) == lt { b1 } else { b2 };
        }

        let mut cnt = 0;
        for i in (0..=26).rev() {
            if u[i] == b3 {
                cnt += 1;
            }
        }

        cnt
    }
}
