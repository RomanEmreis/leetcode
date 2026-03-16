/*
  642. Design Search Autocomplete System
  
  Design a search autocomplete system for a search engine. Users may input a sentence (at least one word and end with a special character '#').
  
  You are given a string array sentences and an integer array times both of length n where sentences[i] is a previously typed sentence and times[i]
  is the corresponding number of times the sentence was typed. For each input character except '#', return the top 3 historical hot sentences 
  that have the same prefix as the part of the sentence already typed.
  
  Here are the specific rules:
      The hot degree for a sentence is defined as the number of times a user typed the exactly same sentence before.
      The returned top 3 hot sentences should be sorted by hot degree (The first is the hottest one). 
      If several sentences have the same hot degree, use ASCII-code order (smaller one appears first).
      If less than 3 hot sentences exist, return as many as you can.
      When the input is a special character, it means the sentence ends, and in this case, you need to return an empty list.
  
  Implement the AutocompleteSystem class:
  
      AutocompleteSystem(String[] sentences, int[] times) Initializes the object with the sentences and times arrays.
      List<String> input(char c) This indicates that the user typed the character c.
          Returns an empty array [] if c == '#' and stores the inputted sentence in the system.
          Returns the top 3 historical hot sentences that have the same prefix as the part of the sentence already typed. If there are fewer than 3 matches, return them all.
  
  Example 1:
  Input
  ["AutocompleteSystem", "input", "input", "input", "input"]
  [[["i love you", "island", "iroman", "i love leetcode"], [5, 3, 2, 2]], ["i"], [" "], ["a"], ["#"]]
  Output
  [null, ["i love you", "island", "i love leetcode"], ["i love you", "i love leetcode"], [], []]
  
  Explanation
  AutocompleteSystem obj = new AutocompleteSystem(["i love you", "island", "iroman", "i love leetcode"], [5, 3, 2, 2]);
  obj.input("i"); // return ["i love you", "island", "i love leetcode"]. There are four sentences that have prefix "i". Among them, "ironman" and "i love leetcode" have same hot degree. 
  Since ' ' has ASCII code 32 and 'r' has ASCII code 114, "i love leetcode" should be in front of "ironman". Also we only need to output top 3 hot sentences, so "ironman" will be ignored.
  obj.input(" "); // return ["i love you", "i love leetcode"]. There are only two sentences that have prefix "i ".
  obj.input("a"); // return []. There are no sentences that have prefix "i a".
  obj.input("#"); // return []. The user finished the input, the sentence "i a" should be saved as a historical sentence in system. And the following input will be counted as a new search.
*/
use std::collections::HashMap;

const NONE: usize = usize::MAX;

struct TrieNode {
    children: [usize; 128],
    top3: Vec<String>,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: [NONE; 128],
            top3: Vec::with_capacity(3),
        }
    }
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: [NONE; 128],
            top3: Vec::with_capacity(3),
        }
    }
}

struct AutocompleteSystem {
    arena: Vec<TrieNode>,
    counts: HashMap<String, i32>,
    prefix: String,
    current: usize, 
}

impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut sys = Self {
            arena: vec![TrieNode::new()], // root = 0
            counts: HashMap::new(),
            prefix: String::new(),
            current: 0,
        };
        for (sentence, &time) in sentences.iter().zip(times.iter()) {
            sys.counts.insert(sentence.clone(), time);
            sys.add_to_trie(sentence);
        }
        sys
    }

    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            let sentence = std::mem::take(&mut self.prefix);
            let count = self.counts.entry(sentence.clone()).or_insert(0);
            *count += 1;
            if *count == 1 {
                self.add_to_trie(&sentence);
            } else {
                self.update_trie(&sentence);
            }
            self.current = 0;
            return vec![];
        }

        self.prefix.push(c);

        if self.current == NONE {
            return vec![];
        }

        let idx = c as usize;
        let next = self.arena[self.current].children[idx];
        if next == NONE {
            self.current = NONE;
            return vec![];
        }
        self.current = next;
        self.arena[self.current].top3.clone()
    }

    fn add_to_trie(&mut self, sentence: &str) {
        let mut node = 0usize;
        for ch in sentence.chars() {
            let idx = ch as usize;
            if self.arena[node].children[idx] == NONE {
                self.arena.push(TrieNode::new());
                let new_idx = self.arena.len() - 1;
                self.arena[node].children[idx] = new_idx;
            }
            node = self.arena[node].children[idx];
            Self::insert_into_top3(&mut self.arena[node].top3, sentence, &self.counts);
        }
    }

    fn update_trie(&mut self, sentence: &str) {
        let mut node = 0usize;
        for ch in sentence.chars() {
            node = self.arena[node].children[ch as usize];
            let top = &mut self.arena[node].top3;
            top.retain(|s| s != sentence);
            Self::insert_into_top3(top, sentence, &self.counts);
        }
    }

    fn insert_into_top3(top: &mut Vec<String>, sentence: &str, counts: &HashMap<String, i32>) {
        let new_times = counts[sentence];
        let pos = top.iter().position(|s| {
            let t = counts[s.as_str()];
            new_times > t || (new_times == t && sentence < s.as_str())
        });
        let insert_at = pos.unwrap_or(top.len());
        if insert_at < 3 {
            top.insert(insert_at, sentence.to_owned());
            if top.len() > 3 {
                top.pop();
            }
        }
    }
}
