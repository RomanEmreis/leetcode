/*
  In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".
  Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.

  Return the sentence after the replacement.

  Example 1:
    Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
    Output: "the cat was rat by the bat"
*/
class Solution {
public:
    string replaceWords(vector<string>& dictionary, string sentence) {
        Trie* root = new Trie();
        for (const string& word : dictionary) {
            root->insert(word);
        }
        return root->transform(sentence);
    }

    class Node {
    public:
        bool isLeaf = false;
        Node* links[26] = {nullptr}; 

        void add(const char& c) {
            links[c - 'a'] = new Node();
        }   

        Node* get(const char& c) {
            return links[c - 'a'];
        }   

        bool contains(const char& c) {
            return links[c - 'a'] != nullptr;
        }
    };

    class Trie {
    private:
        void deleteTrie(Node* node) {
            for(int i = 0; i < 26; i++) {
                if(node->links[i]) {
                    deleteTrie(node->links[i]);
                }
            }
            delete node;
        }

    public:
        Node* root;

        Trie() {
            root = new Node();
        }

        ~Trie() {
            deleteTrie(root);
        }
    
        void insert(const string& word) {
            Node* curr = root;
            for (int i = 0; i < word.size(); ++i) {
                if (!curr->contains(word[i])) curr->add(word[i]);
                curr = curr->get(word[i]);
            }
            curr->isLeaf = true;
        }
    
        string transform(string& sentence) {
            vector<string> words = split(sentence);
            for (int i = 0; i < words.size(); ++i) {
                Node* curr = root;
                string lookup;
                int n = words[i].size();
                for (int j = 0; j < n; ++j) {
                    if (curr->contains(words[i][j])) {
                        lookup +=words[i][j];
                        curr = curr->get(words[i][j]);
                        if (curr->isLeaf) {
                            words[i] = lookup;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
    
            string result;
            for (const auto& word : words) {
                result += word + ' ';
            }
            result.pop_back();

            return result;
        }
    
        vector<string> split(const string& sentence) {
            stringstream ss(sentence); 
            string token; 
            vector<string> tokens; 
            char delimiter = ' '; 
    
            while (getline(ss, token, delimiter)) { 
                tokens.push_back(token); 
            }
    
            return tokens; 
        }
    };
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
