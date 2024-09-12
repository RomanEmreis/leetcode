/*
  You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.
  
  Return the number of consistent strings in the array words.
  
  Example 1:
    Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
    Output: 2
    Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
*/
use std::collections::HashSet;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let set: HashSet<char> = allowed.chars().collect();

        words.iter()
            .filter(|&word| word.chars().all(|ch| set.contains(&ch)))
            .count() as i32
    }
}
