/*
  3074. Apple Redistribution into Boxes
  
  You are given an array apple of size n and an array capacity of size m.
  There are n packs where the ith pack contains apple[i] apples. There are m boxes as well, and the ith box has a capacity of capacity[i] apples.
  Return the minimum number of boxes you need to select to redistribute these n packs of apples into boxes.
  
  Note that, apples from the same pack can be distributed into different boxes.
  
  Example 1:
  Input: apple = [1,3,2], capacity = [4,3,1,5,2]
  Output: 2
  Explanation: We will use boxes with capacities 4 and 5.
  It is possible to distribute the apples as the total capacity is greater than or equal to the total number of apples.
  
  Example 2:
  Input: apple = [5,5,5], capacity = [2,4,2,7]
  Output: 4
  Explanation: We will need to use all the boxes.
*/
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_by(|a, b| b.cmp(&a));

        let total: i32 = apple.iter().sum();
        let mut boxed = 0;
        let mut res = 0;

        for c in capacity.into_iter() {
            boxed += c;
            res += 1;
            if boxed >= total {
                return res;
            }
        }

        res
    }
}
