/*
  Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.

  Example:
    Input: words = ["bella","label","roller"]
    Output: ["e","l","l"]
*/
class Solution {
public:
    vector<string> commonChars(vector<string>& words) {
        int n = words.size();
        vector<string> result;
        if (n == 1) {
            for(char c : words[0]) {
                result.push_back(string(1, c));
            }
            return result;
        }

        vector<char> buffer(words[0].begin(), words[0].end());

        for (int i = 1; i < n; ++i) {
            vector<char> w(words[i].begin(), words[i].end());
            vector<char> temp;
            for (char c : buffer) {
                auto it = find(w.begin(), w.end(), c);
                if (it != w.end()) {
                    temp.push_back(c);
                    w.erase(it);
                }
            }
            buffer = temp;
        }

        for(char c : buffer) {
            result.push_back(string(1, c));
        }

        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
