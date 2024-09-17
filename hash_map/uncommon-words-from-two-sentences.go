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
func uncommonFromSentences(s1 string, s2 string) []string {
    c1, c2 := make(map[string]int), make(map[string]int);

    for _, w := range strings.Split(s1, " ") {
        if _, exists := c1[w]; exists {
            c1[w]++;
        } else {
            c1[w] = 1;
        }
    }

    for _, w := range strings.Split(s2, " ") {
        if _, exists := c2[w]; exists {
            c2[w]++;
        } else {
            c2[w] = 1;
        }
    }

    result := make([]string, 0);

    for k, v := range c1 {
        if v != 1 {
            continue;
        }
        if _, exists := c2[k]; exists {
            continue;
        }
        result = append(result, k);
    }

    for k, v := range c2 {
        if v != 1 {
            continue;
        }
        if _, exists := c1[k]; exists {
            continue;
        }
        result = append(result, k);
    }

    return result;
}
