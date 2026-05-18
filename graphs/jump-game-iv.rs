/*
  1345. Jump Game IV
  
  Given an array of integers arr, you are initially positioned at the first index of the array.
  In one step you can jump from index i to index:
      i + 1 where: i + 1 < arr.length.
      i - 1 where: i - 1 >= 0.
      j where: arr[i] == arr[j] and i != j.
  
  Return the minimum number of steps to reach the last index of the array.
  
  Notice that you can not jump outside of the array at any time.
  
  Example 1:
  Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
  Output: 3
  Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
  
  Example 2:
  Input: arr = [7]
  Output: 0
  Explanation: Start index is the last index. You do not need to jump.
  
  Example 3:
  Input: arr = [7,6,9,6,9,6,9,7]
  Output: 1
  Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
*/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n0 = arr.len();
        if n0 == 1 {
            return 0;
        }

        let mut compact: Vec<i32> = Vec::with_capacity(n0);
        compact.push(arr[0]);
        for i in 1..n0 - 1 {
            if !(arr[i] == arr[i - 1] && arr[i] == arr[i + 1]) {
                compact.push(arr[i]);
            }
        }
        compact.push(arr[n0 - 1]);
        let n = compact.len();
        if n == 1 {
            return 0;
        }

        let mut order: Vec<u32> = (0..n as u32).collect();
        order.sort_unstable_by_key(|&i| compact[i as usize]);

        let mut idx: Vec<u32> = vec![0; n];
        let mut k: u32 = 0;
        
        let mut group_members: Vec<u32> = Vec::with_capacity(n);
        let mut group_starts: Vec<u32> = Vec::with_capacity(n + 1);
        group_starts.push(0);

        let mut p = 0;
        while p < n {
            let v = compact[order[p] as usize];
            let mut q = p;
            while q < n && compact[order[q] as usize] == v {
                let oi = order[q] as usize;
                idx[oi] = k;
                group_members.push(order[q]);
                q += 1;
            }
            group_starts.push(group_members.len() as u32);
            k += 1;
            p = q;
        }
        let mut group_open = vec![true; k as usize];

        let mut seen: Vec<u8> = vec![0u8; n];

        let mut front: Vec<u32> = Vec::with_capacity(n);
        let mut next: Vec<u32> = Vec::with_capacity(n);
        let mut back: Vec<u32> = Vec::with_capacity(n);
        let mut next_b: Vec<u32> = Vec::with_capacity(n);

        front.push(0);
        back.push((n - 1) as u32);
        unsafe {
            *seen.get_unchecked_mut(0) |= 0b01;
            *seen.get_unchecked_mut(n - 1) |= 0b10;
        }

        if compact[0] == compact[n - 1] {
            return 1;
        }

        let mut step_f: i32 = 0;
        let mut step_b: i32 = 0;

        loop {
            if front.is_empty() || back.is_empty() {
                return -1;
            }

            let expand_forward = front.len() <= back.len();
            let (cur, nxt, my_bit, other_bit, my_step) = if expand_forward {
                step_f += 1;
                (&mut front, &mut next, 0b01u8, 0b10u8, step_f + step_b)
            } else {
                step_b += 1;
                (&mut back, &mut next_b, 0b10u8, 0b01u8, step_f + step_b)
            };

            nxt.clear();

            unsafe {
                for &iu in cur.iter() {
                    let i = iu as usize;

                    if i + 1 < n {
                        let s = *seen.get_unchecked(i + 1);
                        if s & other_bit != 0 {
                            return my_step;
                        }
                        if s & my_bit == 0 {
                            *seen.get_unchecked_mut(i + 1) = s | my_bit;
                            nxt.push((i + 1) as u32);
                        }
                    }
                    
                    if i > 0 {
                        let s = *seen.get_unchecked(i - 1);
                        if s & other_bit != 0 {
                            return my_step;
                        }
                        if s & my_bit == 0 {
                            *seen.get_unchecked_mut(i - 1) = s | my_bit;
                            nxt.push((i - 1) as u32);
                        }
                    }
                    
                    let gi = *idx.get_unchecked(i) as usize;
                    if *group_open.get_unchecked(gi) {
                        *group_open.get_unchecked_mut(gi) = false;
                        let start = *group_starts.get_unchecked(gi) as usize;
                        let end = *group_starts.get_unchecked(gi + 1) as usize;
                        for p in start..end {
                            let j = *group_members.get_unchecked(p) as usize;
                            if j == i { continue; }
                            let s = *seen.get_unchecked(j);
                            if s & other_bit != 0 {
                                return my_step;
                            }
                            if s & my_bit == 0 {
                                *seen.get_unchecked_mut(j) = s | my_bit;
                                nxt.push(j as u32);
                            }
                        }
                    }
                }
            }

            std::mem::swap(cur, nxt);
        }
    }
}
