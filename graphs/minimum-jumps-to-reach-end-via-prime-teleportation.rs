/*
  3629. Minimum Jumps to Reach End via Prime Teleportation
  
  You are given an integer array nums of length n.
  You start at index 0, and your goal is to reach index n - 1.
  
  From any index i, you may perform one of the following operations:
      Adjacent Step: Jump to index i + 1 or i - 1, if the index is within bounds.
      Prime Teleportation: If nums[i] is a p, you may instantly jump to any index j != i such that nums[j] % p == 0.
  
  Return the minimum number of jumps required to reach index n - 1.
  
  Example 1:
  Input: nums = [1,2,4,6]
  Output: 2
  Explanation:
  One optimal sequence of jumps is:
      Start at index i = 0. Take an adjacent step to index 1.
      At index i = 1, nums[1] = 2 is a prime number. Therefore, we teleport to index i = 3 as nums[3] = 6 is divisible by 2.
  Thus, the answer is 2.
  
  Example 2:
  Input: nums = [2,3,4,7,9]
  Output: 2
  Explanation:
  One optimal sequence of jumps is:
      Start at index i = 0. Take an adjacent step to index i = 1.
      At index i = 1, nums[1] = 3 is a prime number. Therefore, we teleport to index i = 4 since nums[4] = 9 is divisible by 3.
  Thus, the answer is 2.
  
  Example 3:
  Input: nums = [4,6,5,8]
  Output: 3
  Explanation:
      Since no teleportation is possible, we move through 0 → 1 → 2 → 3. Thus, the answer is 3.
*/
use std::collections::HashMap;
use std::sync::OnceLock;
 
const MAX_V: usize = 1_000_001;
 
fn spf() -> &'static [u32] {
    static SPF: OnceLock<Vec<u32>> = OnceLock::new();
    SPF.get_or_init(|| {
        let mut spf: Vec<u32> = vec![0; MAX_V];
        let mut primes: Vec<u32> = Vec::with_capacity(80_000);
        for i in 2..MAX_V {
            if spf[i] == 0 {
                spf[i] = i as u32;
                primes.push(i as u32);
            }
            for &p in &primes {
                let ip = (p as usize) * i;
                if p > spf[i] || ip >= MAX_V {
                    break;
                }
                spf[ip] = p;
            }
        }
        spf
    })
    .as_slice()
}
 
impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let spf = spf();
 
        let mut buckets: HashMap<u32, Vec<u32>> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let v = v as usize;
            if v >= 2 && spf[v] as usize == v {
                buckets
                    .entry(v as u32)
                    .or_default()
                    .push(i as u32);
            }
        }
 
        let mut seen = vec![false; n];
        seen[n - 1] = true;
        let mut current: Vec<u32> = Vec::with_capacity(n);
        current.push((n - 1) as u32);
        let mut next: Vec<u32> = Vec::with_capacity(n);
        let mut dist: i32 = 0;
 
        while !current.is_empty() {
            for &i_u32 in &current {
                let i = i_u32 as usize;
                if i == 0 {
                    return dist;
                }
 
                if i > 0 && !seen[i - 1] {
                    seen[i - 1] = true;
                    next.push((i - 1) as u32);
                }
                if i + 1 < n && !seen[i + 1] {
                    seen[i + 1] = true;
                    next.push((i + 1) as u32);
                }
 
                let mut x = nums[i] as usize;
                while x > 1 {
                    let p = spf[x];
                    if let Some(bucket) = buckets.get_mut(&p) {
                        for &j in bucket.iter() {
                            let j = j as usize;
                            if !seen[j] {
                                seen[j] = true;
                                next.push(j as u32);
                            }
                        }
                        bucket.clear();
                    }
                    let pu = p as usize;
                    while x % pu == 0 {
                        x /= pu;
                    }
                }
            }
            
            current.clear();
            std::mem::swap(&mut current, &mut next);
            dist += 1;
        }
 
        -1
    }
}
