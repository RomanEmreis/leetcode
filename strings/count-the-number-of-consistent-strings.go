/*
  You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.
  
  Return the number of consistent strings in the array words.
  
  Example 1:
    Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
    Output: 2
    Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
*/
func countConsistentStrings(allowed string, words []string) int {
    set := make(map[rune]bool);
    for _, ch := range allowed {
        set[ch] = true;
    }

    result := 0;

    for _, word := range words {
        isConsistent := true;

        for _, ch := range word {
            if !set[ch] {
                isConsistent = false;
                break;
            }
        }

        if isConsistent {
            result++;
        }
    }

    return result;
}
