/*
  966. Vowel Spellchecker
  
  Given a wordlist, we want to implement a spellchecker that converts a query word into a correct word.
  For a given query word, the spell checker handles two categories of spelling mistakes:
      Capitalization: If the query matches a word in the wordlist (case-insensitive), then the query word is returned with the same case as the case in the wordlist.
          Example: wordlist = ["yellow"], query = "YellOw": correct = "yellow"
          Example: wordlist = ["Yellow"], query = "yellow": correct = "Yellow"
          Example: wordlist = ["yellow"], query = "yellow": correct = "yellow"
      Vowel Errors: If after replacing the vowels ('a', 'e', 'i', 'o', 'u') of the query word with any vowel individually, it matches a word in the wordlist (case-insensitive), then the query word is returned with the same case as the match in the wordlist.
          Example: wordlist = ["YellOw"], query = "yollow": correct = "YellOw"
          Example: wordlist = ["YellOw"], query = "yeellow": correct = "" (no match)
          Example: wordlist = ["YellOw"], query = "yllw": correct = "" (no match)
  
  In addition, the spell checker operates under the following precedence rules:
      When the query exactly matches a word in the wordlist (case-sensitive), you should return the same word back.
      When the query matches a word up to capitlization, you should return the first such match in the wordlist.
      When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
      If the query has no matches in the wordlist, you should return the empty string.
  
  Given some queries, return a list of words answer, where answer[i] is the correct word for query = queries[i].
  
  Example 1:
  Input: wordlist = ["KiTe","kite","hare","Hare"], queries = ["kite","Kite","KiTe","Hare","HARE","Hear","hear","keti","keet","keto"]
  Output: ["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]
  
  Example 2:
  Input: wordlist = ["yellow"], queries = ["YellOw"]
  Output: ["yellow"]
*/
use std::collections::HashMap;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let n = queries.len();
        let mut res = Vec::with_capacity(n);
        let mut dict = HashMap::with_capacity(n);
        wordlist.iter().for_each(|w| {
            dict.entry(w.clone()).or_insert(w.clone());
            dict.entry(lower(w)).or_insert(w.clone());
            dict.entry(vowel(w)).or_insert(w.clone());
        });
        queries.iter().for_each(|q| {
            if let Some(w) = dict.get(q) {
                res.push(w.clone());
            } else if let Some(w) = dict.get(&lower(q)) {
                res.push(w.clone());
            } else if let Some(w) = dict.get(&vowel(q)) {
                res.push(w.clone());
            } else {
                res.push("".to_owned());
            }
        });
        res
    }
}

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

fn lower(word: &String) -> String {
    let mut res = String::with_capacity(1 + word.len());
    res.push('$');
    res.extend(word.chars().flat_map(|c| c.to_lowercase()));
    res
}

fn vowel(word: &String) -> String {
    let mut res = String::with_capacity(word.len());
    for ch in word.chars() {
        if VOWELS.contains(&ch) {
            res.push('*');
        } else {
            res.extend(ch.to_lowercase())
        }
    }
    res
}
