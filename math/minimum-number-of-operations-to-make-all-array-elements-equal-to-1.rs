/*
  2654. Minimum Number of Operations to Make All Array Elements Equal to 1
  
  You are given a 0-indexed array nums consisiting of positive integers. You can do the following operation on the array any number of times:
      Select an index i such that 0 <= i < n - 1 and replace either of nums[i] or nums[i+1] with their gcd value.
  
  Return the minimum number of operations to make all elements of nums equal to 1. If it is impossible, return -1.
  
  The gcd of two integers is the greatest common divisor of the two integers.
  
  Example 1:
  Input: nums = [2,6,3,4]
  Output: 4
  Explanation: We can do the following operations:
  - Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
  - Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
  - Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
  - Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].
  
  Example 2:
  Input: nums = [2,10,6,14]
  Output: -1
  Explanation: It can be shown that it is impossible to make all the elements equal to 1.
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let ones = nums.iter().filter(|&n| *n == 1).count();
        let n = nums.len();
        if ones > 0 {
            return (n - ones) as i32;
        }
        
        let mut ops = i32::MAX;

        for i in 0..n {
            let mut g = nums[i];
            for j in i + 1..n {
                g = gcd(g, nums[j]);
                if g == 1 {
                    ops = ops.min((j - i) as i32);
                    break;
                }
            }
        }

        if ops == i32::MAX {
            -1
        } else {
            n as i32 + ops - 1
        }
    }
}

#[inline(always)]
fn gcd(mut a: i32, mut b:i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
