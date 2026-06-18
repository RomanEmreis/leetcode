/*
  1344. Angle Between Hands of a Clock
  
  Given two numbers, hour and minutes, return the smaller angle (in degrees) formed between the hour and the minute hand.
  Answers within 10-5 of the actual value will be accepted as correct.
  
  Example 1:
  Input: hour = 12, minutes = 30
  Output: 165
  
  Example 2:
  Input: hour = 3, minutes = 30
  Output: 75
  
  Example 3:
  Input: hour = 3, minutes = 15
  Output: 7.5
*/
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = hour as f64;
        let minutes = minutes as f64;
        let hour_hand = (hour % 12.0 + minutes / 60.0) * 30.0;
        let minutes_hand = minutes * 6.0;
        let diff = (hour_hand - minutes_hand).abs();
        diff.min(360.0 - diff)
    }
}
