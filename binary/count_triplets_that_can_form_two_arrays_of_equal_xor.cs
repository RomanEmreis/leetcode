/*
  Given an array of integers arr.
  We want to select three indices i, j and k where (0 <= i < j <= k < arr.length).

  Let's define a and b as follows:
    a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
    b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
  Note that ^ denotes the bitwise-xor operation.

  Return the number of triplets (i, j and k) Where a == b.

  Example 1:
    Input: arr = [2,3,1,6,7]
    Output: 4
    Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
  
  Example 2:
    Input: arr = [1,1,1,1,1]
    Output: 10
*/
public class Solution {
    public int CountTriplets(int[] arr) {
        int result = 0;

        for (int i = 0; i < arr.Length; ++i) {
            int value = arr[i];

            for (int k = i + 1; k < arr.Length; ++k) {
                value ^= arr[k];
                if (value == 0) result += (k - i);
            }
        }

        return result;
    }
}
