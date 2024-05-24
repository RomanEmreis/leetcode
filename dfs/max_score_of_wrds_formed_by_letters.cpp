/*
  Given a list of words, list of  single letters (might be repeating) and score of every character.
  Return the maximum score of any valid set of words formed by using the given letters (words[i] cannot be used two or more times).

  It is not necessary to use all characters in letters and each letter can only be used once. Score of letters 'a', 'b', 'c', ... ,'z' is given by score[0], score[1], ... , score[25] respectively.

  Example:
    Input: words = ["dog","cat","dad","good"], letters = ["a","a","c","d","d","d","g","o","o"], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
    Output: 23
    Explanation:
    Score  a=1, c=9, d=5, g=3, o=2
    Given letters, we can form the words "dad" (5+1+5) and "good" (3+2+2+5) with a score of 23.
    Words "dad" and "dog" only get a score of 21.
*/
class Solution {
private:
    vector<int> a;
    vector<int> b;
    int result = 0;

    void solve(int idx, vector<string>& words, vector<int>& score) {
        if (idx == words.size()) {
            int sc = 0;
            for (int i = 0; i < 26; i++) {
                if (b[i] > a[i]) return;
                sc += score[i] * b[i];
            }
            result = max(result, sc);
            return;
        }
        solve(idx + 1, words, score);
        for (auto i : words[idx]) b[i - 'a']++;
        solve(idx + 1, words, score);
        for (auto i : words[idx]) b[i - 'a']--;
    }

public:
    Solution(): a(26), b(26) {}

    int maxScoreWords(vector<string>& words, vector<char>& letters, vector<int>& score) {
        for (auto i : letters) a[i - 'a']++;
        solve(0, words, score);
        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
