/*
  3691. Maximum Total Subarray Value II
  
  You are given an integer array nums of length n and an integer k.
  You must select exactly k distinct nums[l..r] of nums. Subarrays may overlap, but the exact same subarray (same l and r) cannot be chosen more than once.
    The value of a subarray nums[l..r] is defined as: max(nums[l..r]) - min(nums[l..r]).
    The total value is the sum of the values of all chosen subarrays.
  
  Return the maximum possible total value you can achieve.
  
  Example 1:
  Input: nums = [1,3,2], k = 2
  Output: 4
  Explanation:
  
  One optimal approach is:
      Choose nums[0..1] = [1, 3]. The maximum is 3 and the minimum is 1, giving a value of 3 - 1 = 2.
      Choose nums[0..2] = [1, 3, 2]. The maximum is still 3 and the minimum is still 1, so the value is also 3 - 1 = 2.
  
  Adding these gives 2 + 2 = 4.
  
  Example 2:
  Input: nums = [4,2,5,1], k = 3
  Output: 12
  Explanation:
  One optimal approach is:
      Choose nums[0..3] = [4, 2, 5, 1]. The maximum is 5 and the minimum is 1, giving a value of 5 - 1 = 4.
      Choose nums[1..3] = [2, 5, 1]. The maximum is 5 and the minimum is 1, so the value is also 4.
      Choose nums[2..3] = [5, 1]. The maximum is 5 and the minimum is 1, so the value is again 4.
  
  Adding these gives 4 + 4 + 4 = 12.
*/
fn walk<F>(a: &[i32], x: i64, max_dq: &mut [u32], min_dq: &mut [u32], mut on_match: F)
where
    F: FnMut(usize, usize, i32, i32) -> bool,
{
    let n = a.len();
    let (mut mh, mut mt) = (0usize, 0usize);
    let (mut nh, mut nt) = (0usize, 0usize);
    let mut r = 0usize;

    for l in 0..n {
        while mh < mt && (max_dq[mh] as usize) < l {
            mh += 1;
        }
        while nh < nt && (min_dq[nh] as usize) < l {
            nh += 1;
        }
        loop {
            if mh < mt {
                let mx = a[max_dq[mh] as usize];
                let mn = a[min_dq[nh] as usize];
                if i64::from(mx) - i64::from(mn) >= x {
                    if !on_match(l, r - 1, mx, mn) {
                        return;
                    }
                    break;
                }
            }
            if r == n {
                return;
            }
            let v = a[r];
            while mh < mt && a[max_dq[mt - 1] as usize] <= v {
                mt -= 1;
            }
            max_dq[mt] = r as u32;
            mt += 1;
            while nh < nt && a[min_dq[nt - 1] as usize] >= v {
                nt -= 1;
            }
            min_dq[nt] = r as u32;
            nt += 1;
            r += 1;
        }
    }
}

fn count_at_least(a: &[i32], x: i64, cap: i64, max_dq: &mut [u32], min_dq: &mut [u32]) -> i64 {
    let n = a.len() as i64;
    if x <= 0 {
        return n * (n + 1) / 2;
    }
    let mut cnt = 0i64;
    walk(a, x, max_dq, min_dq, |_, r_min, _, _| {
        cnt += n - r_min as i64;
        cnt < cap
    });
    cnt
}

fn sum_at_least(a: &[i32], x: i64, max_dq: &mut [u32], min_dq: &mut [u32]) -> (i64, i64) {
    let n = a.len();
    let (mut sum, mut cnt) = (0i64, 0i64);
    walk(a, x, max_dq, min_dq, |_, r_min, mx0, mn0| {
        let (mut mx, mut mn) = (mx0, mn0);
        sum += i64::from(mx) - i64::from(mn);
        for &v in &a[r_min + 1..n] {
            if v > mx {
                mx = v;
            }
            if v < mn {
                mn = v;
            }
            sum += i64::from(mx) - i64::from(mn);
        }
        cnt += (n - r_min) as i64;
        true
    });
    (sum, cnt)
}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = i64::from(k);
        let mut max_dq = vec![0u32; n];
        let mut min_dq = vec![0u32; n];

        let (gmax, gmin) = nums
            .iter()
            .fold((i32::MIN, i32::MAX), |(mx, mn), &v| (mx.max(v), mn.min(v)));
        let max_value = i64::from(gmax) - i64::from(gmin);

        let (mut lo, mut hi, mut t) = (0i64, max_value, 0i64);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if count_at_least(&nums, mid, k, &mut max_dq, &mut min_dq) >= k {
                t = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        let (sum, cnt) = if t < max_value {
            sum_at_least(&nums, t + 1, &mut max_dq, &mut min_dq)
        } else {
            (0, 0)
        };
        sum + (k - cnt) * t
    }
}
