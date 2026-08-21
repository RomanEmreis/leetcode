/*
  3116. Kth Smallest Amount With Single Denomination Combination
  
  You are given an integer array coins representing coins of different denominations and an integer k.
  You have an infinite number of coins of each denomination. However, you are not allowed to combine coins of different denominations.
  
  Return the kth smallest amount that can be made using these coins.
  
  Example 1:
  Input: coins = [3,6,9], k = 3
  Output: 9
  Explanation: The given coins can make the following amounts:
  Coin 3 produces multiples of 3: 3, 6, 9, 12, 15, etc.
  Coin 6 produces multiples of 6: 6, 12, 18, 24, etc.
  Coin 9 produces multiples of 9: 9, 18, 27, 36, etc.
  All of the coins combined produce: 3, 6, 9, 12, 15, etc.
  
  Example 2:
  Input: coins = [5,2], k = 7
  Output: 12
  Explanation: The given coins can make the following amounts:
  Coin 5 produces multiples of 5: 5, 10, 15, 20, etc.
  Coin 2 produces multiples of 2: 2, 4, 6, 8, 10, 12, etc.
  All of the coins combined produce: 2, 4, 5, 6, 8, 10, 12, 14, 15, etc.
*/
impl Solution {
    pub fn find_kth_smallest(mut coins: Vec<i32>, k: i32) -> i64 {
        coins.sort_unstable();

        let mut length = 0usize;

        for read in 0..coins.len() {
            let coin = coins[read];
            let mut redundant = false;

            for previous in 0..length {
                if coin % coins[previous] == 0 {
                    redundant = true;
                    break;
                }
            }

            if !redundant {
                coins[length] = coin;
                length += 1;
            }
        }

        coins.truncate(length);

        if coins[0] == 1 {
            return k as i64;
        }

        let target = k as i64;
        let mut left = 1i64;
        let mut right = coins[0] as i64 * target;

        while left < right {
            let middle = left + (right - left) / 2;
            let count = Self::count_subsets(
                &coins,
                0,
                1,
                1,
                middle,
            );

            if count >= target {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left
    }

    fn count_subsets(
        coins: &[i32],
        start: usize,
        current_lcm: i64,
        sign: i64,
        limit: i64,
    ) -> i64 {
        let mut total = 0i64;

        for index in start..coins.len() {
            let coin = coins[index] as i64;
            let quotient =
                current_lcm / Self::gcd(current_lcm, coin);

            if quotient > limit / coin {
                continue;
            }

            let next_lcm = quotient * coin;

            total += sign * (limit / next_lcm);
            total += Self::count_subsets(
                coins,
                index + 1,
                next_lcm,
                -sign,
                limit,
            );
        }

        total
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }
}
