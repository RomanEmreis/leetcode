/*
  2131. Longest Palindrome by Concatenating Two Letter Words
  
  You are given an array of strings words. Each element of words consists of two lowercase English letters.
  Create the longest possible palindrome by selecting some elements from words and concatenating them in any order. Each element can be selected at most once.
  
  Return the length of the longest palindrome that you can create. If it is impossible to create any palindrome, return 0.
  
  A palindrome is a string that reads the same forward and backward.
  
  Example 1:
  Input: words = ["lc","cl","gg"]
  Output: 6
  Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of length 6.
  Note that "clgglc" is another longest palindrome that can be created.
  
  Example 2:
  Input: words = ["ab","ty","yt","lc","cl","ab"]
  Output: 8
  Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt", of length 8.
  Note that "lcyttycl" is another longest palindrome that can be created.
  
  Example 3:
  Input: words = ["cc","ll","xx"]
  Output: 2
  Explanation: One longest palindrome is "cc", of length 2.
  Note that "ll" is another longest palindrome that can be created, and so is "xx".
*/
public class Solution {
    public int LongestPalindrome(string[] words) {
        var count = new int[26, 26];
        int result = 0;

        foreach(var word in words) {        
            var c1 = word[0] - 'a';
            var c2 = word[1] - 'a';

            if(count[c2, c1] != 0) { 
                result += 4;
                --count[c2, c1];
            } else {
                ++count[c1, c2];
            }
        }

        for(int i = 0; i < 26; ++i){
            if(count[i, i] != 0) {
                result += 2;
                break;
            }
        }

        return result;
    }
}
