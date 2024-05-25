/*
  Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
  Note that the same word in the dictionary may be reused multiple times in the segmentation.

  Example:
    Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
    Output: ["cats and dog","cat sand dog"]
*/
class Solution {
private:
    static vector<string> dfs(int start, const string& s, const unordered_set<string>& set) {
        vector<string> result;

        if (start == s.length()) result.push_back("");

        for (int end = start + 1; end <= s.length(); ++end) {
            string prefix = s.substr(start, end - start);
            if (set.contains(prefix)) {
                vector<string> suffixes = dfs(end, s, set);
                for (const string& suffix : suffixes) {
                    result.push_back(prefix + (suffix.empty() ? "" : " ") + suffix);
                }
            }
        }

        return result;
    }

public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> set(wordDict.begin(), wordDict.end());
        return dfs(0, s, set);
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
