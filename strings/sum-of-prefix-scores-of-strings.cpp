/*
  You are given an array words of size n consisting of non-empty strings.
  
  We define the score of a string word as the number of strings words[i] such that word is a prefix of words[i].
  
  For example, if words = ["a", "ab", "abc", "cab"], then the score of "ab" is 2, since "ab" is a prefix of both "ab" and "abc".
  Return an array answer of size n where answer[i] is the sum of scores of every non-empty prefix of words[i].
  
  Note that a string is considered as a prefix of itself.
  
  Example 1:
    Input: words = ["abc","ab","bc","b"]
    Output: [5,4,3,2]
    Explanation: The answer for each string is the following:
    - "abc" has 3 prefixes: "a", "ab", and "abc".
    - There are 2 strings with the prefix "a", 2 strings with the prefix "ab", and 1 string with the prefix "abc".
    The total is answer[0] = 2 + 2 + 1 = 5.
    - "ab" has 2 prefixes: "a" and "ab".
    - There are 2 strings with the prefix "a", and 2 strings with the prefix "ab".
    The total is answer[1] = 2 + 2 = 4.
    - "bc" has 2 prefixes: "b" and "bc".
    - There are 2 strings with the prefix "b", and 1 string with the prefix "bc".
    The total is answer[2] = 2 + 1 = 3.
    - "b" has 1 prefix: "b".
    - There are 2 strings with the prefix "b".
    The total is answer[3] = 2.
*/
class TrieNode {
public:
    std::unordered_map<char, TrieNode*> children;
    int prefixCount = 0;
    
    ~TrieNode() {
        for (auto child : children) {
            delete child.second;
        }
    }
};

class Trie {
private:
    TrieNode root;
    
public:
    void insert(const std::string& word) {
        TrieNode* node = &root;
        for (char ch : word) {
            if (node->children.find(ch) == node->children.end()) {
                node->children[ch] = new TrieNode();
            }
            node = node->children[ch];
            node->prefixCount++;
        }
    }

    int getPrefixScores(const std::string& word) {
        TrieNode* node = &root;
        int score = 0;
        for (char ch : word) {
            if (node->children.find(ch) != node->children.end()) {
                node = node->children[ch];
                score += node->prefixCount;
            } else {
                break;
            }
        }
        return score;
    }
};

class Solution {
public:
    vector<int> sumPrefixScores(vector<string>& words) {
        Trie trie;
        for (const auto& word : words) {
            trie.insert(word);
        }

        std::vector<int> scores;
        for (const auto& word : words) {
            scores.push_back(trie.getPrefixScores(word));
        }
        return scores;
    }
};
