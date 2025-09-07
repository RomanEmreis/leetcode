/*
  1304. Find N Unique Integers Sum up to Zero
  
  Given an integer n, return any array containing n unique integers such that they add up to 0.
  
  Example 1:
  Input: n = 5
  Output: [-7,-1,1,3,4]
  Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
  
  Example 2:
  Input: n = 3
  Output: [-1,0,1]
  
  Example 3:
  Input: n = 1
  Output: [0]
*/
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0i32; n];
        let n = if n % 2 == 0 { n } else { n - 1 };
        
        let mut i: usize = 0;
        let mut j: i32 = 1;
        while i < n {
            res[i] = j;
            i += 1;
            res[i] = -j;
            i += 1;
            j += 1;
        }
        res
    }
}
