/*
  3754. Concatenate Non-Zero Digits and Multiply by Sum I
  
  You are given an integer n.
  Form a new integer x by concatenating all the non-zero digits of n in their original order. If there are no non-zero digits, x = 0.
  
  Let sum be the sum of digits in x.
  
  Return an integer representing the value of x * sum.
  
  Example 1:
  Input: n = 10203004
  Output: 12340
  Explanation:
      The non-zero digits are 1, 2, 3, and 4. Thus, x = 1234.
      The sum of digits is sum = 1 + 2 + 3 + 4 = 10.
      Therefore, the answer is x * sum = 1234 * 10 = 12340.
  
  Example 2:
  Input: n = 1000
  Output: 1
  Explanation:
      The non-zero digit is 1, so x = 1 and sum = 1.
      Therefore, the answer is x * sum = 1 * 1 = 1.
*/
impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut sum: i64 = 0;
        let mut num: i64 = 0;
        let mut p: i64 = 1;

        while n > 0 {
            let d = n % 10;

            if d != 0 {
                let d = d as i64;
                
                sum += d;
                num += d * p;
                p *= 10;
            }

            n /= 10;
        }

        num * sum
    }
}
