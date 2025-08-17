/*
  837. New 21 Game
  
  Alice plays the following game, loosely based on the card game "21".
  Alice starts with 0 points and draws numbers while she has less than k points. During each draw, she gains an integer number of points randomly from the range [1, maxPts], where maxPts is an integer. Each draw is independent and the outcomes have equal probabilities.
  Alice stops drawing numbers when she gets k or more points.
  
  Return the probability that Alice has n or fewer points.
  Answers within 10-5 of the actual answer are considered accepted.
  
  Example 1:
  Input: n = 10, k = 1, maxPts = 10
  Output: 1.00000
  Explanation: Alice gets a single card, then stops.
  
  Example 2:
  Input: n = 6, k = 1, maxPts = 10
  Output: 0.60000
  Explanation: Alice gets a single card, then stops.
  In 6 out of 10 possibilities, she is at or below 6 points.
  
  Example 3:
  Input: n = 21, k = 17, maxPts = 10
  Output: 0.73278
*/
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }

        let mut dp = vec![0f64; (k + max_pts) as usize];
        let m = std::cmp::min(n + 1, k + max_pts) as usize;
        for i in k as usize..m {
            dp[i] = 1.0;
        }
        
        dp[k as usize - 1] = std::cmp::min(n - k + 1, max_pts) as f64 / max_pts as f64;

        let m = k - 2;
        if m >= 0 {
            for i in (0..=m as usize).rev() {
                dp[i] = dp[i + 1] + (dp[i + 1] - dp[i + max_pts as usize + 1]) / max_pts as f64;
            }
        } 

        dp[0]
    }
}
