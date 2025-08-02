/*
  2561. Rearranging Fruits
  
  You have two fruit baskets containing n fruits each. You are given two 0-indexed integer arrays basket1 and basket2 representing the cost of fruit in each basket. 
  You want to make both baskets equal. To do so, you can use the following operation as many times as you want:
      Chose two indices i and j, and swap the ith fruit of basket1 with the jth fruit of basket2.
      The cost of the swap is min(basket1[i],basket2[j]).
  
  Two baskets are considered equal if sorting them according to the fruit cost makes them exactly the same baskets.
  
  Return the minimum cost to make both the baskets equal or -1 if impossible.
  
  Example 1:
  Input: basket1 = [4,2,2,2], basket2 = [1,4,1,2]
  Output: 1
  Explanation: Swap index 1 of basket1 with index 0 of basket2, which has cost 1. Now basket1 = [4,1,2,2] and basket2 = [2,4,1,2]. Rearranging both the arrays makes them equal.
  
  Example 2:
  Input: basket1 = [2,3,4,1], basket2 = [3,2,5,1]
  Output: -1
  Explanation: It can be shown that it is impossible to make both the baskets equal.
*/
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let n = basket1.len();
        let mut freq: HashMap<i32, i32> = HashMap::with_capacity(n);
        for i in 0..n {
            *freq.entry(basket1[i]).or_insert(0) += 1;
            *freq.entry(basket2[i]).or_insert(0) -= 1;
        }

        let mut min = i32::MAX;
        let mut nums: Vec<i32> = Vec::with_capacity(n * 2);
        for (x, v) in freq {
            if v % 2 != 0 {
                return -1;
            }
            for _ in 0..(v.abs() / 2) {
                nums.push(x);
            }
            min = min.min(x);
        } 

        nums.sort();

        let m = nums.len();
        let mut result: i64 = 0;
        for i in 0..(m / 2) {
            result += nums[i].min(min * 2) as i64;
        }
        result
    }
}
