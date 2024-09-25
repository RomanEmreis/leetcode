/*
  You are given an array words of size n consisting of non-empty strings.
  
  We define the score of a string word as the number of strings words[i] such that word is a prefix of words[i].
  
  For example, if words = ["a", "ab", "abc", "cab"], then the score of "ab" is 2, since "ab" is a prefix of both "ab" and "abc".
  Return an array answer of size n where answer[i] is the sum of scores of every non-empty prefix of words[i].
  
  Note that a string is considered as a prefix of itself.
  
  Example 1:
    Input: words = ["abc","ab","bc","b"]
    Output: [5,4,3,2]
    Explanation: The answer for each string is the following:
    - "abc" has 3 prefixes: "a", "ab", and "abc".
    - There are 2 strings with the prefix "a", 2 strings with the prefix "ab", and 1 string with the prefix "abc".
    The total is answer[0] = 2 + 2 + 1 = 5.
    - "ab" has 2 prefixes: "a" and "ab".
    - There are 2 strings with the prefix "a", and 2 strings with the prefix "ab".
    The total is answer[1] = 2 + 2 = 4.
    - "bc" has 2 prefixes: "b" and "bc".
    - There are 2 strings with the prefix "b", and 1 string with the prefix "bc".
    The total is answer[2] = 2 + 1 = 3.
    - "b" has 1 prefix: "b".
    - There are 2 strings with the prefix "b".
    The total is answer[3] = 2.
*/
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    count: i32
}

struct Trie {
    root: TrieNode
}

impl Trie {
    #[inline]
    fn new() -> Self {
        Self { root: TrieNode::default() }
    }

    #[inline]
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
            node.count += 1;
        }
    }

    #[inline]
    fn get_prefix_score(&self, word: &str) -> i32 {
        let mut node = &self.root;
        let mut score = 0;
        for c in word.chars() {
            if let Some(next_node) = node.children.get(&c) {
                score += next_node.count;
                node = next_node;
            } else {
                break;
            }
        }
        score
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }

        words.iter()
            .map(|word| trie.get_prefix_score(&word))
            .collect()
    }
}
