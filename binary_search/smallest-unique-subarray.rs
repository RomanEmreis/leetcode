/*
  3934. Smallest Unique Subarray
  
  You are given an integer array nums.
  Find the minimum length of a subarray that is not identical to any other subarray in nums.
  
  Return an integer denoting the minimum possible length of such a subarray.
  
  Two subarrays are considered identical if they have the same length and the same elements in corresponding positions.
  
  Example 1:
  Input: nums = [3,3,3]
  Output: 3
  Explanation:
      Subarrays of length 1: [3] → appears 3 times
      Subarrays of length 2: [3, 3] → appears 2 times
      Subarrays of length 3: [3, 3, 3] → appears once
  
  The subarray [3, 3, 3] is unique, so the smallest unique subarray length is 3.
  
  Example 2:
  Input: nums = [2,1,2,3,3]
  Output: 1
  Explanation:
  Subarrays of length 1:
      [2] → appears 2 times
      [1] → appears once
      [3] → appears 2 times
  
  The subarray [1] is unique, so the smallest unique subarray length is 1.
  
  Example 3:
  Input: nums = [1,1,2,2,1]
  Output: 2
  Explanation:
  Subarrays of length 1:
      [1] → appears 3 times
      [2] → appears 2 times
  
  Subarrays of length 2:
      [1, 1] → appears once
      [1, 2] → appears once
      [2, 2] → appears once
      [2, 1] → appears once
  
  There is at least one subarray of length 2 that is unique, so the smallest unique subarray length is 2.
*/
impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut sa: Vec<u32> = (0..n as u32).collect();
        sa.sort_unstable_by_key(|&i| nums[i as usize]);
        let mut rank = vec![0u32; n];
        let mut classes = 1u32;
        for j in 1..n {
            if nums[sa[j] as usize] != nums[sa[j - 1] as usize] {
                classes += 1;
            }
            rank[sa[j] as usize] = classes - 1;
        }

        let mut tmp = vec![0u32; n];
        let mut cnt = vec![0u32; n];
        let mut k = 1;
        while (classes as usize) < n {
            
            let mut p = 0;
            for i in n - k..n {
                tmp[p] = i as u32;
                p += 1;
            }
            for &s in &sa {
                if s as usize >= k {
                    tmp[p] = s - k as u32;
                    p += 1;
                }
            }
            
            let m = classes as usize;
            cnt[..m].fill(0);
            for &r in &rank {
                cnt[r as usize] += 1;
            }
            for c in 1..m {
                cnt[c] += cnt[c - 1];
            }
            for &i in tmp.iter().rev() {
                let c = &mut cnt[rank[i as usize] as usize];
                *c -= 1;
                sa[*c as usize] = i;
            }
            
            let second = |i: usize| if i + k < n { rank[i + k] as i64 } else { -1 };
            tmp[sa[0] as usize] = 0;
            classes = 1;
            for j in 1..n {
                let (a, b) = (sa[j - 1] as usize, sa[j] as usize);
                if rank[a] != rank[b] || second(a) != second(b) {
                    classes += 1;
                }
                tmp[b] = classes - 1;
            }
            std::mem::swap(&mut rank, &mut tmp);
            k <<= 1;
        }
        drop(cnt);

        let mut lcp = tmp;
        lcp.fill(0);
        let mut h = 0usize;
        for i in 0..n {
            let r = rank[i] as usize;
            if r == 0 {
                h = 0;
                continue;
            }
            let j = sa[r - 1] as usize;
            while i + h < n && j + h < n && nums[i + h] == nums[j + h] {
                h += 1;
            }
            lcp[r] = h as u32;
            h = h.saturating_sub(1);
        }

        let mut res = n;
        for r in 0..n {
            let next = if r + 1 < n { lcp[r + 1] } else { 0 };
            let need = lcp[r].max(next) as usize + 1;
            if need <= n - sa[r] as usize {
                res = res.min(need);
            }
        }
        res as i32
    }
}
