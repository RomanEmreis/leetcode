/*
  3345. Smallest Divisible Digit Product I
  
  You are given two integers n and t. Return the smallest number greater than or equal to n such that the product of its digits is divisible by t.
  
  Example 1:
  Input: n = 10, t = 2
  Output: 10
  Explanation:
  The digit product of 10 is 0, which is divisible by 2, making it the smallest number greater than or equal to 10 that satisfies the condition.
  
  Example 2:
  Input: n = 15, t = 3
  Output: 16
  Explanation:
  The digit product of 16 is 6, which is divisible by 3, making it the smallest number greater than or equal to 15 that satisfies the condition.
*/
impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        if t == 1 {
            return n;
        }

        let limit = ((n + 9) / 10) * 10;

        for res in n..=limit {
            let product = if res < 10 {
                res
            } else {
                (res / 10) * (res % 10)
            };

            if product % t == 0 {
                return res;
            }
        }

        -1
    }
}
