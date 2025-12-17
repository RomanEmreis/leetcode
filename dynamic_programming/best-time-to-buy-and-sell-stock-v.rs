/*
  3573. Best Time to Buy and Sell Stock V
  
  You are given an integer array prices where prices[i] is the price of a stock in dollars on the ith day, and an integer k.
  You are allowed to make at most k transactions, where each transaction can be either of the following:
      Normal transaction: Buy on day i, then sell on a later day j where i < j. You profit prices[j] - prices[i].
      Short selling transaction: Sell on day i, then buy back on a later day j where i < j. You profit prices[i] - prices[j].
  
  Note that you must complete each transaction before starting another. Additionally, you can't buy or sell on the same day you are selling or buying back as part of a previous transaction.
  Return the maximum total profit you can earn by making at most k transactions.
  
  Example 1:
  Input: prices = [1,7,9,8,2], k = 2
  Output: 14
  Explanation:
  We can make $14 of profit through 2 transactions:
      A normal transaction: buy the stock on day 0 for $1 then sell it on day 2 for $9.
      A short selling transaction: sell the stock on day 3 for $8 then buy back on day 4 for $2.
  
  Example 2:
  Input: prices = [12,16,19,19,8,1,19,13,9], k = 3
  Output: 36
  Explanation:
  We can make $36 of profit through 3 transactions:
      A normal transaction: buy the stock on day 0 for $12 then sell it on day 2 for $19.
      A short selling transaction: sell the stock on day 3 for $19 then buy back on day 4 for $8.
      A normal transaction: buy the stock on day 5 for $1 then sell it on day 6 for $19.
*/
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;
        let mut dp: Vec<[i64; 3]> = vec![[0; 3]; k + 1];

        let price = prices[0] as i64;
        for i in 1..=k {    
            dp[i][1] = -price;
            dp[i][2] = price;
        }

        for i in 1..n {
            let price = prices[i] as i64;
            for j in (1..=k).rev() {
                dp[j][0] = dp[j][0].max(
                    (dp[j][1] + price).max(dp[j][2] - price)
                );

                dp[j][1] = dp[j][1].max(dp[j - 1][0] - price);
                dp[j][2] = dp[j][2].max(dp[j - 1][0] + price);
            }
        }

        dp[k][0]
    }
}
