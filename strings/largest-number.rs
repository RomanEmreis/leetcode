/*
  Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
  
  Since the result may be very large, so you need to return a string instead of an integer.
  
  Example 1:
    Input: nums = [10,2]
    Output: "210"
  
  Example 2:
    Input: nums = [3,30,34,5,9]
    Output: "9534330"
*/
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_strs: Vec<String> = nums.iter().map(|&n| n.to_string()).collect();
        
        num_strs.sort_by(|x, y| {
            let xy = x.clone() + &y;
            let yx = y.clone() + &x;

            yx.cmp(&xy)
        });

        if num_strs[0] == "0" {
            return String::from("0");
        }

        num_strs.concat()
    }
}
