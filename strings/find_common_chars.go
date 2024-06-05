/*
  Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.

  Example:
    Input: words = ["bella","label","roller"]
    Output: ["e","l","l"]
*/
func commonChars(words []string) []string {
    n := len(words);
    result := strings.Split(words[0], "");

    if n == 1 {
        return result;
    }

    for i := 1; i < n; i++ {
        w := strings.Split(words[i], "");
        temp := []string{};
        for _, c := range result {
            for j := 0; j < len(w); j++ {
                if w[j] == c {
                    temp = append(temp, c);
                    w = append(w[:j], w[j + 1:]...);
                    break;
                }
            }
        }
        result = temp;
    }

    return result;
}
