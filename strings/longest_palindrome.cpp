/*
  Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
  Letters are case sensitive, for example, "Aa" is not considered a palindrome.

  Example:
    Input: s = "abccccdd"
    Output: 7
    Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
*/
class Solution {
public:
    int longestPalindrome(string s) {
        if (s.length() == 1) return 1;

        unordered_set<char> set;
        int result = 0;

        for (const char& c : s) {
            if (set.contains(c)) {
                set.erase(c);
                result += 2;
            } else {
                set.insert(c);
            }
        }

        if (!set.empty()) ++result;

        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
