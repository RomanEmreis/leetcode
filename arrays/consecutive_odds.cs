/*
  Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.
 
  Example:
    Input: arr = [2,6,4,1]
    Output: false
    Explanation: There are no three consecutive odds.
*/
public class Solution {
    public bool ThreeConsecutiveOdds(int[] arr) {
        if (arr.Length < 3) return false;
        
        int i = 0;
        foreach (int n in arr) {
            if (n % 2 == 1) ++i;
            else i = 0;

            if (i == 3) return true;
        }

        return false;
    }
}
