/*
  A sentence is a string of single-space separated words where each word consists only of lowercase letters.
  A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
  
  Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
  
  Example 1:
    Input: s1 = "this apple is sweet", s2 = "this apple is sour"
    Output: ["sweet","sour"]
    Explanation:
    The word "sweet" appears only in s1, while the word "sour" appears only in s2.
  
  Example 2:
    Input: s1 = "apple apple", s2 = "banana"
    Output: ["banana"]
*/
class Solution {
public:
    vector<string> uncommonFromSentences(string s1, string s2) {
        unordered_map<string, int> map;

        istringstream iss1(s1);
        istringstream iss2(s2);
        string w;

        while (iss1 >> w) ++map[w];
        while (iss2 >> w) ++map[w];

        vector<string> result;
        for (auto& [k, v] : map) {
            if (v != 1) continue;
            result.push_back(k);
        }

        return result;
    }
};
