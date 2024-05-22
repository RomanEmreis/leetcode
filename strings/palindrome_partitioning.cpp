/*
  Given a string s, partition s such that every substring of the partition is a palindrome
  Return all possible palindrome partitioning of s.

  Example:
    Input: s = "aab"
    Output: [["a","a","b"],["aa","b"]]
*/
class Solution {
private:
    static void dfs(int idx, int& n, string& s, vector<vector<string>>& result, vector<string>& current) {
        if (idx >= n) {
            result.push_back(current);
            return;
        }

        for (int i = idx; i < n; ++i) {
            string sub = s.substr(idx, i - idx + 1);
            if (isPalindrome(sub)) {
                current.push_back(sub);
                dfs(i + 1, n, s, result, current);
                current.pop_back();
            }
        }
    }

    static bool isPalindrome(const string& s) {
        int l = 0, r = s.size() - 1;
        while (l < r) {
            if (s[l++] != s[r--]) return false;
        }

        return true;
    }

public:
    vector<vector<string>> partition(string& s) {
        vector<vector<string>> result;
        vector<string> current;
        int n = s.size();

        dfs(0, n, s, result, current);

        return result;
    }
};
