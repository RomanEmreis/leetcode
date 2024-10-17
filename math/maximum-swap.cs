/*
  You are given an integer num. You can swap two digits at most once to get the maximum valued number.
  
  Return the maximum valued number you can get.
  
  Example 1:
    Input: num = 2736
    Output: 7236
    Explanation: Swap the number 2 and the number 7.
  
  Example 2:
    Input: num = 9973
    Output: 9973
    Explanation: No swap.
*/
public class Solution {
    public int MaximumSwap(int num) {
        char[] digits = num.ToString().ToCharArray();
        int[] last = new int[10];
        
        for (int i = 0; i < digits.Length; i++) {
            last[digits[i] - '0'] = i;
        }
        
        for (int i = 0; i < digits.Length; i++) {
            for (int d = 9; d > digits[i] - '0'; d--) {
                if (last[d] > i) {
                    char temp = digits[i];
                    digits[i] = digits[last[d]];
                    digits[last[d]] = temp;
                    return int.Parse(new string(digits));
                }
            }
        }
        
        return num;
    }
}
