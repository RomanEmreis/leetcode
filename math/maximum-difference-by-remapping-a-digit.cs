/*
  2566. Maximum Difference by Remapping a Digit
  
  You are given an integer num. You know that Bob will sneakily remap one of the 10 possible digits (0 to 9) to another digit.
  
  Return the difference between the maximum and minimum values Bob can make by remapping exactly one digit in num.
  
  Notes:
  When Bob remaps a digit d1 to another digit d2, Bob replaces all occurrences of d1 in num with d2.
  Bob can remap a digit to itself, in which case num does not change.
  Bob can remap different digits for obtaining minimum and maximum values respectively.
  The resulting number after remapping can contain leading zeroes.
   
  
  Example 1:
  Input: num = 11891
  Output: 99009
  Explanation: 
  To achieve the maximum value, Bob can remap the digit 1 to the digit 9 to yield 99899.
  To achieve the minimum value, Bob can remap the digit 1 to the digit 0, yielding 890.
  The difference between these two numbers is 99009.
  
  Example 2:
  Input: num = 90
  Output: 99
  Explanation:
  The maximum value that can be returned by the function is 99 (if 0 is replaced by 9) and the minimum value that can be returned by the function is 0 (if 9 is replaced by 0).
  Thus, we return 99.
*/
public class Solution {
    public int MinMaxDifference(int num) {
        List<int> digits = [];
        int n = num;
        while (n > 0) {
            int d = n % 10;
            n /= 10;
            digits.Add(d);
        }

        digits.Reverse();

        int maxD = -1;
        int minD = -1;

        foreach (int d in digits) {
            if (maxD == -1 && d != 9) maxD = d;
            if (minD == -1 && d != 0) minD = d;
            if (maxD != -1 && minD != -1) break;
        }

        int max = 0;
        for (int i = 0; i < digits.Count; ++i) {
            max *= 10;
            max += digits[i] == maxD ? 9 : digits[i];
        }

        int min = 0;
        for (int i = 0; i < digits.Count; ++i) {
            min *= 10;
            min += digits[i] == minD ? 0 : digits[i];
        }

        return max - min;
    }
}
