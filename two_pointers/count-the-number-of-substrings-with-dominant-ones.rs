/*
  3234. Count the Number of Substrings With Dominant Ones
  
  You are given a binary string s.
  
  Return the number of substrings with dominant ones.
  
  A string has dominant ones if the number of ones in the string is greater than or equal to the square of the number of zeros in the string.
  
  Example 1:
  Input: s = "00011"
  Output: 5
  
  Example 2:
  Input: s = "101101"
  Output: 16
*/
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let b = s.as_bytes();
        let mut zero_idx = Vec::with_capacity(n);
        for i in 0..n {
            if b[i] == b'0' {
                zero_idx.push(i);
            }
        }

        let mut res = 0;
        let mut j = 0;

        let z = zero_idx.len();
        for i in 0..n {
            while j < z && zero_idx[j] < i {
                j += 1;
            }
            if j == z {
                res += n - i;
                continue;
            }
            res += zero_idx[j] - i;
            for k in j..z {
                let zeros = k - j + 1;
                let ones = zeros * zeros;
                let min_index = i + zeros + ones - 1;
                if n <= min_index {
                    break;
                }
                let max_index = if k + 1 < z {
                    zero_idx[k + 1]
                } else {
                    n
                };

                if min_index < max_index {
                    res += max_index - zero_idx[k].max(min_index);
                }
            }
        }

        res as i32
    }
}
