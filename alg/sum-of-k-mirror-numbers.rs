/*
  2081. Sum of k-Mirror Numbers
  
  A k-mirror number is a positive integer without leading zeros that reads the same both forward and backward in base-10 as well as in base-k.
  
  For example, 9 is a 2-mirror number. The representation of 9 in base-10 and base-2 are 9 and 1001 respectively, which read the same both forward and backward.
  On the contrary, 4 is not a 2-mirror number. The representation of 4 in base-2 is 100, which does not read the same both forward and backward.
  Given the base k and the number n, return the sum of the n smallest k-mirror numbers.
  
  Example 1:
  Input: k = 2, n = 5
  Output: 25
  Explanation:
  The 5 smallest 2-mirror numbers and their representations in base-2 are listed as follows:
    base-10    base-2
      1          1
      3          11
      5          101
      7          111
      9          1001
  Their sum = 1 + 3 + 5 + 7 + 9 = 25. 
  
  Example 2:
  Input: k = 3, n = 7
  Output: 499
  Explanation:
  The 7 smallest 3-mirror numbers are and their representations in base-3 are listed as follows:
    base-10    base-3
      1          1
      2          2
      4          11
      8          22
      121        11111
      151        12121
      212        21212
  Their sum = 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499.
  
  Example 3:
  Input: k = 7, n = 17
  Output: 20379000
  Explanation: The 17 smallest 7-mirror numbers are:
  1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596
*/
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut digit = [0; 100];
        let mut left = 1;
        let mut count = 0;
        let mut result = 0;
        let mut is_palindrome = |mut x: i64, k: i64| {
            let mut ln: i32 = -1;
            while x > 0 {
                ln += 1;
                digit[ln as usize] = (x % k) as i32;
                x /= k;
            }
            let mut i = 0;
            let mut j = ln as usize;
            while i < j {
                if digit[i] != digit[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        };
        while count < n {
            let mut right = left * 10;
            for op in 0..2 {
                let mut i = left;
                while i < right && count < n {
                    let mut c = i;
                    let mut x = if op == 0 { i / 10 } else { i };
                    while x > 0 {
                        c = c * 10 + x % 10;
                        x /= 10;
                    }
                    if is_palindrome(c, k as i64) {
                        count += 1;
                        result += c;
                    }
                    i += 1;
                }
            }
            left = right;
        }
        result
    }
}
