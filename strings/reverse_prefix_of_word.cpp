class Solution {
public:
    string reversePrefix(string word, char ch) {
        if (word.size() == 1) return word;

        int idx = word.find_first_of(ch);
        if (idx == -1) return word;

        string result = {};
        for (int i = idx; i >=0; --i) result += word[i];
        for (int i = idx + 1; i < word.size(); ++i) result +=word[i];

        return result;
    }
};
