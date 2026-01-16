/*
  2975. Maximum Square Area by Removing Fences From a Field
  
  There is a large (m - 1) x (n - 1) rectangular field with corners at (1, 1) and (m, n) containing some horizontal 
  and vertical fences given in arrays hFences and vFences respectively.
  Horizontal fences are from the coordinates (hFences[i], 1) to (hFences[i], n) and vertical fences are from the coordinates (1, vFences[i]) to (m, vFences[i]).
  
  Return the maximum area of a square field that can be formed by removing some fences (possibly none) or -1 if it is impossible to make a square field.
  
  Since the answer may be large, return it modulo 109 + 7.
  Note: The field is surrounded by two horizontal fences from the coordinates (1, 1) to (1, n) and (m, 1) to (m, n) 
  and two vertical fences from the coordinates (1, 1) to (m, 1) and (1, n) to (m, n). These fences cannot be removed.
  
  Example 1:
  Input: m = 4, n = 3, hFences = [2,3], vFences = [2]
  Output: 4
  Explanation: Removing the horizontal fence at 2 and the vertical fence at 2 will give a square field of area 4.
  
  Example 2:
  Input: m = 6, n = 7, hFences = [2], vFences = [4]
  Output: -1
  Explanation: It can be proved that there is no way to create a square field by removing fences.
*/
use std::collections::HashSet;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
        h_fences.push(1);
        h_fences.push(m);
        h_fences.sort_unstable();

        v_fences.push(1);
        v_fences.push(n);
        v_fences.sort_unstable();

        let (short, long) = if h_fences.len() < v_fences.len() {
            (&h_fences, &v_fences)
        } else {
            (&v_fences, &h_fences)
        };

        let n = short.len();
        let mut data = HashSet::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in i + 1..n {
                data.insert(short[j] - short[i]);
            }
        }

        let mut res = -1i64;
        let m = long.len();

        for i in 0..m {
            for j in (i + 1..m).rev() {
                let diff = long[j] - long[i];
                if diff as i64 <= res {
                    break;
                }

                if data.contains(&diff) {
                    res = diff as i64;
                    break;
                }
            }
        }

        if res == -1 {
            -1
        } else {
            ((res * res) % MOD) as i32
        }
    }
}
