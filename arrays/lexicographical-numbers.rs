/*
  Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
  
  You must write an algorithm that runs in O(n) time and uses O(1) extra space. 
  
  Example 1:
  Input: n = 13
  Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
*/
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut curr = 1;

        for i in 1..=n {
            result[(i - 1) as usize] = curr;
            if curr * 10 <= n {
                curr *= 10;
            } else {
                if curr >= n {
                    curr /= 10;
                }
                curr += 1;
                while curr % 10 == 0 {
                    curr /= 10;
                }
            }
        }

        result
    }
}
