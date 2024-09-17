/*
  A sentence is a string of single-space separated words where each word consists only of lowercase letters.
  A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
  
  Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
  
  Example 1:
    Input: s1 = "this apple is sweet", s2 = "this apple is sour"
    Output: ["sweet","sour"]
    Explanation:
    The word "sweet" appears only in s1, while the word "sour" appears only in s2.
  
  Example 2:
    Input: s1 = "apple apple", s2 = "banana"
    Output: ["banana"]
*/
use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::new();
        for s in s1.split_whitespace() {
            map.entry(s).and_modify(|c| *c += 1).or_insert(1);
        }
        for s in s2.split_whitespace() {
            map.entry(s).and_modify(|c| *c += 1).or_insert(1);
        }

        map.into_iter()
            .filter(|&(_, v)| v == 1)
            .map(|(k, _)| String::from(k))
            .collect()
    }
}
