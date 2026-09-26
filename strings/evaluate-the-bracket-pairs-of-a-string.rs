/*
  1807. Evaluate the Bracket Pairs of a String
  
  You are given a string s that contains some bracket pairs, with each pair containing a non-empty key.
      For example, in the string "(name)is(age)yearsold", there are two bracket pairs that contain the keys "name" and "age".
  
  You know the values of a wide range of keys. This is represented by a 2D string array knowledge where each knowledge[i] = [keyi, valuei] indicates that key keyi has a value of valuei.
  You are tasked to evaluate all of the bracket pairs. When you evaluate a bracket pair that contains some key keyi, you will:
      Replace keyi and the bracket pair with the key's corresponding valuei.
      If you do not know the value of the key, you will replace keyi and the bracket pair with a question mark "?" (without the quotation marks).
  
  Each key will appear at most once in your knowledge. There will not be any nested brackets in s.
  
  Return the resulting string after evaluating all of the bracket pairs.
  
  Example 1:
  Input: s = "(name)is(age)yearsold", knowledge = [["name","bob"],["age","two"]]
  Output: "bobistwoyearsold"
  Explanation:
  The key "name" has a value of "bob", so replace "(name)" with "bob".
  The key "age" has a value of "two", so replace "(age)" with "two".
  
  Example 2:
  Input: s = "hi(name)", knowledge = [["a","b"]]
  Output: "hi?"
  Explanation: As you do not know the value of the key "name", replace "(name)" with "?".
  
  Example 3:
  Input: s = "(a)(a)(a)aaa", knowledge = [["a","yes"]]
  Output: "yesyesyesaaa"
  Explanation: The same key can appear multiple times.
  The key "a" has a value of "yes", so replace all occurrences of "(a)" with "yes".
  Notice that the "a"s not in a bracket pair are not evaluated.
*/
use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let values: HashMap<String, String> = knowledge
            .into_iter()
            .map(|p| {
                let mut it = p.into_iter();
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect();

        let mut buf = s.into_bytes();
        let n = buf.len();

        let (mut r, mut w, mut grow) = (0usize, 0usize, 0usize);
        while r < n {
            if buf[r] != b'(' {
                buf[w] = buf[r];
                w += 1;
                r += 1;
                continue;
            }
            let close = r + 1 + buf[r + 1..].iter().position(|&b| b == b')').unwrap();
            let ph = close + 1 - r;
            let key = unsafe { std::str::from_utf8_unchecked(&buf[r + 1..close]) };
            match values.get(key) {
                Some(v) if v.len() <= ph => {
                    buf[w..w + v.len()].copy_from_slice(v.as_bytes());
                    w += v.len();
                }
                Some(v) => {
                    grow += v.len() - ph;
                    buf.copy_within(r..=close, w);
                    w += ph;
                }
                None => {
                    buf[w] = b'?';
                    w += 1;
                }
            }
            r = close + 1;
        }
        buf.truncate(w);

        if grow > 0 {
            let m = w;
            let total = m + grow;
            buf.resize(total, 0);
            let (mut r, mut w) = (m, total);
            while r > 0 {
                r -= 1;
                if buf[r] != b')' {
                    w -= 1;
                    buf[w] = buf[r];
                    continue;
                }
                let open = buf[..r].iter().rposition(|&b| b == b'(').unwrap();
                let key = unsafe { std::str::from_utf8_unchecked(&buf[open + 1..r]) };
                let v = values[key].as_bytes();
                w -= v.len();
                buf[w..w + v.len()].copy_from_slice(v);
                r = open;
            }
        }

        unsafe { String::from_utf8_unchecked(buf) }
    }
}
