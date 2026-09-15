/*
  2472. Maximum Number of Non-overlapping Palindrome Substrings
  
  You are given a string s and a positive integer k.
  Select a set of non-overlapping substrings from the string s that satisfy the following conditions:
      The length of each substring is at least k.
      Each substring is a palindrome.
  
  Return the maximum number of substrings in an optimal selection.
  
  A substring is a contiguous sequence of characters within a string.
   
  Example 1:
  Input: s = "abaccdbbd", k = 3
  Output: 2
  Explanation: We can select the substrings underlined in s = "abaccdbbd". Both "aba" and "dbbd" are palindromes and have a length of at least k = 3.
  It can be shown that we cannot find a selection with more than two valid substrings.
  
  Example 2:
  Input: s = "adbcda", k = 2
  Output: 0
  Explanation: There is no palindrome substring of length at least 2 in the string.
*/
impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        let mut odd = vec![0usize; n];
        let mut even = vec![0usize; n];

        let (mut left, mut right) = (0i32, -1i32);
        for center in 0..n {
            let c = center as i32;
            let mut radius = if c > right {
                1
            } else {
                odd[(left + right - c) as usize].min((right - c + 1) as usize)
            };

            while center >= radius
                && center + radius < n
                && bytes[center - radius] == bytes[center + radius]
            {
                radius += 1;
            }

            odd[center] = radius;

            if c + radius as i32 - 1 > right {
                left = c - radius as i32 + 1;
                right = c + radius as i32 - 1;
            }
        }

        (left, right) = (0, -1);

        for center in 0..n {
            let c = center as i32;
            let mut radius = if c > right {
                0
            } else {
                even[(left + right - c + 1) as usize].min((right - c + 1) as usize)
            };

            while center > radius
                && center + radius < n
                && bytes[center - radius - 1] == bytes[center + radius]
            {
                radius += 1;
            }

            even[center] = radius;

            if c + radius as i32 - 1 > right {
                left = c - radius as i32;
                right = c + radius as i32 - 1;
            }
        }

        fn is_palindrome(
            end: usize,
            length: usize,
            odd: &[usize],
            even: &[usize],
        ) -> bool {
            if end + 1 < length {
                return false;
            }

            let start = end + 1 - length;
            if length % 2 == 1 {
                odd[start + length / 2] >= length / 2 + 1
            } else {
                even[start + length / 2] >= length / 2
            }
        }

        let mut res = 0;
        let mut next_start = 0;

        for end in 0..n {
            let found = [k, k + 1].into_iter().any(|length| {
                end + 1 >= length
                    && end + 1 - length >= next_start
                    && is_palindrome(end, length, &odd, &even)
            });

            if found {
                res += 1;
                next_start = end + 1;
            }
        }

        res
    }
}
