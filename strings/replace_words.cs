/*
  In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".
  Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.

  Return the sentence after the replacement.

  Example 1:
    Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
    Output: "the cat was rat by the bat"
*/
public class Solution {
    public string ReplaceWords(IList<string> dictionary, string sentence) {
        Trie root = new();
        foreach (string word in dictionary) {
            root.Insert(word);
        }
        return root.Transform(sentence);
    }

    public sealed class Node() {
        public bool IsLeaf = false;
        public Node[] Links = new Node[26];

        public void Add(ref readonly char c) => Links[c - 'a'] = new();
        public Node Get(ref readonly char c) => Links[c - 'a'];
        public bool Contains(ref readonly char c) => Links[c - 'a'] != null;
    }

    public ref struct Trie() {
        public Node Root = new();

        public void Insert(string word)
        {
            Node current = Root;
            for (int i = 0; i < word.Length; ++i) {
                char c = word[i];
                if (!current.Contains(ref c)) current.Add(ref c);
                current = current.Get(ref c);
            }
            current.IsLeaf = true;
        }

        public string Transform(string sentence) {
            string[] words = sentence.Split(' ');
            for (int i = 0; i < words.Length; ++i) {
                Node current = Root;
                string lookup = string.Empty;
                for (int j = 0; j < words[i].Length; ++j) {
                    char c = words[i][j];
                    if (current.Contains(ref c)) {
                        lookup += words[i][j];
                        current = current.Get(ref c);
                        if (current.IsLeaf) {
                            words[i] = lookup;
                            break;
                        }
                    } else {
                        break;
                    }                  
                }
            }
            return string.Join(' ', words);
        }
    }
}
