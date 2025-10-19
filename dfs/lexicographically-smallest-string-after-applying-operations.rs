/*
  1625. Lexicographically Smallest String After Applying Operations
  
  You are given a string s of even length consisting of digits from 0 to 9, and two integers a and b.
  You can apply either of the following two operations any number of times and in any order on s:
      Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example, if s = "3456" and a = 5, s becomes "3951".
      Rotate s to the right by b positions. For example, if s = "3456" and b = 1, s becomes "6345".
  
  Return the lexicographically smallest string you can obtain by applying the above operations any number of times on s.
  
  A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, 
  string a has a letter that appears earlier in the alphabet than the corresponding letter in b.
  For example, "0158" is lexicographically smaller than "0190" because the first position they differ is at the third letter, and '5' comes before '9'.
  
  Example 1:
  Input: s = "5525", a = 9, b = 2
  Output: "2050"
  Explanation: We can apply the following operations:
  Start:  "5525"
  Rotate: "2555"
  Add:    "2454"
  Add:    "2353"
  Rotate: "5323"
  Add:    "5222"
  Add:    "5121"
  Rotate: "2151"
  Add:    "2050"​​​​​
  There is no way to obtain a string that is lexicographically smaller than "2050".
  
  Example 2:
  Input: s = "74", a = 5, b = 1
  Output: "24"
  Explanation: We can apply the following operations:
  Start:  "74"
  Rotate: "47"
  ​​​​​​​Add:    "42"
  ​​​​​​​Rotate: "24"​​​​​​​​​​​​
  There is no way to obtain a string that is lexicographically smaller than "24".
  
  Example 3:
  Input: s = "0011", a = 4, b = 2
  Output: "0011"
  Explanation: There are no sequence of operations that will give us a lexicographically smaller string than "0011".
*/
impl Solution {
    pub fn find_lex_smallest_string(mut s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let g = gcd(b as usize, n);
        
        let mut res = s.clone();
        s += &s.clone();

        for i in (0..n).step_by(g) {
            let mut t: Vec<char> = s[i..i + n].chars().collect();
            add(&mut t, n, a, 1);

            if b % 2 != 0 {
                add(&mut t, n, a, 0);
            }

            let t_str: String = t.iter().collect();
            if t_str < res {
                res = t_str;
            }
        }

        res
    }
}

#[inline]
fn add(t: &mut [char], n: usize, a: i32, start: usize) {
    let mut min = 10;
    let mut times = 0;

    for i in 0..10 {
        let added = ((t[start] as i32 - '0' as i32) + i * a) % 10;
        if added < min {
            min = added;
            times = i;
        }
    }

    for i in (start..n).step_by(2) {
        let v = ((t[i] as i32 - '0' as i32) + times * a) % 10;
        t[i] = std::char::from_digit(v as u32, 10).unwrap();
    }
}

#[inline]
fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}
