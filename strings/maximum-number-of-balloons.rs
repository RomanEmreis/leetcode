/*
  1189. Maximum Number of Balloons
  
  Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.
  
  You can use each character in text at most once. Return the maximum number of instances that can be formed.
  
  Example 1:
  Input: text = "nlaebolko"
  Output: 1
  
  Example 2:
  Input: text = "loonbalxballpoon"
  Output: 2
  
  Example 3:
  Input: text = "leetcode"
  Output: 0
*/
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut freq = text.bytes().fold([0; 26], |mut freq, ch| {
            freq[(ch - b'a') as usize] += 1;
            freq
        });
        
        freq[1]
            .min(freq[0])
            .min(freq[11] / 2)
            .min(freq[14] / 2)
            .min(freq[13])
    }
}
