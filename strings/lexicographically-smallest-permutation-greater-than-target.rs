/*
  3720. Lexicographically Smallest Permutation Greater Than Target
  
  You are given two strings s and target, both having length n, consisting of lowercase English letters.
  
  Return the lexicographically smallest of s that is strictly greater than target. If no permutation of s is lexicographically strictly greater than target, return an empty string.
  
  A string a is lexicographically strictly greater than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears later in the alphabet than the corresponding letter in b.
   
  Example 1:
  Input: s = "abc", target = "bba"
  Output: "bca"
  Explanation:
      The permutations of s (in lexicographical order) are "abc", "acb", "bac", "bca", "cab", and "cba".
      The lexicographically smallest permutation that is strictly greater than target is "bca".
  
  Example 2:
  Input: s = "leet", target = "code"
  Output: "eelt"
  Explanation:
      The permutations of s (in lexicographical order) are "eelt", "eetl", "elet", "elte", "etel", "etle", "leet", "lete", "ltee", "teel", "tele", and "tlee".
      The lexicographically smallest permutation that is strictly greater than target is "eelt".
  
  Example 3:
  Input: s = "baba", target = "bbaa"
  Output: ""
  Explanation:
      The permutations of s (in lexicographical order) are "aabb", "abab", "abba", "baab", "baba", and "bbaa".
      None of them is lexicographically strictly greater than target. Therefore, the answer is "".
*/
impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut counts = [0u16; 26];
        for byte in s.bytes() {
            counts[(byte - b'a') as usize] += 1;
        }

        let mut available = 0u32;
        for (letter, &count) in counts.iter().enumerate() {
            if count != 0 {
                available |= 1u32 << letter;
            }
        }

        let target_bytes = target.into_bytes();
        let candidate = {
            let n = target_bytes.len();
            let mut prefix = 0usize;
            while prefix < n {
                let letter = (target_bytes[prefix] - b'a') as usize;
                if counts[letter] == 0 {
                    break;
                }

                counts[letter] -= 1;

                if counts[letter] == 0 {
                    available &= !(1u32 << letter);
                }

                prefix += 1;
            }

            let mut pivot;

            if prefix == n {
                pivot = n - 1;
                let restored = (target_bytes[pivot] - b'a') as usize;

                if counts[restored] == 0 {
                    available |= 1u32 << restored;
                }

                counts[restored] += 1;
            } else {
                pivot = prefix;
            }

            loop {
                let target_letter = (target_bytes[pivot] - b'a') as usize;
                let greater = available & (u32::MAX << (target_letter + 1));

                if greater != 0 {
                    let next = greater.trailing_zeros() as usize;

                    counts[next] -= 1;
                    break Some((pivot, next, counts));
                }

                if pivot == 0 {
                    break None;
                }

                pivot -= 1;

                let restored = (target_bytes[pivot] - b'a') as usize;
                if counts[restored] == 0 {
                    available |= 1u32 << restored;
                }

                counts[restored] += 1;
            }
        };

        let Some((pivot, next, counts)) = candidate else {
            return String::new();
        };

        let mut output = s.into_bytes();
        output[..pivot].copy_from_slice(&target_bytes[..pivot]);

        let mut position = pivot;
        output[position] = b'a' + next as u8;
        position += 1;

        for (letter, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                output[position] = b'a' + letter as u8;
                position += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(output) }
    }
}
