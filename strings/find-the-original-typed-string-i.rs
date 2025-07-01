/*
  3330. Find the Original Typed String I
  
  Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.
  Although Alice tried to focus on her typing, she is aware that she may still have done this at most once.
  You are given a string word, which represents the final output displayed on Alice's screen.
  
  Return the total number of possible original strings that Alice might have intended to type.
  
  Example 1:
  Input: word = "abbcccc"
  Output: 5
  Explanation:
  The possible strings are: "abbcccc", "abbccc", "abbcc", "abbc", and "abcccc".
  
  Example 2:
  Input: word = "abcd"
  Output: 1
  Explanation:
  The only possible string is "abcd".
  
  Example 3:
  Input: word = "aaaa"
  Output: 4
*/
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let n = word.len();
        let bytes = word.as_bytes();
        let mut result = 1;
        for i in 1..n {
            if bytes[i] == bytes[i - 1] {
                result += 1;
            }
        }
        result
    }
}
