/*
  3734. Lexicographically Smallest Palindromic Permutation Greater Than Target
  
  You are given two strings s and target, each of length n, consisting of lowercase English letters.
  
  Return the string that is both a of s and strictly greater than target. If no such permutation exists, return an empty string.
  
  Example 1:
  Input: s = "baba", target = "abba"
  Output: "baab"
  Explanation:
      The palindromic permutations of s (in lexicographical order) are "abba" and "baab".
      The lexicographically smallest permutation that is strictly greater than target is "baab".
  
  Example 2:
  Input: s = "baba", target = "bbaa"
  Output: ""
  Explanation:
      The palindromic permutations of s (in lexicographical order) are "abba" and "baab".
      None of them is lexicographically strictly greater than target. Therefore, the answer is "".
  
  Example 3:
  Input: s = "abc", target = "abb"
  Output: ""
  Explanation:
  s has no palindromic permutations. Therefore, the answer is "".
  
  Example 4:
  Input: s = "aac", target = "abb"
  Output: "aca"
  Explanation:
      The only palindromic permutation of s is "aca".
      "aca" is strictly greater than target. Therefore, the answer is "aca".
*/
impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let mut half = [0u16; 26];
        let mut middle = None;
        let mut odd_count = 0;

        for byte in s.bytes() {
            half[(byte - b'a') as usize] += 1;
        }

        for letter in 0..26 {
            if half[letter] % 2 != 0 {
                odd_count += 1;
                middle = Some(b'a' + letter as u8);
            }

            half[letter] /= 2;
        }

        if odd_count > 1 {
            return String::new();
        }

        let target_bytes = target.as_bytes();
        let n = target_bytes.len();
        let m = n / 2;

        if m == 0 {
            return if s.as_bytes() > target_bytes {
                s
            } else {
                String::new()
            };
        }

        let mut remaining = half;
        let mut available = 0u32;

        for (letter, &count) in remaining.iter().enumerate() {
            if count != 0 {
                available |= 1u32 << letter;
            }
        }

        let mut prefix = 0usize;

        while prefix < m {
            let letter = (target_bytes[prefix] - b'a') as usize;

            if remaining[letter] == 0 {
                break;
            }

            remaining[letter] -= 1;

            if remaining[letter] == 0 {
                available &= !(1u32 << letter);
            }

            prefix += 1;
        }

        let mut pivot;

        if prefix == m {
            let mut exact_is_greater = false;

            for index in m..n {
                let candidate = if n % 2 != 0 && index == m {
                    middle.unwrap()
                } else {
                    target_bytes[n - 1 - index]
                };

                if candidate != target_bytes[index] {
                    exact_is_greater = candidate > target_bytes[index];
                    break;
                }
            }

            if exact_is_greater {
                let mut output = s.into_bytes();
                output[..m].copy_from_slice(&target_bytes[..m]);

                if let Some(center) = middle {
                    output[m] = center;
                }

                for index in 0..m {
                    output[n - 1 - index] = output[index];
                }

                return unsafe {
                    String::from_utf8_unchecked(output)
                };
            }

            pivot = m - 1;
            let restored = (target_bytes[pivot] - b'a') as usize;
            remaining[restored] += 1;
            available |= 1u32 << restored;
        } else {
            pivot = prefix;
        }

        let next = loop {
            let current = (target_bytes[pivot] - b'a') as usize;
            let greater = available & (u32::MAX << (current + 1));

            if greater != 0 {
                break greater.trailing_zeros() as usize;
            }

            if pivot == 0 {
                return String::new();
            }

            pivot -= 1;
            let restored = (target_bytes[pivot] - b'a') as usize;
            remaining[restored] += 1;
            available |= 1u32 << restored;
        };

        remaining[next] -= 1;

        let mut output = s.into_bytes();
        output[..pivot].copy_from_slice(&target_bytes[..pivot]);

        let mut position = pivot;
        output[position] = b'a' + next as u8;
        position += 1;

        for (letter, &count) in remaining.iter().enumerate() {
            for _ in 0..count {
                output[position] = b'a' + letter as u8;
                position += 1;
            }
        }

        debug_assert_eq!(position, m);

        if let Some(center) = middle {
            output[m] = center;
        }

        for index in 0..m {
            output[n - 1 - index] = output[index];
        }

        unsafe { String::from_utf8_unchecked(output) }
    }
}
