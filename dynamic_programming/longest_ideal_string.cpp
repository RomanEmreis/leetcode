class Solution {
public:
    int longestIdealString(string s, int k) {
        const char a = 'a';
        const int max_chars = 25;

        vector<int> dp(max_chars + 1, 0);

        for (const char& c : s) {
            int tmp = 0;
            int x = c - a;
            for (int i = max(0, x - k); i <= min(max_chars, x + k); ++i) {
                tmp = max(tmp, dp[i]);
            }

            dp[x] = ++tmp;
        }

        return *max_element(begin(dp), end(dp));
    }
};

static auto init = []() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);
  cout.tie(nullptr);
  return 'c';
}();
