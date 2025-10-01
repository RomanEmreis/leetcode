/*
  1518. Water Bottles
  
  There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.
  The operation of drinking a full water bottle turns it into an empty bottle.
  
  Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
  
  Example 1:
  Input: numBottles = 9, numExchange = 3
  Output: 13
  Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
  Number of water bottles you can drink: 9 + 3 + 1 = 13.
  
  Example 2:
  Input: numBottles = 15, numExchange = 4
  Output: 19
  Explanation: You can exchange 4 empty bottles to get 1 full water bottle. 
  Number of water bottles you can drink: 15 + 3 + 1 = 19.
*/
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        while num_bottles >= num_exchange {
            let exchanged: i32 = num_bottles / num_exchange; 
            num_bottles = exchanged + (num_bottles % num_exchange);
            res += exchanged;
        }
        res
    }
}
