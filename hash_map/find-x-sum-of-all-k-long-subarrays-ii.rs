/*
  3321. Find X-Sum of All K-Long Subarrays II
  
  You are given an array nums of n integers and two integers k and x.
  The x-sum of an array is calculated by the following procedure:
      Count the occurrences of all elements in the array.
      Keep only the occurrences of the top x most frequent elements. If two elements have the same number of occurrences, the element with the bigger value is considered more frequent.
      Calculate the sum of the resulting array.
  
  Note that if an array has less than x distinct elements, its x-sum is the sum of the array.
  Return an integer array answer of length n - k + 1 where answer[i] is the x-sum of the nums[i..i + k - 1].
  
  Example 1:
  Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
  Output: [6,10,12]
  Explanation:
      For subarray [1, 1, 2, 2, 3, 4], only elements 1 and 2 will be kept in the resulting array. Hence, answer[0] = 1 + 1 + 2 + 2.
      For subarray [1, 2, 2, 3, 4, 2], only elements 2 and 4 will be kept in the resulting array. Hence, answer[1] = 2 + 2 + 2 + 4. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.
      For subarray [2, 2, 3, 4, 2, 3], only elements 2 and 3 are kept in the resulting array. Hence, answer[2] = 2 + 2 + 2 + 3 + 3.
  
  Example 2:
  Input: nums = [3,8,7,8,7,5], k = 2, x = 2
  Output: [11,15,15,15,12]
  Explanation:
  Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].
*/
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_x_sum(mut nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k: usize = k as usize;
        let size = nums.len();
        let mut temp: Vec<i32> = nums.clone();

        temp.sort_unstable();
        
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut num: i64 = 0;
        let mut map: Vec<i64> = vec![0; size];
        let mut hash: HashMap<i32, i32> = HashMap::new();
        
        while i < size {
            map[num as usize] = temp[i] as i64;
            hash.insert(temp[i], num as i32);
            while j < size && temp[i] == temp[j] {
                j += 1;
            }
            num += 1;
            i = j;
        }
        
        for i in 0..size {
            nums[i] = *hash.get(&nums[i]).unwrap();
        }
        
        let mut balance: Balance = Balance {
            active: BinaryHeap::new(), 
            inactive: BinaryHeap::new(), 
            is_active: vec![false; size], 
            len: 0, 
            sum: 0
        };

        let mut counts: Vec<i64> = vec![0; size];
        let mut res: Vec<i64> = Vec::new();
        for i in 0..size {
            counts[nums[i] as usize] += 1;
            balance.add(&(nums[i] as i64), &counts[nums[i] as usize], &map, &x);
            balance.clean(&counts);
            if i == k - 1 {
                res.push(balance.sum);
            }
            if i >= k  {
                counts[nums[i - k] as usize] -= 1;
                balance.subtract(&(nums[i - k] as i64), &counts[nums[i - k] as usize], &map, &x);
                balance.clean(&counts); 
                res.push(balance.sum);
            }   
        }

        return res
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Element {
    count: i64,
    value: i64,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Balance {
    active: BinaryHeap<Element>,
    inactive: BinaryHeap<Element>,
    is_active: Vec<bool>,
    len: i32,
    sum: i64,
}

impl Balance {
    fn add(&mut self, value: &i64, count: &i64, map: &Vec<i64>, x: &i32) {
        if !self.is_active[*value as usize] {
            self.is_active[*value as usize] = true;
            self.len += 1;
            self.sum += count*map[*value as usize];
        } else {
            self.sum += map[*value as usize];
        }
        self.active.push(Element{count: -1**count, value: -1*value});
        
        if self.len > *x {
            let top_count = -self.active.peek().unwrap().count;
            let top_value = -self.active.peek().unwrap().value;
            
            self.active.pop();
            self.inactive.push(Element{count: top_count, value: top_value});
            self.is_active[top_value as usize] = false;
            self.len -= 1;
            self.sum -= top_count*map[top_value as usize];
        }
    }
    
    fn subtract(&mut self, value: &i64, count: &i64, map: &Vec<i64>, x: &i32) {
        if !self.is_active[*value as usize] {
            self.inactive.push(Element{count: *count, value: *value});
        } else {
            self.active.push(Element{count: -1**count, value: -1*value});
            self.sum -= map[*value as usize];
            
            if !self.inactive.is_empty() {
                let top_count = self.inactive.peek().unwrap().count;
                let top_value = self.inactive.peek().unwrap().value;
             
                self.active.push(Element{count: -top_count, value: -top_value});
                self.inactive.pop();
                self.is_active[top_value as usize] = true;
                self.len += 1;
                self.sum += top_count*map[top_value as usize];
                
                if self.len > *x {
                    let top_count = -self.active.peek().unwrap().count;
                    let top_value = -self.active.peek().unwrap().value;
                    self.active.pop();
                    self.inactive.push(Element{count: top_count, value: top_value});
                    self.is_active[top_value as usize] = false;
                    self.len -= 1;
                    self.sum -= top_count*map[top_value as usize];
                }
            }
        }
    }
    
    fn clean(&mut self, counts: &Vec<i64>) {
        while !self.active.is_empty() {
            let top_count = -self.active.peek().unwrap().count;
            let top_value = -self.active.peek().unwrap().value;
            
            if !self.is_active[top_value as usize] || top_count != counts[top_value as usize] {
                self.active.pop();
            } else {
                break
            }
        }
        
        while !self.inactive.is_empty() {
            let top_count = self.inactive.peek().unwrap().count;
            let top_value = self.inactive.peek().unwrap().value;
            
            if self.is_active[top_value as usize] || top_count != counts[top_value as usize] {
                self.inactive.pop();
            } else {
                break
            }
        }
    }
}
