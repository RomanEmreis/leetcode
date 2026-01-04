/*
  1390. Four Divisors
  
  Given an integer array nums, return the sum of divisors of the integers in that array that have exactly four divisors. 
  If there is no such integer in the array, return 0.
  
  Example 1:
  Input: nums = [21,4,7]
  Output: 32
  Explanation: 
  21 has 4 divisors: 1, 3, 7, 21
  4 has 3 divisors: 1, 2, 4
  7 has 2 divisors: 1, 7
  The answer is the sum of divisors of 21 only.
  
  Example 2:
  Input: nums = [21,21]
  Output: 64
  
  Example 3:
  Input: nums = [1,2,3,4,5]
  Output: 0
*/
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums.into_iter() {
            let mut div = 0;
            for i in 2..=num.isqrt() {
                if num % i == 0 {
                    if div == 0 {
                        div = i;
                    } else {
                        div = 0;
                        break;
                    }
                }
            }
            if div > 0 && div * div < num {
                res += 1 + div + num + num / div;
            }
        }
        res
    }
}
