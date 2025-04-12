/*
  3272. Find the Count of Good Integers
  
  You are given two positive integers n and k.
  An integer x is called k-palindromic if:
  x is a palindrome.
  x is divisible by k.
  An integer is called good if its digits can be rearranged to form a k-palindromic integer. For example, for k = 2, 2020 can be rearranged to form the k-palindromic integer 2002, whereas 1010 cannot be rearranged to form a k-palindromic integer.
  
  Return the count of good integers containing n digits.
  
  Note that any integer must not have leading zeros, neither before nor after rearrangement. For example, 1010 cannot be rearranged to form 101.
  
  Example 1:
  Input: n = 3, k = 5
  Output: 27
  Explanation:
  Some of the good integers are:
  551 because it can be rearranged to form 515.
  525 because it is already k-palindromic.
  
  Example 2:
  Input: n = 1, k = 4
  Output: 2
  Explanation:
  The two good integers are 4 and 8.
  
  Example 3:
  Input: n = 5, k = 6
  Output: 2468
*/
public class Solution {
    public long CountGoodIntegers(int n, int k) {
        Span<long> factorial = stackalloc long[n + 1];
        factorial.Fill(1);
        for (int i = 1; i <= n; ++i) {
            factorial[i] = factorial[i - 1] * i;
        }

        long result = 0;
        int b = (int) Math.Pow(10, (n - 1) / 2);
        HashSet<string> visited = [];

        int offset = n % 2;

        for (int i = b; i < b * 10; ++i) {
            string s = i.ToString();
            char[] revArr = s.ToCharArray();
            Array.Reverse(revArr);
            string rev = new string(revArr);
            s += rev[offset..];

            if (long.Parse(s) % k != 0) continue;

            char[] tArr = s.ToCharArray();
            Array.Sort(tArr);
            string t = new string(tArr);

            if (!visited.Add(t)) continue;

            Span<int> count = stackalloc int[10];
            foreach (char c in t) {
                ++count[c - '0'];
            }

            long res = (n - count[0]) * factorial[n - 1];
            foreach (int c in count) {
                res /= factorial[c];
            }

            result += res;
        }


        return result;
    }
}
