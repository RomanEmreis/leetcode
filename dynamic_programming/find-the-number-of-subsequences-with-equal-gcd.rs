/*
  3336. Find the Number of Subsequences With Equal GCD
  
  You are given an integer array nums.
  Your task is to find the number of pairs of non-empty (seq1, seq2) of nums that satisfy the following conditions:
      The subsequences seq1 and seq2 are disjoint, meaning no index of nums is common between them.
      The of the elements of seq1 is equal to the GCD of the elements of seq2.
  
  Return the total number of such pairs.
  
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: nums = [1,2,3,4]
  Output: 10
  Explanation:
  The subsequence pairs which have the GCD of their elements equal to 1 are:
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
      ([1, 2, 3, 4], [1, 2, 3, 4])
  
  Example 2:
  Input: nums = [10,20,30]
  Output: 2
  Explanation:
  The subsequence pairs which have the GCD of their elements equal to 10 are:
      ([10, 20, 30], [10, 20, 30])
      ([10, 20, 30], [10, 20, 30])
  
  Example 3:
  Input: nums = [1,1,1,1]
  Output: 50
*/
use std::sync::OnceLock;

const MOD: u64 = 1_000_000_007;
const MAX: usize = 200;

fn mobius() -> &'static [i8; MAX + 1] {
    static MU: OnceLock<[i8; MAX + 1]> = OnceLock::new();
    MU.get_or_init(|| {
        let mut mu = [1i8; MAX + 1];
        let mut is_comp = [false; MAX + 1];
        let mut primes = Vec::with_capacity(64);
        for i in 2..=MAX {
            if !is_comp[i] {
                primes.push(i);
                mu[i] = -1;
            }
            for &p in &primes {
                let m = i * p;
                if m > MAX { break; }
                is_comp[m] = true;
                if i % p == 0 { mu[m] = 0; break; }
                mu[m] = -mu[i];
            }
        }
        mu
    })
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_v = nums.iter().copied().max().unwrap_or(0) as usize;

        let mut freq = [0u32; MAX + 1];
        for &x in &nums { 
            freq[x as usize] += 1;
        }
        
        let mut cnt = [0u32; MAX + 1];
        for d in 1..=max_v {
            let mut s = 0;
            let mut m = d;
            while m <= max_v { 
                s += freq[m]; m += d;
            }
            cnt[d] = s;
        }

        let mut pow2 = [1u64; MAX + 1];
        let mut pow3 = [1u64; MAX + 1];
        for i in 1..=n {
            pow2[i] = pow2[i - 1] * 2 % MOD;
            pow3[i] = pow3[i - 1] * 3 % MOD;
        }

        let f = |a: usize, b: usize| -> u64 {
            let ca = cnt[a] as usize;
            let cb = cnt[b] as usize;
            let l = a / gcd(a, b) * b;
            let cl = if l <= max_v { 
                cnt[l] as usize
            } else { 
                0
            };
            (pow3[cl] * pow2[ca + cb - 2 * cl] + 2 * MOD - pow2[ca] - pow2[cb] + 1) % MOD
        };

        let mu = mobius();
        let mut res: u64 = 0;

        for g in 1..=max_v {
            if cnt[g] == 0 { 
                continue;
            }

            let lim = max_v / g;
            for k1 in 1..=lim {
                if mu[k1] == 0 || cnt[g * k1] == 0 { 
                    continue;
                }
                for k2 in 1..=lim {
                    if mu[k2] == 0 || cnt[g * k2] == 0 { 
                        continue;
                    }

                    let t = f(g * k1, g * k2);
                    res += if mu[k1] * mu[k2] == 1 { t } else { MOD - t };
                }
            }
        }

        (res % MOD) as i32
    }
}
