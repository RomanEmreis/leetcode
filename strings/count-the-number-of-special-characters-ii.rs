/*
  3121. Count the Number of Special Characters II
  
  You are given a string word. A letter c is called special if it appears both in lowercase and uppercase in word,
  and every lowercase occurrence of c appears before the first uppercase occurrence of c.
  
  Return the number of special letters in word.
  
  Example 1:
  Input: word = "aaAbcBC"
  Output: 3
  Explanation:
  The special characters are 'a', 'b', and 'c'.
  
  Example 2:
  Input: word = "abc"
  Output: 0
  Explanation:
  There are no special characters in word.
  
  Example 3:
  Input: word = "AbBCab"
  Output: 0
  Explanation:
  There are no special characters in word.
*/
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = [false; 26];
        let mut upper = [false; 26];
        
        word.into_bytes().into_iter().for_each(|ch| {
            if ch.is_ascii_lowercase() {
                let ch = (ch - b'a') as usize;
                lower[ch] = !upper[ch];
            } else {
                upper[(ch - b'A') as usize] = true;
            }
        });

        (0..26)
            .filter(|&i| lower[i] && upper[i])
            .count() as i32
    }
}
