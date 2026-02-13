/*
  3714. Longest Balanced Substring II
  
  You are given a string s consisting only of the characters 'a', 'b', and 'c'.
  A of s is called balanced if all distinct characters in the substring appear the same number of times.
  
  Return the length of the longest balanced substring of s.
  
  Example 1:
  Input: s = "abbac"
  Output: 4
  Explanation:
  The longest balanced substring is "abba" because both distinct characters 'a' and 'b' each appear exactly 2 times.
  
  Example 2:
  Input: s = "aabcc"
  Output: 3
  Explanation:
  The longest balanced substring is "abc" because all distinct characters 'a', 'b' and 'c' each appear exactly 1 time.
  
  Example 3:
  Input: s = "aba"
  Output: 2
  Explanation:
  One of the longest balanced substrings is "ab" because both distinct characters 'a' and 'b' each appear exactly 1 time. Another longest balanced substring is "ba".
*/
use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let b = s.into_bytes();
        
        let x = Self::calc1(&b);
        let y = Self::calc2(&b, b'a', b'b')
            .max(Self::calc2(&b, b'b', b'c'))
            .max(Self::calc2(&b, b'a', b'c'));
        
        let z = Self::calc3(&b);

        x.max(y).max(z)
    }

    fn calc1(bytes: &[u8]) -> i32 {
        let mut res = 0i32;
        let mut i = 0usize;
        let n = bytes.len();

        while i < n {
            let mut j = i + 1;
            while j < n && bytes[j] == bytes[i] {
                j += 1;
            }
            res = res.max((j - i) as i32);
            i = j;
        }
        res
    }

    fn calc2(bytes: &[u8], a: u8, b: u8) -> i32 {
        let mut res = 0i32;
        let mut i = 0usize;
        let n = bytes.len();

        while i < n {
            while i < n && bytes[i] != a && bytes[i] != b {
                i += 1;
            }

            let mut pos: HashMap<i32, i32> = HashMap::new();
            pos.insert(0, i as i32 - 1);

            let mut d = 0i32;
            while i < n && (bytes[i] == a || bytes[i] == b) {
                if bytes[i] == a {
                    d += 1;
                } else {
                    d -= 1;
                }

                if let Some(&pre) = pos.get(&d) {
                    res = res.max(i as i32 - pre);
                } else {
                    pos.insert(d, i as i32);
                }
                i += 1;
            }
        }

        res
    }

    #[inline(always)]
    fn f(x: i32, y: i32) -> i64 {
        (((x + 100000) as i64) << 20) | ((y + 100000) as i64)
    }

    fn calc3(bytes: &[u8]) -> i32 {
        let mut pos: HashMap<i64, i32> = HashMap::new();
        pos.insert(Self::f(0, 0), -1);

        let mut cnt = [0i32; 3];
        let mut res = 0i32;

        for (i, c) in bytes.iter().enumerate() {
            cnt[(c - b'a') as usize] += 1;

            let x = cnt[0] - cnt[1];
            let y = cnt[1] - cnt[2];
            let k = Self::f(x, y);

            if let Some(&pre) = pos.get(&k) {
                res = res.max(i as i32 - pre);
            } else {
                pos.insert(k, i as i32);
            }
        }

        res
    }
}
