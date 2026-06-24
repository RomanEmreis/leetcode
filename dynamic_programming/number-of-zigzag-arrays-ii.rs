/*
  3700. Number of ZigZag Arrays II
  
  You are given three integers n, l, and r.
  A ZigZag array of length n is defined as follows:
      Each element lies in the range [l, r].
      No two adjacent elements are equal.
      No three consecutive elements form a strictly increasing or strictly decreasing sequence.
  
  Return the total number of valid ZigZag arrays.
  
  Since the answer may be large, return it modulo 109 + 7.
  - A sequence is said to be strictly increasing if each element is strictly greater than its previous one (if exists).
  - A sequence is said to be strictly decreasing if each element is strictly smaller than its previous one (if exists)
  
  Example 1:
  Input: n = 3, l = 4, r = 5
  Output: 2
  Explanation:
  There are only 2 valid ZigZag arrays of length n = 3 using values in the range [4, 5]:
      [4, 5, 4]
      [5, 4, 5]
  
  Example 2:
  Input: n = 3, l = 1, r = 3
  Output: 10
  Explanation:
  ​​​​​​​There are 10 valid ZigZag arrays of length n = 3 using values in the range [1, 3]:
      [1, 2, 1], [1, 3, 1], [1, 3, 2]
      [2, 1, 2], [2, 1, 3], [2, 3, 1], [2, 3, 2]
      [3, 1, 2], [3, 1, 3], [3, 2, 3]
  
  All arrays meet the ZigZag conditions.                
*/
const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let m = (r - l + 1) as usize;
        let n = n as u64;

        let mut a = vec![vec![0u64; m]; m];
        for v in 0..m {
            for k in (m - v)..m {
                a[v][k] = 1;
            }
        }

        let p = Self::mat_pow(a, n - 1, m);
        let sum_up: u64 = p
            .iter()
            .map(|row| row.iter().fold(0u64, |acc, &x| (acc + x) % MOD))
            .fold(0u64, |acc, x| (acc + x) % MOD);
        ((2 * sum_up) % MOD) as i32
    }

    fn mat_mul(a: &[Vec<u64>], b: &[Vec<u64>], m: usize) -> Vec<Vec<u64>> {
        let mut c = vec![vec![0u64; m]; m];
        for i in 0..m {
            for j in 0..m {
                let mut acc: u128 = 0;
                for k in 0..m {
                    acc += u128::from(a[i][k]) * u128::from(b[k][j]);
                }
                c[i][j] = (acc % u128::from(MOD)) as u64;
            }
        }
        c
    }

    fn mat_pow(mut base: Vec<Vec<u64>>, mut e: u64, m: usize) -> Vec<Vec<u64>> {
        let mut res = vec![vec![0u64; m]; m];
        for i in 0..m {
            res[i][i] = 1;
        }
        while e > 0 {
            if e & 1 == 1 {
                res = Self::mat_mul(&res, &base, m);
            }
            base = Self::mat_mul(&base, &base, m);
            e >>= 1;
        }
        res
    }
}
