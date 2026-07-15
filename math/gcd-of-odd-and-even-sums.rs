/*
  3658. GCD of Odd and Even Sums
  
  You are given an integer n. Your task is to compute the GCD (greatest common divisor) of two values:
      sumOdd: the sum of the smallest n positive odd numbers.
      sumEven: the sum of the smallest n positive even numbers.
  
  Return the GCD of sumOdd and sumEven.
  
  Example 1:
  Input: n = 4
  Output: 4
  Explanation:
      Sum of the first 4 odd numbers sumOdd = 1 + 3 + 5 + 7 = 16
      Sum of the first 4 even numbers sumEven = 2 + 4 + 6 + 8 = 20
  Hence, GCD(sumOdd, sumEven) = GCD(16, 20) = 4.
  
  Example 2:
  Input: n = 5
  Output: 5
  Explanation:
      Sum of the first 5 odd numbers sumOdd = 1 + 3 + 5 + 7 + 9 = 25
      Sum of the first 5 even numbers sumEven = 2 + 4 + 6 + 8 + 10 = 30
  Hence, GCD(sumOdd, sumEven) = GCD(25, 30) = 5.                    
*/
impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let mut sum_odd = 0;
        let mut sum_even = 0;
        let mut num = 1;
        let mut cnt_odd = 0;
        let mut cnt_even = 0;
        loop {
            if cnt_odd == n && cnt_even == n {
                break;
            }

            if num % 2 == 0 {
                sum_even += num;
                cnt_even += 1;
            } else {
                sum_odd += num;
                cnt_odd += 1;
            }

            num += 1;
        }

        while sum_odd != 0 {
            let temp = sum_odd;
            sum_odd = sum_even % sum_odd;
            sum_even = temp;
        }

        sum_even
    }
}
