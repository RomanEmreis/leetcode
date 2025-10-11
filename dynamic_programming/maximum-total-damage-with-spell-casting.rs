/*
  3186. Maximum Total Damage With Spell Casting
  
  A magician has various spells.
  You are given an array power, where each element represents the damage of a spell. Multiple spells can have the same damage value.
  It is a known fact that if a magician decides to cast a spell with a damage of power[i], they cannot cast any spell with a damage of power[i] - 2, power[i] - 1, power[i] + 1, or power[i] + 2.
  Each spell can be cast only once.
  
  Return the maximum possible total damage that a magician can cast.
  
  Example 1:
  Input: power = [1,1,3,4]
  Output: 6
  Explanation:
  The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.
  
  Example 2:
  Input: power = [7,1,6,6]
  Output: 13
  Explanation:
  The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.
*/
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort_unstable();
        let mut q: VecDeque<(i32, i64)> = VecDeque::new();
        for &p in power.iter() {
            let l = if q.is_empty() { 0 } else { q.len() - 1 };
            if !q.is_empty() && q[l].0 == p {
                q[l].1 += p as i64;
            } else {
                let mut max = 0;
                for i in 0..q.len() {
                    if 2 < p - q[i].0 {
                        max = max.max(q[i].1);
                    }
                }
                q.push_back((p, max + p as i64));
                if q.len() == 6 {
                    q.pop_front();
                }
            }
        }
        q.into_iter()
            .map(|q| q.1)
            .max()
            .unwrap()
    }
}
