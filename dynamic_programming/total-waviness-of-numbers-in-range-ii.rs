/*
  3753. Total Waviness of Numbers in Range II
  
  You are given two integers num1 and num2 representing an inclusive range [num1, num2].
  
  The waviness of a number is defined as the total count of its peaks and valleys:
  
      A digit is a peak if it is strictly greater than both of its immediate neighbors.
      A digit is a valley if it is strictly less than both of its immediate neighbors.
      The first and last digits of a number cannot be peaks or valleys.
      Any number with fewer than 3 digits has a waviness of 0.
  
  Return the total sum of waviness for all numbers in the range [num1, num2].
  
  Example 1:
  Input: num1 = 120, num2 = 130
  Output: 3
  Explanation:
  In the range [120, 130]:
      120: middle digit 2 is a peak, waviness = 1.
      121: middle digit 2 is a peak, waviness = 1.
      130: middle digit 3 is a peak, waviness = 1.
      All other numbers in the range have a waviness of 0.
  
  Thus, total waviness is 1 + 1 + 1 = 3.
  
  Example 2:
  Input: num1 = 198, num2 = 202
  Output: 3
  Explanation:
  In the range [198, 202]:
      198: middle digit 9 is a peak, waviness = 1.
      201: middle digit 0 is a valley, waviness = 1.
      202: middle digit 0 is a valley, waviness = 1.
      All other numbers in the range have a waviness of 0.
  
  Thus, total waviness is 1 + 1 + 1 = 3.
  
  Example 3:
  Input: num1 = 4848, num2 = 4848
  Output: 2
  Explanation:
  Number 4848: the second digit 8 is a peak, and the third digit 4 is a valley, giving a waviness of 2.  
*/
use std::sync::OnceLock;

type Memo = [[[(i64, i64); 11]; 11]; 20];

fn memo() -> &'static Memo {
    static M: OnceLock<Memo> = OnceLock::new();
    M.get_or_init(|| {
        let mut m = [[[(0i64, 0i64); 11]; 11]; 20];
        for p2 in 0..11 {
            for p1 in 0..11 {
                m[0][p2][p1] = (1, 0);
            }
        }
        for r in 1..20 {
            for p2i in 0..11 {
                for p1i in 0..11 {
                    let p2 = p2i as i64 - 1;
                    let p1 = p1i as i64 - 1;
                    let mut cnt = 0i64;
                    let mut wav = 0i64;
                    for d in 0..=9i64 {
                        let wavy = i64::from(
                            p2 >= 0 && p1 >= 0
                                && ((d > p1 && p1 < p2) || (d < p1 && p1 > p2)),
                        );
                        let (c, w) = m[r - 1][p1i][(d + 1) as usize];
                        cnt += c;
                        wav += wavy * c + w;
                    }
                    m[r][p2i][p1i] = (cnt, wav);
                }
            }
        }
        m
    })
}

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        let m = memo();
        Self::count(num2, m) - Self::count(num1 - 1, m)
    }

    fn count(n: i64, m: &Memo) -> i64 {
        if n < 100 {
            return 0;
        }
        
        let mut buf = [0i64; 19];
        let mut len = 0usize;
        let mut tmp = n;
        while tmp > 0 {
            buf[len] = tmp % 10;
            len += 1;
            tmp /= 10;
        }
        buf[..len].reverse();
        let digits = &buf[..len];

        let mut total = 0i64;
        
        for l in 3..len {
            for d in 1..=9usize {
                total += m[l - 1][0][d + 1].1;
            }
        }
        total += Self::digit_dp(digits, m);
        total
    }

    fn digit_dp(digits: &[i64], m: &Memo) -> i64 {
        let l = digits.len();
        let mut wav = 0i64;
        let mut prev2 = -1i64;
        let mut prev1 = -1i64;
        let mut prefix_wav = 0i64;
        for pos in 0..l {
            let lo = if pos == 0 { 1 } else { 0 };
            let hi = digits[pos];
            let rem = l - pos - 1;
            for d in lo..hi {
                let wavy = i64::from(
                    prev2 >= 0 && prev1 >= 0
                        && ((d > prev1 && prev1 < prev2) || (d < prev1 && prev1 > prev2)),
                );
                let (c, w) = m[rem][(prev1 + 1) as usize][(d + 1) as usize];
                wav += (prefix_wav + wavy) * c + w;
            }
            let d = digits[pos];
            let wavy = i64::from(
                prev2 >= 0 && prev1 >= 0
                    && ((d > prev1 && prev1 < prev2) || (d < prev1 && prev1 > prev2)),
            );
            prefix_wav += wavy;
            prev2 = prev1;
            prev1 = d;
        }
        wav + prefix_wav
    }
}
