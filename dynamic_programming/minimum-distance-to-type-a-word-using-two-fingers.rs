/*
  1320. Minimum Distance to Type a Word Using Two Fingers
  
  You have a keyboard layout as shown above in the X-Y plane, where each English uppercase letter is located at some coordinate.
      For example, the letter 'A' is located at coordinate (0, 0), the letter 'B' is located at coordinate (0, 1), the letter 'P' is located at coordinate (2, 3) and the letter 'Z' is located at coordinate (4, 1).
  
  Given the string word, return the minimum total distance to type such string using only two fingers.
  
  The distance between coordinates (x1, y1) and (x2, y2) is |x1 - x2| + |y1 - y2|.
  
  Note that the initial positions of your two fingers are considered free so do not count towards your total distance, also your two fingers do not have to start at the first letter or the first two letters.
  
  Example 1:
  Input: word = "CAKE"
  Output: 3
  Explanation: Using two fingers, one optimal way to type "CAKE" is: 
  Finger 1 on letter 'C' -> cost = 0 
  Finger 1 on letter 'A' -> cost = Distance from letter 'C' to letter 'A' = 2 
  Finger 2 on letter 'K' -> cost = 0 
  Finger 2 on letter 'E' -> cost = Distance from letter 'K' to letter 'E' = 1 
  Total distance = 3
  
  Example 2:
  Input: word = "HAPPY"
  Output: 6
  Explanation: Using two fingers, one optimal way to type "HAPPY" is:
  Finger 1 on letter 'H' -> cost = 0
  Finger 1 on letter 'A' -> cost = Distance from letter 'H' to letter 'A' = 2
  Finger 2 on letter 'P' -> cost = 0
  Finger 2 on letter 'P' -> cost = Distance from letter 'P' to letter 'P' = 0
  Finger 1 on letter 'Y' -> cost = Distance from letter 'A' to letter 'Y' = 4
  Total distance = 6
*/
const FREE: usize = 26;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word = word.into_bytes();
        let n = word.len();

        let mut dp = [i32::MAX; 27];
        dp[FREE] = 0;

        for i in 0..n - 1 {
            let cur = (word[i] - b'A') as usize;
            let nxt = (word[i + 1] - b'A') as usize;
            let mut ndp = [i32::MAX; 27];

            for j in 0..=26 {
                if dp[j] == i32::MAX {
                    continue;
                }
                let cost = dp[j];

                let move_active = cost + dist(cur, nxt);
                if move_active < ndp[j] {
                    ndp[j] = move_active;
                }

                let move_passive = cost + if j == FREE { 0 } else { dist(j, nxt) };
                if move_passive < ndp[cur] {
                    ndp[cur] = move_passive;
                }
            }

            dp = ndp;
        }

        *dp.iter().min().unwrap()
    }
}

#[inline(always)]
fn dist(a: usize, b: usize) -> i32 {
    let (ra, ca) = (a / 6, a % 6);
    let (rb, cb) = (b / 6, b % 6);
    (ra as i32 - rb as i32).abs() + (ca as i32 - cb as i32).abs()
}
