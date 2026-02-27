/*
  3666. Minimum Operations to Equalize Binary String
  
  You are given a binary string s, and an integer k.
  In one operation, you must choose exactly k different indices and flip each '0' to '1' and each '1' to '0'.
  
  Return the minimum number of operations required to make all characters in the string equal to '1'. If it is not possible, return -1.
  
  Example 1:
  Input: s = "110", k = 1
  Output: 1
  Explanation:
      There is one '0' in s.
      Since k = 1, we can flip it directly in one operation.
  
  Example 2:
  Input: s = "0101", k = 3
  Output: 2
  Explanation:
  One optimal set of operations choosing k = 3 indices in each operation is:
      Operation 1: Flip indices [0, 1, 3]. s changes from "0101" to "1000".
      Operation 2: Flip indices [1, 2, 3]. s changes from "1000" to "1111".
  
  Thus, the minimum number of operations is 2.
  
  Example 3:
  Input: s = "101", k = 2
  Output: -1
  Explanation:
  Since k = 2 and s has only one '0', it is impossible to flip exactly k indices to make all '1'. Hence, the answer is -1.
*/
impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        let zeros = s
            .into_bytes()
            .into_iter()
            .filter(|ch| *ch == b'0')
            .count();

        if zeros == 0 {
            return 0;
        } else if zeros == k {
            return 1;
        } else if k == 1 {
            return zeros as i32;
        }

        if k == n {
            if zeros == 0 {
                return 0;
            } else if zeros == k {
                return 1;
            } else {
                return -1;
            }
        }

        let mut res = i64::MAX;

        if zeros % 2 == 0 {
            let c1 = ceil(zeros, k);
            let c2 = ceil(zeros, n - k);
            let mut max = c1.max(c2);

            if max % 2 == 1 {
                max += 1;
            }

            res = res.min(max);
        }

        if zeros % 2 == k % 2 {
            let c1 = ceil(zeros, k);
            let c2 = ceil(n - zeros, n - k);
            let mut max = c1.max(c2);

            if max % 2 == 0 {
                max += 1;
            }

            res = res.min(max);
        }

        if res == i64::MAX {
            -1
        } else {
            res as i32
        }
    }
}

#[inline(always)]
fn ceil(a: usize, k: usize) -> i64 {
    ((a + k - 1) / k) as i64
}
