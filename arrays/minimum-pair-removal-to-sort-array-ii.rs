/*
  3510. Minimum Pair Removal to Sort Array II
  
  Given an array nums, you can perform the following operation any number of times:
      Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
      Replace the pair with their sum.
  
  Return the minimum number of operations needed to make the array non-decreasing.
  
  An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).
  
  Example 1:
  Input: nums = [5,2,3,1]
  Output: 2
  Explanation:
      The pair (3,1) has the minimum sum of 4. After replacement, nums = [5,2,4].
      The pair (2,4) has the minimum sum of 6. After replacement, nums = [5,6].
  The array nums became non-decreasing in two operations.
  
  Example 2:
  Input: nums = [1,2,2]
  Output: 0
  Explanation:
  The array nums is already sorted.
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        
        let mut vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut nexts: Vec<i32> = (0..n as i32).map(|i| i + 1).collect();
        let mut prevs: Vec<i32> = (0..n as i32).map(|i| i - 1).collect();
        let mut removed: Vec<bool> = vec![false; n];
        
        nexts[n - 1] = -1;
        
        let mut heap: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::with_capacity(3 * n);
        let mut unsorted_cnt: i32 = 0;
        
        for i in 0..n - 1 {
            if vals[i] > vals[i + 1] {
                unsorted_cnt += 1;
            }
            heap.push(
                Reverse((vals[i] + vals[i + 1], i as i32))
            );
        }
        
        if unsorted_cnt == 0 {
            return 0;
        }
        
        let mut moves = 0;
        
        while unsorted_cnt > 0 {
            let Reverse((sum, u)) = match heap.pop() {
                Some(x) => x,
                None => break,
            };
            
            let u = u as usize;
            if removed[u] {
                continue;
            }
            
            let v = nexts[u];
            if v == -1 {
                continue;
            }

            let v = v as usize;
            if removed[v] {
                continue;
            }
            
            if vals[u] + vals[v] != sum {
                continue;
            }
            
            let p = prevs[u];
            let next_v = nexts[v];
            
            moves += 1;
            
            if p != -1 && vals[p as usize] > vals[u] {
                unsorted_cnt -= 1;
            }

            if vals[u] > vals[v] {
                unsorted_cnt -= 1;
            }
            
            if next_v != -1 && vals[v] > vals[next_v as usize] {
                unsorted_cnt -= 1;
            }
            
            vals[u] = sum;
            nexts[u] = next_v;
            if next_v != -1 {
                prevs[next_v as usize] = u as i32;
            }

            removed[v] = true;
            
            if p != -1 && vals[p as usize] > vals[u] {
                unsorted_cnt += 1;
            }
            if next_v != -1 && vals[u] > vals[next_v as usize] {
                unsorted_cnt += 1;
            }
            
            if unsorted_cnt == 0 {
                break;
            }
            
            if p != -1 {
                heap.push(
                    Reverse((vals[p as usize] + vals[u], p))
                );
            }

            if next_v != -1 {
                heap.push(
                    Reverse((vals[u] + vals[next_v as usize], u as i32))
                );
            }
        }
        
        moves
    }
}
