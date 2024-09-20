/*
  You are given a string s. You can convert s to a palindrome by adding characters in front of it.
  
  Return the shortest palindrome you can find by performing this transformation.
  
  Example 1:
  Input: s = "aacecaaa"
  Output: "aaacecaaa"
  
  Example 2:
  Input: s = "abcd"
  Output: "dcbabcd"
*/
class Solution {
private:
    constexpr static long long MOD = 1000000007;
    constexpr static long long BASE = 26;
public:
    string shortestPalindrome(string s) {
        if (s.length() <= 1) return s;

        long long hash = 0, reverse_hash = 0, power = 1;
        int length = 0;

        for (size_t i = 0; i < s.length(); ++i) {
            int c_num = s[i] - 'a' + 1;
            hash = (hash * BASE + c_num) % MOD;
            reverse_hash = (reverse_hash + c_num * power) % MOD;
            power = (power * BASE) % MOD;

            if (hash == reverse_hash) length = i + 1;
        }

        string rev_suffix = s.substr(length, s.length());
        reverse(rev_suffix.begin(), rev_suffix.end());
        return rev_suffix + s; 
    }
};
