/*
  67. Add Binary
  
  Given two binary strings a and b, return their sum as a binary string.
  
  Example 1:
  Input: a = "11", b = "1"
  Output: "100"
  
  Example 2:
  Input: a = "1010", b = "1011"
  Output: "10101"
*/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut i = (a.len() - 1) as isize;
        let mut j = (b.len() - 1) as isize;

        let mut s = String::with_capacity(a.len().max(b.len()));
        let mut carry = false;

        let a = a.into_bytes();
        let b = b.into_bytes();

        while i.max(j) >= 0 {
            let l = if i >= 0 {
                a[i as usize]
            } else {
                b'0'
            };
            let r = if j >= 0 {
                b[j as usize]
            } else {
                b'0'
            };

            if l == b'0' && r == b'0' {
                s.insert(0, if carry { '1' } else { '0' });
                carry = false;
            } else if (l == b'1' && r == b'0') || (l == b'0' && r == b'1') {
                s.insert(0, if carry { '0' } else { '1' });
            } else if l == b'1' && r == b'1' {
                s.insert(0, if carry { '1' } else { '0' });
                carry = true;
            }

            i -= 1;
            j -= 1;
        }
        if carry {
            s.insert(0, '1');
        }
        s
    }
}
