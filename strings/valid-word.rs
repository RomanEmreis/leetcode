/*
  3136. Valid Word
  
  A word is considered valid if:
    It contains a minimum of 3 characters.
    It contains only digits (0-9), and English letters (uppercase and lowercase).
    It includes at least one vowel.
    It includes at least one consonant.

  You are given a string word.
  
  Return true if word is valid, otherwise, return false.
  
  Notes:
  'a', 'e', 'i', 'o', 'u', and their uppercases are vowels.
  A consonant is an English letter that is not a vowel.
  
  Example 1:
  Input: word = "234Adas"
  Output: true
  Explanation:
  This word satisfies the conditions.
  
  Example 2:
  Input: word = "b3"
  Output: false
  Explanation:
  The length of this word is fewer than 3, and does not have a vowel.
  
  Example 3:
  Input: word = "a3$e"
  Output: false
  Explanation:
  This word contains a '$' character and does not have a consonant.
*/
impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut has_vowel = false;
        let mut has_consonant = false;
        for ch in word.chars() {
            if ch.is_numeric() {
                continue;
            }
            if !ch.is_alphabetic() {
                return false;
            }
            if Self::is_vowel(&ch) {
                has_vowel = true;
            } else {
                has_consonant = true;
            }
        }
        has_vowel && has_consonant
    }

    fn is_vowel(ch: &char) -> bool {
        match ch {
            'a' => true,
            'A' => true,
            'e' => true,
            'E' => true,
            'i' => true,
            'I' => true,
            'o' => true,
            'O' => true,
            'u' => true,
            'U' => true,
            _ => false
        }
    }
}
