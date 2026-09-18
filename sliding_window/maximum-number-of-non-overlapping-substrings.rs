/*
  1520. Maximum Number of Non-Overlapping Substrings
  
  Given a string s of lowercase letters, you need to find the maximum number of non-empty substrings of s that meet the following conditions:
      The substrings do not overlap, that is for any two substrings s[i..j] and s[x..y], either j < x or i > y is true.
      A substring that contains a certain character c must also contain all occurrences of c.
  
  Find the maximum number of substrings that meet the above conditions. If there are multiple solutions with the same number of substrings, 
  return the one with minimum total length. It can be shown that there exists a unique solution of minimum total length.
  
  Notice that you can return the substrings in any order.
  
  Example 1:
  Input: s = "adefaddaccc"
  Output: ["e","f","ccc"]
  Explanation: The following are all the possible substrings that meet the conditions:
  [
    "adefaddaccc"
    "adefadda",
    "ef",
    "e",
    "f",
    "ccc",
  ]
  If we choose the first string, we cannot choose anything else and we'd get only 1. 
  If we choose "adefadda", we are left with "ccc" which is the only one that doesn't overlap, thus obtaining 2 substrings. 
  Notice also, that it's not optimal to choose "ef" since it can be split into two. Therefore, 
  the optimal way is to choose ["e","f","ccc"] which gives us 3 substrings. No other solution of the same number of substrings exist.
  
  Example 2:
  Input: s = "abbaccd"
  Output: ["d","bb","cc"]
  Explanation: Notice that while the set of substrings ["d","abba","cc"] also has length 3, it's considered incorrect since it has larger total length.
*/
impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut first = [n; 26];
        let mut last = [0usize; 26];

        for index in 0..n {
            let letter = (bytes[index] - b'a') as usize;

            if first[letter] == n {
                first[letter] = index;
            }

            last[letter] = index;
        }

        let mut selected: Vec<(usize, usize)> =
            Vec::with_capacity(26);

        for start in 0..n {
            let initial = (bytes[start] - b'a') as usize;

            if first[initial] != start {
                continue;
            }

            let mut end = last[initial];
            let mut index = start;
            let mut valid = true;

            while index <= end {
                let letter = (bytes[index] - b'a') as usize;

                if first[letter] < start {
                    valid = false;
                    break;
                }

                end = end.max(last[letter]);
                index += 1;
            }

            if !valid {
                continue;
            }

            match selected.last_mut() {
                Some(previous) if start <= previous.1 => {
                    *previous = (start, end);
                }
                _ => selected.push((start, end)),
            }
        }

        selected
            .into_iter()
            .map(|(start, end)| s[start..=end].to_owned())
            .collect()
    }
}
