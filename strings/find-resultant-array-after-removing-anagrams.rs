/*
  2273. Find Resultant Array After Removing Anagrams
  
  You are given a 0-indexed string array words, where words[i] consists of lowercase English letters.
  In one operation, select any index i such that 0 < i < words.length and words[i - 1] and words[i] are anagrams, and delete words[i] from words. Keep performing this operation as long as you can select an index that satisfies the conditions.
  
  Return words after performing all operations. It can be shown that selecting the indices for each operation in any arbitrary order will lead to the same result.
  
  An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase using all the original letters exactly once. For example, "dacb" is an anagram of "abdc".
  
  Example 1:
  Input: words = ["abba","baba","bbaa","cd","cd"]
  Output: ["abba","cd"]
  Explanation:
  One of the ways we can obtain the resultant array is by using the following operations:
  - Since words[2] = "bbaa" and words[1] = "baba" are anagrams, we choose index 2 and delete words[2].
    Now words = ["abba","baba","cd","cd"].
  - Since words[1] = "baba" and words[0] = "abba" are anagrams, we choose index 1 and delete words[1].
    Now words = ["abba","cd","cd"].
  - Since words[2] = "cd" and words[1] = "cd" are anagrams, we choose index 2 and delete words[2].
    Now words = ["abba","cd"].
  We can no longer perform any operations, so ["abba","cd"] is the final answer.
  
  Example 2:
  Input: words = ["a","b","c","d","e"]
  Output: ["a","b","c","d","e"]
  Explanation:
  No two adjacent strings in words are anagrams of each other, so no operations are performed.
*/
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![words[0].clone()];
        for i in 1..words.len() {
            if !is_anagram(&words[i - 1], &words[i]) {
                res.push(words[i].clone());
            }
        }
        res
    }
}

fn is_anagram(s1: &String, s2: &String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut freq = [0; 26];

    for b in s1.as_bytes() {
        freq[(b - b'a') as usize] += 1;
    }
    for b in s2.as_bytes() {
        let i = (b - b'a') as usize;
        freq[i] -= 1;
        if freq[i] < 0 {
            return false;
        }
    }

    true
}
