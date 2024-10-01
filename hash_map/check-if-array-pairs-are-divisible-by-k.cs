/*
  Given an array of integers arr of even length n and an integer k.
  
  We want to divide the array into exactly n / 2 pairs such that the sum of each pair is divisible by k.
  
  Return true If you can find a way to do that or false otherwise.
  
  Example 1:
    Input: arr = [1,2,3,4,5,10,6,7,8,9], k = 5
    Output: true
    Explanation: Pairs are (1,9),(2,8),(3,7),(4,6) and (5,10).
*/
public class Solution {
    public bool CanArrange(int[] arr, int k) {
        Dictionary<int, int> freq = new();
        for (int i = 0; i < arr.Length; ++i) {
            int rem = ((arr[i] % k) + k) % k;
            if (!freq.ContainsKey(rem)) freq[rem] = 1;
            else ++freq[rem];
        }

        foreach (var (key, val) in freq) {
            int c = (k - key) % k;
            if (c == key) {
                if (val % 2 != 0) return false;
            } else {
                if (!freq.ContainsKey(c) || val != freq[c]) return false;
            }
        }

        return true;
    }
}
