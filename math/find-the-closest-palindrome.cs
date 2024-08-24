/*
  Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
  
  The closest is defined as the absolute difference minimized between two integers.
  
  Example 1:
    Input: n = "123"
    Output: "121"
  
  Example 2:
    Input: n = "1"
    Output: "0"
    Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
*/
public class Solution {
    public string NearestPalindromic(string n) {
        long original = long.Parse(n);
        int len = n.Length;
        
        HashSet<long> candidates = [];
        
        string half = n[0..((len + 1) / 2)];
        long halfNum = long.Parse(half);
        
        for (int i = -1; i <= 1; ++i) {
            candidates.Add(CreatePalindrome((halfNum + i).ToString(), len));
        }
        
        candidates.Add((long)Math.Pow(10, len - 1) - 1);
        candidates.Add((long)Math.Pow(10, len) + 1);
        candidates.Remove(original);

        long closest = long.MaxValue;
        long minDiff = long.MaxValue;
        
        foreach (long candidate in candidates) {
            long diff = Math.Abs(candidate - original);
            if (diff < minDiff || (diff == minDiff && candidate < closest)) {
                closest = candidate;
                minDiff = diff;
            }
        }
        
        return closest.ToString();
    }

    private static long CreatePalindrome(string half, int len) {
        Span<char> halfArr = half.ToCharArray();
        halfArr.Reverse();

        string secondHalf = new string(halfArr);
        return long.Parse(half + secondHalf[(len % 2)..]);
    }
}
