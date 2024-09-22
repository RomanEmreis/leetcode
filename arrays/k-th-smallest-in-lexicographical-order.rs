/*
  Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].
  
  Example 1:
    Input: n = 13, k = 2
    Output: 10
    Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.

  Example 2:
    Input: n = 1, k = 1
    Output: 1
*/
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let (mut curr, n, mut k) = (1i64, n as i64, k as i64);
        k -= 1;

        while k > 0 {
            let (mut steps, mut first, mut last) = (0 as i64, curr, curr);
            while first <= n {
                if last < n { steps += last - first + 1; }
                else { steps += n - first + 1; }
                first *= 10;
                last = last * 10 + 9;
            }

            if steps <= k { 
                curr += 1;
                k -= steps;
            } else {
                curr *= 10;
                k -= 1;
            }
        }

        curr as i32
    }
}
