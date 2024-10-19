/*
  Given two positive integers n and k, the binary string Sn is formed as follows:
  
  S1 = "0"
  Si = Si - 1 + "1" + reverse(invert(Si - 1)) for i > 1
  Where + denotes the concatenation operation, reverse(x) returns the reversed string x, and invert(x) inverts all the bits in x (0 changes to 1 and 1 changes to 0).
  
  For example, the first four strings in the above sequence are:
  
  S1 = "0"
  S2 = "011"
  S3 = "0111001"
  S4 = "011100110110001"
  Return the kth bit in Sn. It is guaranteed that k is valid for the given n.
  
  Example 1:
    Input: n = 3, k = 1
    Output: "0"
    Explanation: S3 is "0111001".
    The 1st bit is "0".
*/
public class Solution {
    public char FindKthBit(int n, int k) {
        StringBuilder sb = new();
        sb.Append('0');

        for (int i = 1; i < n; ++i) {
            StringBuilder s = new();
            s.Append('1');
            for (int j = sb.Length - 1; j >= 0; --j) {
                s.Append(sb[j] == '0' ? '1' : '0');
            }
            sb.Append(s);
        }

        return sb[k - 1];
    }
}
