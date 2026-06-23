/*
  3699. Number of ZigZag Arrays I
  
  You are given three integers n, l, and r.
  A ZigZag array of length n is defined as follows:
      Each element lies in the range [l, r].
      No two adjacent elements are equal.
      No three consecutive elements form a strictly increasing or strictly decreasing sequence.
  
  Return the total number of valid ZigZag arrays.
  
  Since the answer may be large, return it modulo 109 + 7.
  A sequence is said to be strictly increasing if each element is strictly greater than its previous one (if exists).
  A sequence is said to be strictly decreasing if each element is strictly smaller than its previous one (if exists).
  
  Example 1:
  Input: n = 3, l = 4, r = 5
  Output: 2
  Explanation:
  There are only 2 valid ZigZag arrays of length n = 3 using values in the range [4, 5]:
      [4, 5, 4]
      [5, 4, 5]​​​​​​​
  
  Example 2:
  Input: n = 3, l = 1, r = 3
  Output: 10
  Explanation:
  There are 10 valid ZigZag arrays of length n = 3 using values in the range [1, 3]:
      [1, 2, 1], [1, 3, 1], [1, 3, 2]
      [2, 1, 2], [2, 1, 3], [2, 3, 1], [2, 3, 2]
      [3, 1, 2], [3, 1, 3], [3, 2, 3]
  
  All arrays meet the ZigZag conditions.
*/
impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let m = (r - l + 1) as usize;
        let n = n as usize;

        let mut up = vec![1u64; m];
        let mut down = vec![1u64; m];

        let mut pre_up = vec![0u64; m + 1];
        let mut pre_down = vec![0u64; m + 1];

        for _ in 1..n {
            for v in 0..m {
                pre_up[v + 1] = pre_up[v] + up[v];
                pre_down[v + 1] = pre_down[v] + down[v];
            }
            let total_up = pre_up[m];

            for v in 0..m {
                up[v] = pre_down[v] % MOD;
                down[v] = (total_up - pre_up[v + 1]) % MOD;
            }
        }

        let mut res = 0u64;
        for v in 0..m {
            res += up[v] + down[v];
        }

        (res % MOD) as i32
    }
}
